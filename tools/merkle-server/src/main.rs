//! Merkle Proof Server
//!
//! Minimal HTTP server that stores and serves BLAKE3 merkle proofs for
//! the cw-whitelist-merkletree contract.
//!
//! Read access: public (no auth required)
//! Write access: requires a secp256k1 ECDSA signature from the configured admin key
//!
//! ## Two-step write workflow (private key never passes through this binary)
//!
//! ### Upload
//!   # 1. Get the canonical message + timestamp to sign:
//!   merkle-server prepare upload <tree_id> <tree.json>
//!
//!   # 2. Sign the printed canonical message with your key (any ECDSA-SHA256 tool):
//!   echo -n "<canonical>" | openssl dgst -sha256 -sign privkey.pem | xxd -p -c 0
//!
//!   # 3. Submit:
//!   merkle-server upload <tree_id> <tree.json> --timestamp <ts> --signature <sig_hex>
//!
//! ### Delete
//!   merkle-server prepare delete <tree_id>
//!   # sign it ...
//!   merkle-server delete <tree_id> --timestamp <ts> --signature <sig_hex>
//!
//! ## Other commands
//!   merkle-server serve    [--config config.json]   Run the HTTP server
//!   merkle-server keygen                            Generate a keypair
//!   merkle-server build    <addrs.txt> [-o tree.json]  Build tree from address list
//!
//! ## REST API
//!   GET  /health
//!   GET  /trees
//!   GET  /tree/<id>
//!   GET  /tree/<id>/root
//!   GET  /tree/<id>/members
//!   GET  /tree/<id>/proof/<address>
//!   POST   /tree/<id>     (X-Timestamp + X-Signature required)
//!   DELETE /tree/<id>     (X-Timestamp + X-Signature required)

mod auth;
mod store;
mod tree;

use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::{HeaderMap, Method, StatusCode},
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use clap::{Parser, Subcommand};
use serde::Deserialize;
use serde_json::{json, Value};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tower_http::cors::{Any, CorsLayer};

use auth::AuthKey;
use store::Store;

// ── CLI ───────────────────────────────────────────────────────────────────────

#[derive(Parser)]
#[command(name = "merkle-server", about = "BLAKE3 merkle proof server for whitelists")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Run the HTTP server (default when no subcommand given)
    Serve {
        #[arg(long, env = "MERKLE_CONFIG", default_value = "config.json")]
        config: PathBuf,
    },

    /// Generate a secp256k1 keypair.
    /// Store the private key securely — it is only used externally to sign requests.
    Keygen,

    /// Build a merkle tree from a newline-delimited address file
    Build {
        /// Path to file with one address per line (# comments and blank lines ignored)
        addrs: PathBuf,
        /// Output JSON file
        #[arg(short, long, default_value = "tree.json")]
        out: PathBuf,
    },

    /// Print the canonical message + timestamp that must be signed before uploading/deleting.
    /// Sign the output with your private key externally — then pass --timestamp and --signature
    /// to the `upload` or `delete` commands.
    Prepare {
        #[command(subcommand)]
        action: PrepareAction,
    },

    /// Upload a tree JSON to the server.
    /// Obtain --timestamp and --signature from `merkle-server prepare upload` first.
    Upload {
        /// Tree identifier (e.g. collection address or "season-1")
        tree_id: String,
        /// Path to tree.json produced by `build`
        tree_file: PathBuf,
        /// Timestamp from `prepare upload` output (must match the one you signed)
        #[arg(long)]
        timestamp: String,
        /// Compact 64-byte ECDSA-SHA256 signature (hex) over the canonical message
        #[arg(long)]
        signature: String,
        /// Server URL
        #[arg(long, default_value = "http://127.0.0.1:8765")]
        server: String,
    },

    /// Delete a tree from the server.
    /// Obtain --timestamp and --signature from `merkle-server prepare delete` first.
    Delete {
        tree_id: String,
        #[arg(long)]
        timestamp: String,
        #[arg(long)]
        signature: String,
        #[arg(long, default_value = "http://127.0.0.1:8765")]
        server: String,
    },
}

#[derive(Subcommand)]
enum PrepareAction {
    /// Prepare a canonical message for an upload (POST /tree/<id>)
    Upload {
        tree_id: String,
        tree_file: PathBuf,
    },
    /// Prepare a canonical message for a delete (DELETE /tree/<id>)
    Delete {
        tree_id: String,
    },
}

// ── Config ────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct Config {
    /// Compressed secp256k1 public key, 66 hex chars (33 bytes).
    public_key: String,
    #[serde(default = "defaults::data_dir")]
    data_dir: PathBuf,
    #[serde(default = "defaults::host")]
    host: String,
    #[serde(default = "defaults::port")]
    port: u16,
    #[serde(default = "defaults::tolerance")]
    timestamp_tolerance_s: u64,
}

mod defaults {
    use std::path::PathBuf;
    pub fn data_dir() -> PathBuf { PathBuf::from("./data") }
    pub fn host() -> String { "127.0.0.1".into() }
    pub fn port() -> u16 { 8765 }
    pub fn tolerance() -> u64 { 300 }
}

// ── App state ─────────────────────────────────────────────────────────────────

struct AppState {
    store: Store,
    auth: AuthKey,
    tolerance_s: u64,
}

// ── Error type ────────────────────────────────────────────────────────────────

struct AppError(StatusCode, String);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (self.0, Json(json!({"error": self.1}))).into_response()
    }
}

type Res = Result<Json<Value>, AppError>;

macro_rules! err {
    ($code:expr, $msg:expr $(,)?) => {
        return Err(AppError($code, $msg.into()))
    };
}

// ── Handlers ──────────────────────────────────────────────────────────────────

async fn health() -> Json<Value> {
    Json(json!({"status": "ok"}))
}

async fn list_trees(State(s): State<Arc<AppState>>) -> Json<Value> {
    Json(json!({"trees": s.store.list()}))
}

async fn get_tree(State(s): State<Arc<AppState>>, Path(id): Path<String>) -> Res {
    match s.store.load(&id) {
        Some(t) => Ok(Json(serde_json::to_value(t).unwrap())),
        None => err!(StatusCode::NOT_FOUND, "tree not found"),
    }
}

async fn get_root(State(s): State<Arc<AppState>>, Path(id): Path<String>) -> Res {
    match s.store.load(&id) {
        Some(t) => Ok(Json(json!({"root": t.root}))),
        None => err!(StatusCode::NOT_FOUND, "tree not found"),
    }
}

async fn get_members(State(s): State<Arc<AppState>>, Path(id): Path<String>) -> Res {
    match s.store.load(&id) {
        Some(t) => {
            let members: Vec<&String> = t.members.keys().collect();
            Ok(Json(json!({"members": members})))
        }
        None => err!(StatusCode::NOT_FOUND, "tree not found"),
    }
}

async fn get_proof(
    State(s): State<Arc<AppState>>,
    Path((id, address)): Path<(String, String)>,
) -> Res {
    match s.store.load(&id) {
        None => err!(StatusCode::NOT_FOUND, "tree not found"),
        Some(t) => match t.members.get(&address) {
            None => err!(StatusCode::NOT_FOUND, "address not in whitelist"),
            Some(member) => Ok(Json(json!({
                "root": t.root,
                "proof_hashes": member.proof_hashes,
                "allocation": member.allocation,
                "tier": member.tier,
            }))),
        },
    }
}

async fn upload_tree(
    State(s): State<Arc<AppState>>,
    Path(id): Path<String>,
    headers: HeaderMap,
    body: String,
) -> Res {
    let timestamp = header_str(&headers, "x-timestamp");
    let sig_hex   = header_str(&headers, "x-signature");

    if !s.auth.verify("POST", &format!("/tree/{id}"), timestamp, body.as_bytes(), sig_hex, s.tolerance_s) {
        err!(StatusCode::UNAUTHORIZED, "invalid or expired signature");
    }

    let input: store::TreeInput = serde_json::from_str(&body)
        .map_err(|e| AppError(StatusCode::BAD_REQUEST, format!("invalid JSON: {e}")))?;

    input.validate()
        .map_err(|e| AppError(StatusCode::BAD_REQUEST, e))?;

    let member_count = input.accounts.len();
    let root = input.merkle_root.clone();
    s.store.save(&id, input)
        .map_err(|e| AppError(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    eprintln!("[tree] saved {id}: {member_count} members, root={}", &root[..16]);
    Ok(Json(json!({"ok": true, "tree_id": id, "root": root, "members": member_count})))
}

async fn delete_tree(
    State(s): State<Arc<AppState>>,
    Path(id): Path<String>,
    headers: HeaderMap,
) -> Res {
    let timestamp = header_str(&headers, "x-timestamp");
    let sig_hex   = header_str(&headers, "x-signature");

    if !s.auth.verify("DELETE", &format!("/tree/{id}"), timestamp, b"", sig_hex, s.tolerance_s) {
        err!(StatusCode::UNAUTHORIZED, "invalid or expired signature");
    }
    if !s.store.exists(&id) {
        err!(StatusCode::NOT_FOUND, "tree not found");
    }
    s.store.delete(&id)
        .map_err(|e| AppError(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(json!({"ok": true, "deleted": id})))
}

fn header_str<'a>(headers: &'a HeaderMap, name: &str) -> &'a str {
    headers.get(name).and_then(|v| v.to_str().ok()).unwrap_or("")
}

// ── Server setup ──────────────────────────────────────────────────────────────

async fn serve(config_path: PathBuf) -> Result<()> {
    let cfg: Config = {
        let raw = std::fs::read_to_string(&config_path)
            .map_err(|_| anyhow::anyhow!(
                "Config not found: {}\nCopy config.example.json → config.json and set your public_key.\nGenerate a keypair: merkle-server keygen",
                config_path.display()
            ))?;
        serde_json::from_str(&raw)?
    };

    let auth = AuthKey::from_hex(&cfg.public_key)
        .map_err(|e| anyhow::anyhow!("Invalid public_key in config: {e}"))?;
    let store = Store::open(&cfg.data_dir)?;

    let state = Arc::new(AppState {
        store,
        auth,
        tolerance_s: cfg.timestamp_tolerance_s,
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::OPTIONS])
        .allow_headers(Any);

    let app = Router::new()
        .route("/health",                    get(health))
        .route("/trees",                     get(list_trees))
        .route("/tree/:id",                  get(get_tree).post(upload_tree).delete(delete_tree))
        .route("/tree/:id/root",             get(get_root))
        .route("/tree/:id/members",          get(get_members))
        .route("/tree/:id/proof/:address",   get(get_proof))
        .layer(cors)
        .with_state(state);

    let addr: SocketAddr = format!("{}:{}", cfg.host, cfg.port).parse()?;
    let key_preview = format!("{}…{}", &cfg.public_key[..8], &cfg.public_key[cfg.public_key.len()-8..]);

    eprintln!("Merkle Proof Server");
    eprintln!("  Listening : http://{addr}");
    eprintln!("  Data dir  : {}", cfg.data_dir.display());
    eprintln!("  Admin key : {key_preview}");
    eprintln!("  Replay TTL: {}s", cfg.timestamp_tolerance_s);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

// ── Subcommand helpers ────────────────────────────────────────────────────────

fn cmd_keygen() -> Result<()> {
    let (sk_hex, pk_hex) = AuthKey::generate()?;
    println!("=== Merkle Server Keypair ===\n");
    println!("PRIVATE KEY (keep this secret — only needed to sign requests externally):");
    println!("  {sk_hex}\n");
    println!("PUBLIC KEY (put in config.json → \"public_key\"):");
    println!("  {pk_hex}\n");
    println!("The private key never needs to be passed to this binary.");
    println!("Use `merkle-server prepare` to get the message to sign,");
    println!("sign it externally, then pass --signature to upload/delete.");
    Ok(())
}

fn cmd_build(addrs_path: PathBuf, out_path: PathBuf) -> Result<()> {
    let raw = std::fs::read_to_string(&addrs_path)?;
    // One address per line; all get allocation=1 (use gen_merkle for tiered allocations)
    let entries: Vec<(String, u32)> = raw
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(|l| (l.to_owned(), 1u32))
        .collect();

    eprintln!("Building merkle tree for {} addresses (allocation=1)…", entries.len());
    let (root, members) = tree::build(&entries)?;
    eprintln!("Root: {root}");

    let mut failures = 0usize;
    for (addr, (alloc, proof)) in &members {
        if !tree::verify(addr, *alloc, proof, &root)? {
            eprintln!("  FAIL: {addr}");
            failures += 1;
        }
    }
    if failures > 0 {
        anyhow::bail!("{failures} proof(s) failed self-verification");
    }
    eprintln!("All {} proofs verified ✓", entries.len());

    // Output in gen_merkle-compatible format so it can be uploaded directly
    let accounts: serde_json::Map<String, serde_json::Value> = members
        .iter()
        .map(|(addr, (alloc, hashes))| {
            (addr.clone(), serde_json::json!({
                "tier": 1u8,
                "allocation": alloc,
                "proof_hashes": hashes,
            }))
        })
        .collect();

    let output = serde_json::json!({
        "merkle_root": root,
        "total_addresses": entries.len(),
        "accounts": accounts,
    });
    std::fs::write(&out_path, serde_json::to_string(&output)?)?;
    eprintln!("Written: {}", out_path.display());
    Ok(())
}

fn cmd_prepare(action: PrepareAction) -> Result<()> {
    let (method, path, body) = match &action {
        PrepareAction::Upload { tree_id, tree_file } => {
            let body = std::fs::read(tree_file)?;
            ("POST".to_string(), format!("/tree/{tree_id}"), body)
        }
        PrepareAction::Delete { tree_id } => {
            ("DELETE".to_string(), format!("/tree/{tree_id}"), vec![])
        }
    };

    let timestamp = auth::now_timestamp();
    let canonical = auth::canonical_message(&method, &path, &timestamp, &body);
    let canonical_hex = hex::encode(canonical.as_bytes());

    println!("=== Message to Sign ===\n");
    println!("Timestamp : {timestamp}");
    println!("Method    : {method}");
    println!("Path      : {path}");
    println!();
    println!("Canonical message (sign this with ECDSA-SHA256 / secp256k1):");
    println!("{canonical}");
    println!();
    println!("Canonical message as hex:");
    println!("{canonical_hex}");
    println!();
    println!("=== After signing, run ===\n");

    match action {
        PrepareAction::Upload { tree_id, tree_file } => {
            println!(
                "merkle-server upload {tree_id} {} \\\n  --timestamp {timestamp} \\\n  --signature <your_sig_hex>",
                tree_file.display()
            );
            println!();
            println!("Or with curl:");
            println!("  curl -X POST http://127.0.0.1:8765/tree/{tree_id} \\");
            println!("    -H 'Content-Type: application/json' \\");
            println!("    -H 'X-Timestamp: {timestamp}' \\");
            println!("    -H 'X-Signature: <your_sig_hex>' \\");
            println!("    --data-binary @{}", tree_file.display());
        }
        PrepareAction::Delete { tree_id } => {
            println!(
                "merkle-server delete {tree_id} \\\n  --timestamp {timestamp} \\\n  --signature <your_sig_hex>"
            );
            println!();
            println!("Or with curl:");
            println!("  curl -X DELETE http://127.0.0.1:8765/tree/{tree_id} \\");
            println!("    -H 'X-Timestamp: {timestamp}' \\");
            println!("    -H 'X-Signature: <your_sig_hex>'");
        }
    }
    Ok(())
}

fn cmd_upload(
    tree_id: String,
    tree_file: PathBuf,
    timestamp: String,
    signature: String,
    server: String,
) -> Result<()> {
    let body = std::fs::read(&tree_file)?;
    let data: serde_json::Value = serde_json::from_slice(&body)?;
    let member_count = data["accounts"].as_object().map(|m| m.len()).unwrap_or(0);

    let url = format!("{}/tree/{tree_id}", server.trim_end_matches('/'));
    eprintln!("Uploading tree '{tree_id}' ({member_count} members) → {url}");

    send_request("POST", &url, &timestamp, &signature, Some(&body), &tree_file)
}

fn cmd_delete(
    tree_id: String,
    timestamp: String,
    signature: String,
    server: String,
) -> Result<()> {
    let url = format!("{}/tree/{tree_id}", server.trim_end_matches('/'));
    eprintln!("Deleting tree '{tree_id}' from {url}");
    send_request("DELETE", &url, &timestamp, &signature, None, std::path::Path::new(""))
}

/// Send a signed request via curl (or print the curl command if curl is not available).
fn send_request(
    method: &str,
    url: &str,
    timestamp: &str,
    signature: &str,
    body: Option<&[u8]>,
    body_file: &std::path::Path,
) -> Result<()> {
    let mut args = vec![
        "-s".to_string(), "-f".to_string(),
        "-X".to_string(), method.to_string(),
        "-H".to_string(), format!("X-Timestamp: {timestamp}"),
        "-H".to_string(), format!("X-Signature: {signature}"),
    ];

    if body.is_some() {
        args.extend([
            "-H".to_string(), "Content-Type: application/json".to_string(),
            "--data-binary".to_string(), "@-".to_string(),
        ]);
    }
    args.push(url.to_string());

    let mut cmd = std::process::Command::new("curl");
    cmd.args(&args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());

    if body.is_some() {
        cmd.stdin(std::process::Stdio::piped());
    }

    match cmd.spawn() {
        Ok(mut child) => {
            if let (Some(stdin), Some(data)) = (child.stdin.as_mut(), body) {
                use std::io::Write;
                stdin.write_all(data)?;
            }
            let out = child.wait_with_output()?;
            if out.status.success() {
                eprintln!("OK: {}", String::from_utf8_lossy(&out.stdout));
            } else {
                anyhow::bail!(
                    "Request failed: {}{}",
                    String::from_utf8_lossy(&out.stdout),
                    String::from_utf8_lossy(&out.stderr)
                );
            }
        }
        Err(_) => {
            // curl not available — print the equivalent command
            eprintln!("(curl not found — run this manually)\n");
            let mut line = format!("curl -X {method}");
            line += &format!(" \\\n    -H 'X-Timestamp: {timestamp}'");
            line += &format!(" \\\n    -H 'X-Signature: {signature}'");
            if body.is_some() {
                line += " \\\n    -H 'Content-Type: application/json'";
                line += &format!(" \\\n    --data-binary @{}", body_file.display());
            }
            line += &format!(" \\\n    {url}");
            eprintln!("{line}");
        }
    }
    Ok(())
}

// ── main ──────────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command.unwrap_or(Command::Serve { config: "config.json".into() }) {
        Command::Serve { config } => serve(config).await,
        Command::Keygen => cmd_keygen(),
        Command::Build { addrs, out } => cmd_build(addrs, out),
        Command::Prepare { action } => cmd_prepare(action),
        Command::Upload { tree_id, tree_file, timestamp, signature, server } => {
            cmd_upload(tree_id, tree_file, timestamp, signature, server)
        }
        Command::Delete { tree_id, timestamp, signature, server } => {
            cmd_delete(tree_id, timestamp, signature, server)
        }
    }
}

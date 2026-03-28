//! headstash-server — Privacy note + circuit key server for the MetaMask snap.

mod middleware;
mod pir;
mod store;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    middleware as axum_mw,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use clap::Parser;
use std::sync::Arc;
use store::Store;

#[derive(Parser)]
#[command(name = "headstash-server", about = "Privacy note + key server")]
struct Cli {
    #[arg(short, long, default_value = "config.toml")]
    config: String,
}

#[derive(serde::Deserialize)]
struct Config {
    bind: String,
    data_dir: String,
}

struct AppState {
    store: Store,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();
    let config_str = std::fs::read_to_string(&cli.config)?;
    let config: Config = toml::from_str(&config_str)?;

    let store = Store::new(std::path::Path::new(&config.data_dir))?;
    let state = Arc::new(AppState { store });

    // Public routes (no auth)
    let public = Router::new()
        .route("/health", get(health))
        .route("/headstash/{id}", get(get_headstash))
        .route("/keys", get(list_keys_public));

    // Authenticated routes (secp256k1 / JWT)
    let authed = Router::new()
        .route("/headstash/{id}", post(register_headstash))
        .route("/headstash/{id}/root", get(get_headstash_root))
        .route("/headstash/{id}/sync", post(sync_headstash))
        .route("/notes/{hs_id}/{addr}", get(get_note))
        .route("/notes/{hs_id}", get(list_notes))
        .route("/notes/{hs_id}/pir", post(pir_fetch_note))
        .route("/keys/{key_id}", get(download_key))
        .route("/keys/{key_id}/pir", post(pir_fetch_key))
        .layer(axum_mw::from_fn(middleware::auth_middleware));

    let app = Router::new()
        .merge(public)
        .merge(authed)
        .layer(
            tower_http::cors::CorsLayer::permissive(),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&config.bind).await?;
    tracing::info!(bind = %config.bind, "headstash-server listening");
    axum::serve(listener, app).await?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

async fn health() -> impl IntoResponse {
    (StatusCode::OK, "ok")
}

// ── Headstash ────────────────────────────────────────────────────────

async fn get_headstash(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    state
        .store
        .get_headstash(&id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn register_headstash(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(body): Json<serde_json::Value>,
) -> Result<StatusCode, StatusCode> {
    state
        .store
        .set_headstash(&id, &body)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::CREATED)
}

async fn get_headstash_root(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let root = state
        .store
        .get_headstash_root(&id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(serde_json::json!({ "root": root })))
}

async fn sync_headstash(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(body): Json<serde_json::Value>,
) -> Result<StatusCode, StatusCode> {
    // Merge sync data into existing record
    let mut record = state
        .store
        .get_headstash(&id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .unwrap_or(serde_json::json!({}));

    if let (Some(existing), Some(incoming)) = (record.as_object_mut(), body.as_object()) {
        for (k, v) in incoming {
            existing.insert(k.clone(), v.clone());
        }
    }

    state
        .store
        .set_headstash(&id, &record)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::OK)
}

// ── Notes ────────────────────────────────────────────────────────────

async fn get_note(
    State(state): State<Arc<AppState>>,
    Path((hs_id, addr)): Path<(String, String)>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    state
        .store
        .get_note(&hs_id, &addr)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn list_notes(
    State(state): State<Arc<AppState>>,
    Path(hs_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let keys = state
        .store
        .list_note_keys(&hs_id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(serde_json::json!({ "keys": keys })))
}

#[derive(serde::Deserialize)]
struct PirRequest {
    selector: Vec<u8>,
}

async fn pir_fetch_note(
    State(state): State<Arc<AppState>>,
    Path(hs_id): Path<String>,
    Json(req): Json<PirRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let (_, blobs) = state
        .store
        .get_note_blobs(&hs_id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let blob_refs: Vec<&[u8]> = blobs.iter().map(|b| b.as_slice()).collect();
    let result = pir::xor_pir(&blob_refs, &req.selector)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok(Json(serde_json::json!({ "result": hex::encode(result) })))
}

// ── Keys ─────────────────────────────────────────────────────────────

async fn download_key(
    State(state): State<Arc<AppState>>,
    Path(key_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let data = state
        .store
        .get_key(&key_id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(serde_json::json!({ "data": hex::encode(data) })))
}

async fn pir_fetch_key(
    State(state): State<Arc<AppState>>,
    Path(key_id): Path<String>,
    Json(req): Json<PirRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let _ = key_id; // key_id in path for API consistency; PIR uses selector
    let (_, blobs) = state
        .store
        .get_key_blobs()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let blob_refs: Vec<&[u8]> = blobs.iter().map(|b| b.as_slice()).collect();
    let result = pir::xor_pir(&blob_refs, &req.selector)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok(Json(serde_json::json!({ "result": hex::encode(result) })))
}

async fn list_keys_public(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let keys = state
        .store
        .list_keys()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(serde_json::json!({ "keys": keys })))
}

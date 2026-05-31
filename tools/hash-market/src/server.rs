//! Unified hash-market server.
//!
//! Composes:
//! - Vote Extension server (VE routes for ABCI ExtendVote/VerifyVote)
//! - Blossom server (BUD protocol for blob/merkle tree serving)
//!
//! The unified `AppState` holds both VE handler and blob storage, allowing
//! both protocol handlers to share the same runtime.
//!
//! # Architecture
//!
//! Blossom blob storage delegates to `TreeStore` (file-based with `/f/` prefix)
//! via the `BlobStore` trait — no independent in-memory store exists. Content
//! lookups are O(1) through the QMD-style content-ID index layered on top.
//! This matches the BUD-02/BUD-06 spec while keeping a single canonical store.

use axum::body::Body;
use axum::http::Response;
use axum::{
    extract::{Path, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use cw721_nips::buds::{self, BlossomState};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{ProviderKey, ProviderStatus};

#[cfg(feature = "ve")]
use crate::client::ve::VoteExtensionHandler;
use crate::store::{HeadstashStore, TreeInput, TreeStore};
#[cfg(feature = "transport")]
use crate::transport::TransportMode;
#[cfg(feature = "ve")]
use crate::ve::server as ve_server;

// ── Unified AppState ─────────────────────────────────────────────────────────

/// Unified server state — holds both VE handler and blob/tree storage.
///
/// All blob storage (BUD, blossom) delegates to `tree_store` through
/// `blossom.store`, which is `Arc<dyn BlobStore>` backed by `TreeStore`.
/// No independent in-memory blob store exists.
pub struct AppState {
    /// Vote extension signing handler (from custody)
    pub ve_handler: VoteExtensionHandler,
    /// CometBFT chain ID
    pub chain_id: String,
    /// Provider data for VE (chain_uid → data)
    pub provider_data: RwLock<HashMap<ProviderKey, ve_server::ProviderData>>,
    /// Provider metadata for status
    pub provider_status: RwLock<Vec<ProviderStatus>>,
    /// Current block height
    pub current_height: RwLock<u64>,
    /// Blossom blob storage — delegates to TreeStore via BlobStore trait
    pub blossom: Arc<BlossomState>,
    /// Merkle tree storage (headstash trees)
    pub tree_store: Arc<TreeStore>,
    /// Headstash metadata storage
    pub headstash_store: Arc<HeadstashStore>,
}

impl AppState {
    /// Create new unified state.
    ///
    /// `BlossomState` is initialized with `TreeStore` as its `BlobStore`
    /// backend — no separate in-memory store is created.
    pub fn new(
        ve_handler: VoteExtensionHandler,
        chain_id: String,
        data_dir: PathBuf,
    ) -> anyhow::Result<Self> {
        let tree_store = Arc::new(TreeStore::open(data_dir.join("trees"))?);
        let headstash_store = Arc::new(HeadstashStore::new(&data_dir)?);
        // BlossomState wraps TreeStore via BlobStore trait — single canonical store
        let blossom = Arc::new(BlossomState::new(tree_store.clone()));

        Ok(Self {
            ve_handler,
            chain_id,
            provider_data: RwLock::new(HashMap::new()),
            provider_status: RwLock::new(Vec::new()),
            current_height: RwLock::new(0),
            blossom,
            tree_store,
            headstash_store,
        })
    }

    /// Add a provider to track (persisted to shared state).
    pub async fn add_provider(&self, status: ProviderStatus) {
        let mut statuses = self.provider_status.write().await;
        statuses.push(status);
    }
}

// ── Unified Router ───────────────────────────────────────────────────────────

/// Build the unified router combining VE and Blossom routes.
///
/// Route layout:
/// ```text
/// /health                      — unified health (VE + blossom)
/// /blobs/{hash}  GET, DELETE   — BUD-02 blob retrieval/deletion
/// /blobs         GET, POST     — BUD-06 list/upload
/// /upload        POST          — alias for /blobs POST
/// /trees/{id}    GET, POST, DELETE — merkle tree CRUD
/// /trees         GET           — list tree IDs
/// /headstash/{id} GET, POST    — headstash registration
/// /ve/*                        — VE routes (nested)
/// ```
pub fn router(state: Arc<AppState>) -> Router {
    #[cfg(feature = "ve")]
    let ve_router = ve_server::router(state.clone());

    // Build the unified router
    Router::new()
        // Root-level health that reports both subsystems
        .route("/health", get(unified_health))
        // Blossom BUD protocol routes — delegate to TreeStore via BlossomState
        .route("/blobs/:hash", get(blob_get).delete(blob_delete))
        .route("/blobs", get(blob_list).post(blob_upload))
        .route("/upload", post(blob_upload))
        // Headstash merkle tree routes
        .route("/trees/:id", get(tree_get).post(tree_save).delete(tree_delete))
        .route("/trees", get(tree_list))
        // Headstash routes
        .route("/headstash/:id", get(headstash_get).post(headstash_save))
        .with_state(state.clone())
        .nest("/ve", ve_router) // VE routes: /ve/health, /ve/providers, etc.
}

// ── Health ───────────────────────────────────────────────────────────────────

async fn unified_health(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let ve_status = {
        let statuses = state.provider_status.read().await;
        let active = statuses.iter().filter(|s| s.running).count();
        serde_json::json!({
            "active_providers": active,
            "total_providers": statuses.len(),
        })
    };

    let blossom_status = serde_json::json!({
        "blobs": state.blossom.store.list().len(),
        "store": "TreeStore (/f/ prefix + QMD content-ID index)",
    });

    Json(serde_json::json!({
        "status": "ok",
        "ve": ve_status,
        "blossom": blossom_status,
    }))
}

// ── Blossom/BUD Handlers ─────────────────────────────────────────────────────
//
// These are thin wrappers that delegate to BlossomState (which backs to
// TreeStore via the BlobStore trait). Helper functions (parse_hash,
// check_scope, extract_payment_proof) are re-exported from cw721-nips::buds
// to avoid duplication with the bud06.rs canonical implementation.

/// GET /blobs/{hash} — BUD-01 style blob retrieval.
async fn blob_get(
    State(state): State<Arc<AppState>>,
    Path(hash_hex): Path<String>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, StatusCode> {
    // Verify auth via blossom handler
    let claims = state
        .blossom
        .auth
        .verify(&headers)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Parse hash (uses bud06::parse_hash — canonical impl)
    let hash = buds::parse_hash(&hash_hex)?;

    // Check scope
    buds::check_scope(&claims, &hash)?;

    // Check payment if configured
    if let Some(pv) = &state.blossom.payment {
        let proof = buds::extract_payment_proof(&headers)?;
        pv.verify_download(&proof, &hash)
            .map_err(|_| StatusCode::PAYMENT_REQUIRED)?;
    }

    // Fetch blob — delegated to TreeStore via BlossomState::store
    let blob = state
        .blossom
        .store
        .get(&hash)
        .ok_or(StatusCode::NOT_FOUND)?;

    let mut resp = Response::new(Body::from(blob));
    if let Ok(hv) = HeaderValue::from_str(&hash_hex) {
        resp.headers_mut().insert("X-Content-SHA256", hv);
    }
    // Include metadata from the content-ID index
    if let Some(entry) = state.tree_store.blob_meta(&hash) {
        if let Ok(hv) = HeaderValue::from_str(&entry.size.to_string()) {
            resp.headers_mut().insert("X-Content-Length", hv);
        }
        if let Ok(hv) = HeaderValue::from_str(&entry.uploaded.to_string()) {
            resp.headers_mut().insert("X-Uploaded", hv);
        }
    }
    resp.headers_mut().insert(
        "Content-Type",
        HeaderValue::from_static("application/octet-stream"),
    );
    Ok(resp)
}

/// DELETE /blobs/{hash} — BUD-02 style blob deletion.
async fn blob_delete(
    State(state): State<Arc<AppState>>,
    Path(hash_hex): Path<String>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, StatusCode> {
    let claims = state
        .blossom
        .auth
        .verify(&headers)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    buds::check_action(&claims, "delete")?;
    let hash = buds::parse_hash(&hash_hex)?;

    // Delegates to TreeStore via BlossomState
    state.blossom.store.delete(&hash).map_err(|e| match e {
        cw721_nips::buds::BlobStoreError::NotFound => StatusCode::NOT_FOUND,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok(StatusCode::NO_CONTENT)
}

/// POST /blobs or /upload — BUD-07 upload with optional payment.
async fn blob_upload(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    body: Body,
) -> Result<impl IntoResponse, StatusCode> {
    let claims = state
        .blossom
        .auth
        .verify(&headers)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    buds::check_action(&claims, "upload")?;

    let bytes = axum::body::to_bytes(body, 10 * 1024 * 1024)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    if let Some(pv) = &state.blossom.payment {
        let proof = buds::extract_payment_proof(&headers)?;
        pv.verify_upload(&proof, bytes.len() as u64)
            .map_err(|_| StatusCode::PAYMENT_REQUIRED)?;
    }

    // Delegates to TreeStore via BlossomState
    let descriptor = state
        .blossom
        .store
        .put(bytes.to_vec())
        .map_err(|e| match e {
            cw721_nips::buds::BlobStoreError::AlreadyExists => StatusCode::CONFLICT,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    Ok(Json(serde_json::json!({
        "url": descriptor.url,
        "sha256": hex::encode(&descriptor.sha256),
        "size": descriptor.size,
        "type": descriptor.mime_type,
        "uploaded": descriptor.uploaded,
    })))
}

/// GET /blobs — list all blob hashes with metadata.
async fn blob_list(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, StatusCode> {
    let claims = state
        .blossom
        .auth
        .verify(&headers)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    buds::check_action(&claims, "list")?;

    // Lists via QMD content-ID index (O(1) lookups, no filesystem scan)
    let hashes = state.blossom.store.list();
    // Single lock acquisition for all lookups
    let metas = state.tree_store.blob_metas(&hashes);
    let blob_entries: Vec<serde_json::Value> = hashes
        .iter()
        .map(|h| {
            let meta = metas.get(h);
            serde_json::json!({
                "sha256": hex::encode(h),
                "size": meta.map(|m| m.size),
                "uploaded": meta.map(|m| m.uploaded),
            })
        })
        .collect();

    Ok(Json(serde_json::json!({ "blobs": blob_entries })))
}

// ── Tree (Merkle) Handlers ────────────────────────────────────────────────────

/// GET /trees/{id} — get merkle tree by ID.
async fn tree_get(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let tree = state.tree_store.load(&id).ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(tree))
}

/// POST /trees/{id} — save/upload merkle tree (authenticated).
async fn tree_save(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<String>,
    Json(input): Json<TreeInput>,
) -> Result<impl IntoResponse, StatusCode> {
    state
        .blossom
        .auth
        .verify(&headers)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    // Intent: AuthClaims is dropped — tree/headstash use a single admin-key model
    // where any valid BUD auth token grants write access. No action-level scoping
    // (like blob routes do with check_action) is applied here; the auth itself is
    // the permission. Add per-action checks (e.g. "write:tree") if needed later.

    input.validate().map_err(|e| {
        tracing::warn!(id = %id, error = %e, "invalid tree input");
        StatusCode::BAD_REQUEST
    })?;

    state.tree_store.save(&id, input).map_err(|e| {
        tracing::error!(id = %id, error = %e, "failed to save tree");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(StatusCode::CREATED)
}

/// DELETE /trees/{id} — delete merkle tree (authenticated).
async fn tree_delete(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    state
        .blossom
        .auth
        .verify(&headers)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    state.tree_store.delete(&id).map_err(|e| {
        tracing::error!(id = %id, error = %e, "failed to delete tree");
        StatusCode::NOT_FOUND
    })?;
    Ok(StatusCode::NO_CONTENT)
}

/// GET /trees — list all tree IDs.
async fn tree_list(State(state): State<Arc<AppState>>) -> Json<Vec<String>> {
    Json(state.tree_store.list())
}

// ── Headstash Handlers ─────────────────────────────────────────────────────────

/// GET /headstash/{id} — get headstash registration.
async fn headstash_get(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let data = state
        .headstash_store
        .get_headstash(&id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(data))
}

/// POST /headstash/{id} — save headstash registration (authenticated).
async fn headstash_save(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<String>,
    Json(data): Json<serde_json::Value>,
) -> Result<impl IntoResponse, StatusCode> {
    state
        .blossom
        .auth
        .verify(&headers)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    // See tree_save for design rationale — AuthClaims dropped intentionally:
    // single admin-key model, no per-action scoping.

    state
        .headstash_store
        .set_headstash(&id, &data)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::CREATED)
}

// ── Provider Feeder Delegate ──────────────────────────────────────────────────

#[cfg(feature = "ve")]
/// Start a provider feeder (delegates to ve_server).
pub async fn run_provider_feeder(
    state: Arc<AppState>,
    provider_idx: usize,
    name: String,
    mode: TransportMode,
) {
    ve_server::run_provider_feeder(state, provider_idx, name, mode).await
}

// ── Binary entry point ─────────────────────────────────────────────────────────
//
// The actual binary lives in `src/bin/server.rs` which uses these library
// functions. This module provides the shared router, handlers, and AppState.
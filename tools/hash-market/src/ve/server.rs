//! Axum HTTP server for the validator sidecar.
//!
//! Exposes:
//! - `GET  /health` — liveness check + provider status
//! - `GET  /providers` — list registered providers and their latest data status
//! - `POST /extend-vote` — produce a signed vote extension for a specific chain
//! - `POST /verify-vote-extension` — verify a peer's signed extension

use crate::client::ve::SignedVoteExtension;
use crate::msg::VoteExtensionHashData;
use crate::server::AppState;
use crate::transport::TransportMode;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

// ---------------------------------------------------------------------------
// Provider — a named data source for a specific (chain_uid, algo) pair
// ---------------------------------------------------------------------------

/// Metadata about a registered provider.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProviderStatus {
    pub name: String,
    pub chain_uid: String,
    pub algo: String,
    pub running: bool,
    pub last_update: Option<u64>,
    pub foreign_height: Option<u64>,
}

/// Keyed data entry from a provider.
pub struct ProviderData {
    pub data: VoteExtensionHashData,
    pub received_at: u64,
}

/// Composite key for provider data lookup.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ProviderKey {
    pub chain_uid: String,
    pub algo: String,
}

// ---------------------------------------------------------------------------
// AppState — shared across all handlers
// ---------------------------------------------------------------------------

/// Build the axum router.
pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/providers", get(list_providers))
        .route("/vote-extension", get(vote_extension_get))
        .route("/extend-vote", post(extend_vote))
        .route("/verify-vote-extension", post(verify_vote_extension))
        .with_state(state)
}

async fn health(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let statuses = state.provider_status.read().await;
    let active = statuses.iter().filter(|s| s.running).count();
    let total = statuses.len();
    let data_count = state.provider_data.read().await.len();
    Json(serde_json::json!({
        "status": "ok",
        "providers": { "active": active, "total": total },
        "chains_with_data": data_count,
    }))
}

async fn list_providers(State(state): State<Arc<AppState>>) -> Json<Vec<ProviderStatus>> {
    Json(state.provider_status.read().await.clone())
}

// ---------------------------------------------------------------------------
// vote-extension (GET) — terpd ABCI++ ExtendVoteHandler calls this
// ---------------------------------------------------------------------------

/// Query parameters for GET /vote-extension.
#[derive(serde::Deserialize, Default)]
pub struct VoteExtensionQuery {
    /// Optional chain_uid filter. When omitted, returns the single provider's data
    /// (or 400 if multiple providers exist and chain_uid is ambiguous).
    pub chain_uid: Option<String>,
}

/// GET /vote-extension — returns the latest provider data in the format
/// terpd's `ExtendVoteHandler` expects (JSON with hex-encoded root).
///
/// This is the bridge between the Cosmos SDK validator and the sidecar:
/// terpd calls `GET {sidecar_url}/vote-extension?chain_uid=<id>` every block during the
/// vote phase. If no data is available, returns 503 (which terpd treats
/// as "empty extension" — backwards compatible).
async fn vote_extension_get(
    State(state): State<Arc<AppState>>,
    Query(query): Query<VoteExtensionQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let store = state.provider_data.read().await;
    if store.is_empty() {
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    }

    // Filter by chain_uid if provided
    let entries: Vec<&ProviderData> = if let Some(cuid) = &query.chain_uid {
        store
            .iter()
            .filter(|(k, _)| k.chain_uid == *cuid)
            .map(|(_, v)| v)
            .collect()
    } else {
        store.values().collect()
    };

    if entries.is_empty() {
        return Err(StatusCode::NOT_FOUND);
    }

    // Return the most recently updated entry among matching providers
    let entry = entries
        .into_iter()
        .max_by_key(|e| e.received_at)
        .ok_or(StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(Json(serde_json::json!({
        "chain_uid": entry.data.chain_uid,
        "algo": entry.data.algo,
        "root": hex::encode(&entry.data.root),
        "foreign_height": entry.data.foreign_height,
        "foreign_block_time": entry.data.foreign_block_time,
    })))
}

// ---------------------------------------------------------------------------
// extend-vote — now takes chain_uid to select which provider's data to sign
// ---------------------------------------------------------------------------

/// Request body for POST /extend-vote.
#[derive(serde::Deserialize)]
pub struct ExtendVoteRequest {
    pub height: u64,
    /// Which chain to produce the extension for. If omitted and only one
    /// provider has data, that one is used.
    pub chain_uid: Option<String>,
    #[serde(default = "default_algo")]
    pub algo: String,
}

fn default_algo() -> String {
    "keccak256".to_string()
}

/// Response body for POST /extend-vote.
#[derive(serde::Serialize)]
pub struct ExtendVoteResponse {
    pub chain_uid: String,
    pub algo: String,
    pub foreign_height: u64,
    /// Hex-encoded protobuf extension
    pub extension: String,
    /// Hex-encoded signature
    pub signature: String,
    /// Hex-encoded public key
    pub public_key: String,
}

async fn extend_vote(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ExtendVoteRequest>,
) -> Result<Json<ExtendVoteResponse>, StatusCode> {
    let store = state.provider_data.read().await;

    let data = if let Some(chain_uid) = &req.chain_uid {
        // Explicit chain requested
        let key = ProviderKey {
            chain_uid: chain_uid.clone(),
            algo: req.algo.clone(),
        };
        store.get(&key).ok_or(StatusCode::NOT_FOUND)?
    } else if store.len() == 1 {
        // Single provider — use it
        store
            .values()
            .next()
            .ok_or(StatusCode::SERVICE_UNAVAILABLE)?
    } else if store.is_empty() {
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    } else {
        // Multiple providers, chain_uid required
        return Err(StatusCode::BAD_REQUEST);
    };

    let signed = state
        .ve_handler
        .sign_extension(&state.chain_id, req.height, &data.data)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ExtendVoteResponse {
        chain_uid: data.data.chain_uid.clone(),
        algo: data.data.algo.clone(),
        foreign_height: data.data.foreign_height,
        extension: hex::encode(&signed.extension),
        signature: hex::encode(&signed.signature),
        public_key: hex::encode(&signed.public_key),
    }))
}

/// Request body for POST /extend-vote/all — sign all available chains.
#[derive(serde::Deserialize)]
pub struct ExtendVoteAllRequest {
    pub height: u64,
}

// ---------------------------------------------------------------------------
// verify-vote-extension — unchanged
// ---------------------------------------------------------------------------

#[derive(serde::Deserialize)]
pub struct VerifyRequest {
    pub height: u64,
    pub extension: String,
    pub signature: String,
    pub public_key: String,
}

#[derive(serde::Serialize)]
pub struct VerifyResponse {
    pub valid: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

async fn verify_vote_extension(
    State(state): State<Arc<AppState>>,
    Json(req): Json<VerifyRequest>,
) -> Json<VerifyResponse> {
    let result = (|| -> anyhow::Result<()> {
        let signed = SignedVoteExtension {
            extension: hex::decode(&req.extension)?,
            signature: hex::decode(&req.signature)?,
            public_key: hex::decode(&req.public_key)?,
        };
        state
            .ve_handler
            .verify_extension(&state.chain_id, req.height, &signed)?;
        Ok(())
    })();

    match result {
        Ok(()) => Json(VerifyResponse {
            valid: true,
            error: None,
        }),
        Err(e) => Json(VerifyResponse {
            valid: false,
            error: Some(e.to_string()),
        }),
    }
}

// ---------------------------------------------------------------------------
// Provider feeder — one per provider, writes to shared state
// ---------------------------------------------------------------------------

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Start a transport listener for a single provider and feed data into shared state.
pub async fn run_provider_feeder(
    state: Arc<AppState>,
    provider_idx: usize,
    name: String,
    mode: TransportMode,
) {
    // Mark running
    {
        let mut statuses = state.provider_status.write().await;
        if let Some(s) = statuses.get_mut(provider_idx) {
            s.running = true;
        }
    }

    tracing::info!(provider = %name, "provider feeder starting");

    let mut rx = crate::transport::start(mode);
    while let Some(data) = rx.recv().await {
        let key = ProviderKey {
            chain_uid: data.chain_uid.clone(),
            algo: data.algo.clone(),
        };
        let height = data.foreign_height;
        let now = now_unix();

        state.provider_data.write().await.insert(
            key,
            ProviderData {
                data,
                received_at: now,
            },
        );

        // Update status
        {
            let mut statuses = state.provider_status.write().await;
            if let Some(s) = statuses.get_mut(provider_idx) {
                s.last_update = Some(now);
                s.foreign_height = Some(height);
            }
        }
    }

    // Transport ended — mark not running
    {
        let mut statuses = state.provider_status.write().await;
        if let Some(s) = statuses.get_mut(provider_idx) {
            s.running = false;
        }
    }
    tracing::warn!(provider = %name, "provider feeder stopped");
}

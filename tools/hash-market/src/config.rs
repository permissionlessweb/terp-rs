//! Canonical config types for hash-market binaries.
//!
//! These types are the single source of truth for TOML/JSON config serialization.
//! Both the binary entry points (`bin/server.rs`, `bin/client.rs`) and the test
//! suite (`tests/src/suite/hashmerchant.rs`) import from here instead of
//! redefining private duplicates.
//!
//! # Design
//!
//! - All fields mirror the exact TOML key names for serde compatibility.
//! - `Serialize` is derived so test suites can write config files programmatically.
//! - `Deserialize` is derived so binaries can parse config files at startup.
//! - No runtime state (e.g. running, last_update) — those belong in `ProviderStatus`.

use serde::{Deserialize, Serialize};

// ── Server config ───────────────────────────────────────────────────────────

/// TOML config for `hash-market-server`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    /// HTTP bind address (e.g. "0.0.0.0:9090")
    pub bind: String,
    /// CometBFT chain ID
    pub chain_id: String,
    /// Hex-encoded secp256k1 signing key
    pub signing_key: String,
    /// Optional data directory for tree/blob/headstash storage (default: "data")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_dir: Option<String>,
    /// Provider definitions
    pub providers: Vec<ProviderConfig>,
}

/// A single provider definition in the server TOML config.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// Human-readable name
    pub name: String,
    /// Chain UID (e.g. "ethereum-mainnet", "terp-test-1")
    pub chain_uid: String,
    /// Hash algorithm (default: "keccak256")
    #[serde(default = "default_algo")]
    pub algo: String,
    /// Transport mode: "grpc", "http_poll", or "websocket"
    pub mode: String,
    /// Transport address (gRPC listen, HTTP URL, or WS URL)
    pub address: String,
    /// Polling interval in seconds (default: 12)
    #[serde(default = "default_interval")]
    pub interval_secs: u64,
}

fn default_algo() -> String {
    "keccak256".to_string()
}

fn default_interval() -> u64 {
    12
}

// ── Client config ───────────────────────────────────────────────────────────

/// TOML config for `hash-market-client`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientConfig {
    /// Ethereum JSON-RPC URL
    pub eth_rpc: String,
    /// hash-market-server URL (e.g. "http://localhost:9090")
    pub sidecar_url: String,
    /// Runtime identifier
    pub runtime_id: String,
    /// Chain UID (e.g. "ethereum-mainnet")
    pub chain_uid: String,
    /// Polling interval in seconds (default: 12)
    #[serde(default = "default_interval")]
    pub interval_secs: u64,
    /// Ethereum account to prove state for
    pub account_address: String,
    /// Storage keys to include in the proof
    #[serde(default)]
    pub storage_keys: Vec<String>,
}
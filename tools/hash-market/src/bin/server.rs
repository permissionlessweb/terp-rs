//! hash-market-server binary.
//!
//! Supports multiple concurrent providers, each feeding data from a
//! different foreign chain via its own transport.

use anyhow::{Context, Result};
use clap::Parser;
use hash_market::custody::local::LocalSecp256k1;
use hash_market::custody::Custody;
use hash_market::server::{self, AppState, ProviderStatus};
use hash_market::transport::TransportMode;
use hash_market::ve::VoteExtensionHandler;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Parser)]
#[command(name = "hash-market-server", about = "Validator sidecar for hashmerchant")]
struct Cli {
    /// Path to config file
    #[arg(short, long, default_value = "config.toml")]
    config: String,
}

#[derive(serde::Deserialize)]
struct Config {
    /// Address to bind the HTTP server (e.g. "0.0.0.0:9090")
    bind: String,
    /// CometBFT chain ID
    chain_id: String,
    /// Signing key (hex-encoded secp256k1 private key)
    signing_key: String,
    /// Provider definitions — each is a named data source for a (chain_uid, algo) pair.
    /// Use `[[providers]]` in TOML for multiple entries.
    providers: Vec<ProviderConfig>,
}

#[derive(serde::Deserialize)]
struct ProviderConfig {
    /// Human-readable name (e.g. "ethereum-alchemy", "cosmoshub-rpc")
    name: String,
    /// Foreign chain UID — must match a RegisteredChain on-chain
    chain_uid: String,
    /// Hash algorithm (e.g. "keccak256", "sha256")
    #[serde(default = "default_algo")]
    algo: String,
    /// Transport: "grpc", "http_poll", or "websocket"
    mode: String,
    /// Transport address/URL
    address: String,
    /// Polling interval in seconds (for http_poll mode)
    #[serde(default = "default_interval")]
    interval_secs: u64,
}

fn default_algo() -> String {
    "keccak256".to_string()
}

fn default_interval() -> u64 {
    12
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();
    let config_str = std::fs::read_to_string(&cli.config)
        .with_context(|| format!("failed to read config: {}", cli.config))?;
    let config: Config = toml::from_str(&config_str).context("invalid config")?;

    anyhow::ensure!(
        !config.providers.is_empty(),
        "at least one [[providers]] entry is required"
    );

    // Init custody
    let custody = LocalSecp256k1::from_hex(&config.signing_key)?;
    tracing::info!(
        custody = custody.label(),
        pubkey = hex::encode(custody.public_key()),
        providers = config.providers.len(),
        "custody initialized"
    );

    let ve_handler = VoteExtensionHandler::new(Box::new(custody));

    // Build provider status list
    let provider_statuses: Vec<ProviderStatus> = config
        .providers
        .iter()
        .map(|p| ProviderStatus {
            name: p.name.clone(),
            chain_uid: p.chain_uid.clone(),
            algo: p.algo.clone(),
            running: false,
            last_update: None,
            foreign_height: None,
        })
        .collect();

    let state = Arc::new(AppState {
        ve_handler,
        chain_id: config.chain_id.clone(),
        provider_data: RwLock::new(HashMap::new()),
        provider_status: RwLock::new(provider_statuses),
        current_height: RwLock::new(0),
    });

    // Start a feeder task for each provider
    for (idx, provider) in config.providers.iter().enumerate() {
        let transport_mode = match provider.mode.as_str() {
            "grpc" => TransportMode::Grpc {
                listen_addr: provider.address.clone(),
            },
            "http_poll" => TransportMode::HttpPoll {
                url: provider.address.clone(),
                interval_secs: provider.interval_secs,
            },
            "websocket" => TransportMode::WebSocket {
                url: provider.address.clone(),
            },
            other => anyhow::bail!(
                "provider '{}': unknown transport mode '{other}'",
                provider.name
            ),
        };

        tracing::info!(
            provider = %provider.name,
            chain_uid = %provider.chain_uid,
            algo = %provider.algo,
            mode = %provider.mode,
            address = %provider.address,
            "registering provider"
        );

        let feeder_state = state.clone();
        let name = provider.name.clone();
        tokio::spawn(server::run_provider_feeder(
            feeder_state,
            idx,
            name,
            transport_mode,
        ));
    }

    // Start HTTP server
    let app = server::router(state);
    let listener = tokio::net::TcpListener::bind(&config.bind).await?;
    tracing::info!(bind = %config.bind, "server listening");
    axum::serve(listener, app).await?;

    Ok(())
}

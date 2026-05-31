//! Hash-market server binary.
//!
//! Unified server: Vote Extension (VE) + Blossom (BUD protocol) + Headstash.
//! Uses the `hash_market::server` library module for shared state and router.
//! Config struct is the canonical `hash_market::config::Config` shared with tests.

use anyhow::Context;
use clap::Parser;

use hash_market::config::Config;
use hash_market::custody::Custody;
use hash_market::server::AppState;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Parser)]
#[command(
    name = "hash-market-server",
    about = "Unified hash-market server: VE + Blossom + Headstash"
)]
struct Cli {
    /// Path to config file
    #[arg(short, long, default_value = "config.toml")]
    config: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

    // Init custody for VE signing
    let custody = hash_market::custody::local::LocalSecp256k1::from_hex(&config.signing_key)?;
    tracing::info!(
        custody = custody.label(),
        pubkey = hex::encode(custody.public_key()),
        providers = config.providers.len(),
        "custody initialized"
    );

    let ve_handler = hash_market::client::ve::VoteExtensionHandler::new(Box::new(custody));
    let data_dir = config
        .data_dir
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("data"));

    // Create unified state (TreeStore-backed BlossomState via f/ prefix)
    let state = Arc::new(AppState::new(
        ve_handler,
        config.chain_id.clone(),
        data_dir,
    )?);

    // Build provider statuses and register via add_provider
    use hash_market::ve::ProviderStatus;
    for p in &config.providers {
        state
            .add_provider(ProviderStatus {
                name: p.name.clone(),
                chain_uid: p.chain_uid.clone(),
                algo: p.algo.clone(),
                running: false,
                last_update: None,
                foreign_height: None,
            })
            .await;
    }

    // Start provider feeders
    use hash_market::transport::TransportMode;
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

        let feeder_state = state.clone();
        let name = provider.name.clone();
        tokio::spawn(hash_market::server::run_provider_feeder(
            feeder_state,
            idx,
            name,
            transport_mode,
        ));
    }

    // Build the unified router and start listening
    let app = hash_market::server::router(state);
    let listener = tokio::net::TcpListener::bind(&config.bind).await?;
    tracing::info!(bind = %config.bind, "hash-market-server listening");
    axum::serve(listener, app).await?;

    Ok(())
}
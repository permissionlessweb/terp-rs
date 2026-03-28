//! hash-market-client binary.

use anyhow::{Context, Result};
use clap::Parser;
use hash_market::client;
use hash_market::eth::EthClient;

#[derive(Parser)]
#[command(name = "hash-market-client", about = "ETH proof poller for hashmerchant sidecar")]
struct Cli {
    /// Path to config file
    #[arg(short, long, default_value = "client.toml")]
    config: String,
}

#[derive(serde::Deserialize)]
struct Config {
    /// Ethereum JSON-RPC URL
    eth_rpc: String,
    /// hash-market-server URL (e.g. "http://localhost:9090")
    sidecar_url: String,
    /// Runtime identifier
    runtime_id: String,
    /// Chain UID (e.g. "ethereum-mainnet")
    chain_uid: String,
    /// Polling interval in seconds
    #[serde(default = "default_interval")]
    interval_secs: u64,
    /// Ethereum account to prove state for
    account_address: String,
    /// Storage keys to include in the proof
    #[serde(default)]
    storage_keys: Vec<String>,
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

    let eth = EthClient::new(&config.eth_rpc);
    tracing::info!(
        rpc = %config.eth_rpc,
        chain_uid = %config.chain_uid,
        account = %config.account_address,
        "client starting"
    );

    client::run_poll_loop(
        &eth,
        &config.sidecar_url,
        &config.runtime_id,
        &config.chain_uid,
        config.interval_secs,
        &config.storage_keys,
        &config.account_address,
    )
    .await
}

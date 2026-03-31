//! Akash Network chain support for ict-rs.
//!
//! Provides Akash-specific genesis modifications and a convenience
//! [`spawn_akash_chain`] helper that returns a fully configured local
//! Akash node ready for deployment testing.
//!
//! Feature-gated by `akash`.

use std::collections::HashMap;

use serde_json::json;

use crate::chain::{Chain, ChainConfig, ChainType, GenesisStyle, SigningAlgorithm};
use crate::error::Result;
use crate::runtime::DockerImage;
#[cfg(feature = "testing")]
use crate::testing::TestChain;
#[cfg(feature = "testing")]
use tracing::info;

/// Modify raw Akash genesis JSON for local testing.
///
/// Sets Akash-specific module params: staking/mint denoms, fast governance,
/// deployment module defaults, and market/provider params.
pub fn modify_akash_genesis(_cfg: &ChainConfig, raw: Vec<u8>) -> Result<Vec<u8>> {
    let mut genesis: serde_json::Value = serde_json::from_slice(&raw)
        .map_err(|e| crate::error::IctError::Config(format!("parse genesis: {}", e)))?;

    // ── Staking ──────────────────────────────────────────────────────────
    if let Some(params) = genesis.pointer_mut("/app_state/staking/params") {
        params["bond_denom"] = json!("uakt");
        // Fast unbonding for tests.
        params["unbonding_time"] = json!("120s");
    }

    // ── Mint ─────────────────────────────────────────────────────────────
    if let Some(params) = genesis.pointer_mut("/app_state/mint/params") {
        params["mint_denom"] = json!("uakt");
    }

    // ── Governance (fast voting for tests) ───────────────────────────────
    if let Some(params) = genesis.pointer_mut("/app_state/gov/params") {
        params["min_deposit"] = json!([{"denom": "uakt", "amount": "10000000"}]);
        params["voting_period"] = json!("90s");
        params["max_deposit_period"] = json!("90s");
    }
    // v1beta1 fallback
    if let Some(dp) = genesis.pointer_mut("/app_state/gov/deposit_params") {
        dp["min_deposit"] = json!([{"denom": "uakt", "amount": "10000000"}]);
        dp["max_deposit_period"] = json!("90s");
    }
    if let Some(vp) = genesis.pointer_mut("/app_state/gov/voting_params") {
        vp["voting_period"] = json!("90s");
    }

    // ── Crisis ────────────────────────────────────────────────────────────
    if let Some(fee) = genesis.pointer_mut("/app_state/crisis/constant_fee") {
        fee["denom"] = json!("uakt");
    }

    // ── Akash deployment module ──────────────────────────────────────────
    // The default genesis already includes min_deposits with BOTH uakt and
    // uact (required by Akash v2.1+). Don't overwrite — the defaults are correct.

    // ── Akash market module ──────────────────────────────────────────────
    // Similarly, bid_min_deposits already has both denoms. Only override
    // order_max_bids if we want a non-default value.
    if let Some(params) = genesis.pointer_mut("/app_state/market/params") {
        params["order_max_bids"] = json!(20);
    }

    let output = serde_json::to_vec_pretty(&genesis)
        .map_err(|e| crate::error::IctError::Config(format!("serialize genesis: {}", e)))?;
    Ok(output)
}

/// Build a full Akash [`ChainConfig`] with genesis modifier and fast block times.
pub fn akash_chain_config() -> ChainConfig {
    let mut config_overrides = HashMap::new();
    // Enable API + gRPC in app.toml.
    config_overrides.insert(
        "config/app.toml".into(),
        json!({
            "api": { "enable": true, "address": "tcp://0.0.0.0:1317" },
            "grpc": { "enable": true, "address": "0.0.0.0:9090" },
        }),
    );
    // Fast blocks.
    config_overrides.insert(
        "config/config.toml".into(),
        json!({
            "consensus": {
                "timeout_propose": "200ms",
                "timeout_propose_delta": "200ms",
                "timeout_prevote": "200ms",
                "timeout_prevote_delta": "200ms",
                "timeout_precommit": "200ms",
                "timeout_precommit_delta": "200ms",
                "timeout_commit": "200ms",
            }
        }),
    );

    ChainConfig {
        chain_type: ChainType::Cosmos,
        name: "akash".to_string(),
        chain_id: "akash-local-1".to_string(),
        images: vec![DockerImage {
            repository: "ghcr.io/akash-network/node".to_string(),
            version: "latest".to_string(),
            uid_gid: None,
        }],
        bin: "akash".to_string(),
        bech32_prefix: "akash".to_string(),
        denom: "uakt".to_string(),
        coin_type: 118,
        signing_algorithm: SigningAlgorithm::Secp256k1,
        gas_prices: "0.025uakt".to_string(),
        gas_adjustment: 1.5,
        trusting_period: "336h".to_string(),
        block_time: "200ms".to_string(),
        genesis: None,
        modify_genesis: Some(Box::new(modify_akash_genesis)),
        pre_genesis: None,
        config_file_overrides: config_overrides,
        additional_start_args: Vec::new(),
        env: Vec::new(),
        sidecar_configs: Vec::new(),
        faucet: None,
        genesis_style: GenesisStyle::Modern,
    }
}

/// A spawned Akash chain with host-accessible endpoints.
#[cfg(feature = "testing")]
pub struct SpawnedAkashChain {
    pub tc: TestChain,
    pub rpc: String,
    pub grpc: String,
    pub rest: String,
    pub chain_id: String,
    pub faucet_mnemonic: String,
}

/// Well-known test mnemonic for faucet/deployer accounts.
#[cfg(feature = "testing")]
pub const TEST_MNEMONIC: &str =
    "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

/// Spawn a single-validator Akash chain for testing.
///
/// Uses a default test name. For parallel tests, use [`spawn_akash_chain_named`]
/// with unique names to avoid Docker container collisions.
#[cfg(feature = "testing")]
pub async fn spawn_akash_chain() -> Result<SpawnedAkashChain> {
    spawn_akash_chain_named("akash-local", TEST_MNEMONIC).await
}

/// Spawn a single-validator Akash chain with a caller-chosen `test_name`.
///
/// Each invocation must use a **unique** `test_name` to avoid Docker container
/// name collisions (container name = `ict-{test_name}-{chain_id}-val-0`).
/// Recovers `faucet_mnemonic` into the node keyring and funds it from the
/// validator account so it can serve as a faucet.
#[cfg(feature = "testing")]
pub async fn spawn_akash_chain_named(
    test_name: &str,
    faucet_mnemonic: &str,
) -> Result<SpawnedAkashChain> {
    use crate::testing::{TestChain, TestChainConfig};

    let config = akash_chain_config();
    let chain_id = config.chain_id.clone();

    let tc = TestChain::setup(
        test_name,
        TestChainConfig {
            chain_config: config,
            num_validators: 1,
            num_full_nodes: 0,
            genesis_wallets: Vec::new(),
        },
    )
    .await?;

    // Recover the faucet key into the running node and fund it from
    // the validator account (which has genesis funds).
    let primary = tc.chain.primary_node()?;
    let _ = primary.recover_key("faucet", faucet_mnemonic).await?;
    let faucet_address = primary.get_key_address("faucet").await?;

    // Fund the faucet from the validator account
    let fund_amount = "100000000000uakt".to_string(); // 100k AKT
    let output = primary
        .bank_send("validator", &faucet_address, &fund_amount, "0.025uakt")
        .await?;
    if output.exit_code != 0 {
        let stderr = output.stderr_str();
        let stdout = output.stdout_str();
        tracing::error!(
            exit_code = output.exit_code,
            stderr = %stderr,
            stdout = %stdout,
            "fund faucet bank_send failed"
        );
        return Err(crate::error::IctError::ExecFailed {
            exit_code: output.exit_code,
            stderr: format!("fund faucet failed: {}", stderr),
        });
    }
    // Wait a block for the tx to be included
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    let rpc = tc.chain.host_rpc_address();
    let grpc = tc.chain.host_grpc_address();
    // REST API uses host_api_port (auto-assigned), not a string-replace hack
    let rest = primary
        .host_api_port
        .map(|p| format!("http://localhost:{p}"))
        .unwrap_or_else(|| "http://localhost:1317".to_string());

    info!(
        chain_id = %chain_id,
        test_name = %test_name,
        rpc = %rpc,
        grpc = %grpc,
        rest = %rest,
        faucet_address = %faucet_address,
        "Akash chain spawned"
    );

    Ok(SpawnedAkashChain {
        tc,
        rpc,
        grpc,
        rest,
        chain_id,
        faucet_mnemonic: faucet_mnemonic.to_string(),
    })
}

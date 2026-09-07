//! Hash-market server binary.
//!
//! Default mode: **Merkle/BUD host** for NFT whitelist proofs (no vote extensions).
//! Validator sidecar: set `ve_enabled = true` and build with `--features ve`.
//! Connect-style price bounds: `[oracle] bounds_enabled = true` + `kind = "price_bound"`.

use anyhow::Context;
use clap::Parser;

use hash_market::config::Config;
#[cfg(feature = "ve")]
use hash_market::custody::Custody;
use hash_market::oracle::ProviderKind;
use hash_market::server::AppState;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Parser)]
#[command(
    name = "hash-market-server",
    about = "hash-market: BUD + Merkle whitelist host (optional ABCI++ VE + Connect-style oracle bounds)"
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
    config.validate().map_err(anyhow::Error::msg)?;

    let ve_enabled = config.wants_ve();
    let bounds_enabled = config.wants_oracle_bounds();
    let data_dir = config
        .data_dir
        .as_ref()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("data"));
    #[cfg(not(feature = "ve"))]
    if ve_enabled {
        anyhow::bail!(
            "ve_enabled=true in config but binary was built without `--features ve`. \
             Rebuild with: cargo build -p hash-market --features full-sidecar"
        );
    }

    #[cfg(feature = "ve")]
    let ve_handler = if ve_enabled {
        let key = config
            .signing_key
            .as_ref()
            .context("signing_key required for VE")?;
        let custody = hash_market::custody::local::LocalSecp256k1::from_hex(key)?;
        tracing::info!(
            custody = custody.label(),
            pubkey = hex::encode(custody.public_key()),
            providers = config.providers.len(),
            "VE custody initialized"
        );
        Some(hash_market::client::ve::VoteExtensionHandler::new(Box::new(
            custody,
        )))
    } else {
        tracing::info!("vote extensions disabled — merkle/BUD host mode");
        None
    };

    #[cfg(not(feature = "ve"))]
    tracing::info!("vote extensions not compiled — merkle/BUD host mode");

    let oracle_policy = config
        .oracle
        .as_ref()
        .map(|o| o.to_policy())
        .unwrap_or_default();

    if bounds_enabled {
        tracing::info!(
            method = oracle_policy.method.as_str(),
            min_sources = oracle_policy.min_sources,
            max_age_secs = oracle_policy.max_age_secs,
            "oracle bounds enabled (Connect-style; role=bound_only)"
        );
    }

    let distribution = if config.wants_distribution() {
        let dcfg = config.distribution.clone().unwrap_or_default();
        tracing::info!(
            ipfs = dcfg.ipfs_api.is_some(),
            pin_on_upload = dcfg.pin_on_upload,
            webhooks = dcfg.webhooks.len(),
            "content distribution enabled (BUD label + optional IPFS)"
        );
        Some(std::sync::Arc::new(
            hash_market::content::DistributionRuntime::open(
                &data_dir,
                dcfg,
                config.public_bud_base(),
            )?,
        ))
    } else {
        None
    };

    let mut state = AppState::new_full(
        #[cfg(feature = "ve")]
        ve_handler,
        config.chain_id.clone(),
        data_dir,
        ve_enabled,
        bounds_enabled,
        oracle_policy.clone(),
        distribution,
    )?;

    // Private notes + blossom auth (noop default; set [notes_auth] for live ops)
    {
        let notes_auth = config.notes_auth.clone().unwrap_or_default();
        let mode = notes_auth.mode.clone();
        state.set_auth_verifier(notes_auth.into_verifier());
        tracing::info!(%mode, "notes/blossom AuthVerifier installed");
    }

    // Cashu mesh: canonical mint discovery cache + encrypted wallet stash
    {
        let enabled = config.wants_cashu_mesh();
        let public_list = config.cashu_public_mint_list();
        state.set_cashu_mesh_policy(enabled, public_list);
        tracing::info!(
            enabled,
            public_mint_list = public_list,
            "cashu mesh policy (canonical mint registry cache + wallet stash)"
        );
    }

    let state = Arc::new(state);

    // ── State-root VE feeders (legacy Path B) ────────────────────────────────
    #[cfg(feature = "ve")]
    if ve_enabled {
        use hash_market::transport::TransportMode;
        use hash_market::ve::ProviderStatus;

        for p in config.state_root_providers() {
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

        // Price-bound providers also appear in status when VE is on
        for p in config.price_bound_providers() {
            let market = p.market_id.as_deref().unwrap_or("");
            let chain_uid = if p.chain_uid.is_empty() {
                hash_market::oracle::price_chain_uid(market)
            } else {
                p.chain_uid.clone()
            };
            let algo = if p.algo.is_empty() || p.algo == "keccak256" {
                hash_market::oracle::ALGO_PRICE_MEDIAN_V1.to_string()
            } else {
                p.algo.clone()
            };
            state
                .add_provider(ProviderStatus {
                    name: p.name.clone(),
                    chain_uid,
                    algo,
                    running: false,
                    last_update: None,
                    foreign_height: None,
                })
                .await;
        }

        for (idx, provider) in config
            .providers
            .iter()
            .filter(|p| p.provider_kind() == ProviderKind::StateRoot)
            .enumerate()
        {
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
                "static" => {
                    // Lab/e2e only — fixed Model A root without external feeder
                    let root_hex = provider
                        .static_root
                        .as_deref()
                        .unwrap_or("")
                        .trim()
                        .trim_start_matches("0x");
                    if root_hex.len() < 16 {
                        anyhow::bail!(
                            "provider '{}': mode=static requires static_root hex (≥16 chars)",
                            provider.name
                        );
                    }
                    let root = hex::decode(root_hex).map_err(|e| {
                        anyhow::anyhow!(
                            "provider '{}': invalid static_root hex: {e}",
                            provider.name
                        )
                    })?;
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .map(|d| d.as_secs() as i64)
                        .unwrap_or(0);
                    let data = hash_market::msg::VoteExtensionHashData {
                        runtime_id: format!("hash-market-static-{}", config.chain_id),
                        chain_uid: provider.chain_uid.clone(),
                        algo: provider.algo.clone(),
                        root,
                        foreign_height: provider.static_height.unwrap_or(1),
                        foreign_block_time: provider.static_block_time.unwrap_or(now),
                        ics23_proof: vec![],
                    };
                    TransportMode::Static {
                        data,
                        interval_secs: provider.interval_secs.max(1),
                    }
                }
                other => anyhow::bail!(
                    "provider '{}': unknown transport mode '{other}'",
                    provider.name
                ),
            };

            let feeder_state = state.clone();
            let name = provider.name.clone();
            // Index in full provider_status list: state roots first, then price — match order above
            tokio::spawn(hash_market::server::run_provider_feeder(
                feeder_state,
                idx,
                name,
                transport_mode,
            ));
        }
    }

    // ── Connect-style price_bound feeders (multi-source → one mid per market) ─
    if bounds_enabled {
        let runtime_id = format!("hash-market-{}", config.chain_id);
        let publish_ve = ve_enabled;
        for provider in config.price_bound_providers() {
            let feeder_state = state.clone();
            let p = provider.clone();
            let policy = oracle_policy.clone();
            let rid = runtime_id.clone();
            tokio::spawn(async move {
                hash_market::oracle::run_price_bound_feeder(
                    feeder_state,
                    p,
                    policy,
                    rid,
                    publish_ve,
                )
                .await;
            });
        }
    }

    let app = hash_market::server::router(state);
    let listener = tokio::net::TcpListener::bind(&config.bind).await?;
    tracing::info!(
        bind = %config.bind,
        mode = if ve_enabled {
            "validator-sidecar"
        } else if bounds_enabled {
            "merkle-bud-host+oracle-bounds"
        } else {
            "merkle-bud-host"
        },
        "hash-market-server listening"
    );
    axum::serve(listener, app).await?;

    Ok(())
}

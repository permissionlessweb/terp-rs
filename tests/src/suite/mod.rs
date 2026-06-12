//! SidecarFleet — composable fleet of Terp Network sidecars (Docker, subprocess, tokio).
//!
//! Combines a cw-orch `Daemon` chain environment with typed access to all
//! Terp Network sidecar services: hashmerchant, merkle-server, indexer,
//! relayer, minio-ipfs, and nostr-relay.
//!
//! Each sidecar element is a typed field with its own lifecycle, health
//! checking, and endpoint resolution. Sidecars are started in dependency
//! order using `SidecarRegistry` for inter-sidecar communication.

pub mod contracts;
pub mod deploy_data;
pub mod hashmerchant;
pub mod sidecar;

#[cfg(feature = "docker")]
pub mod indexer;
#[cfg(feature = "docker")]
pub mod minio_ipfs;
#[cfg(feature = "nostr")]
pub mod nostr;
#[cfg(feature = "docker")]
pub mod relayer;

use std::sync::Arc;

use anyhow::{Context, Result};
use std::collections::HashMap;
use cw_orch::daemon::Daemon;
use cw_orch::environment::{ChainInfoOwned, ChainKind, Environment, NetworkInfoOwned};

use ict_rs::runtime::IctRuntime;

pub use sidecar::{
    DockerSidecar, EndpointMap, HealthStatus, InProcessSidecar, SidecarBinaryResolver,
    SidecarId, SidecarRegistry, SubprocessConfig, SubprocessSidecar,
};
use sidecar::TerpSidecar;
// pub use contracts::TerpNetworkSuite as DeploySuite;
// pub use deploy_data::TerpNetworkDeployData;

use hashmerchant::HashMerchantSuite;
use indexer::IndexerSuite;
use hashmerchant::MerkleServerSuite;
use minio_ipfs::MinioIpfsSuite;
use nostr::NostrRelaySuite;
use relayer::RelayerSuite;

// ---------------------------------------------------------------------------
// SidecarFleet
// ---------------------------------------------------------------------------

/// A fleet of sidecars (Docker, subprocess, tokio) with a chain Daemon.
///
/// # Example
///
/// ```ignore
/// use scripts::suite::SidecarFleet;
///
/// #[tokio::test]
/// async fn test_full_suite() {
///     let mut suite = SidecarFleet::new("my-test", "mnemonic ...")
///         .with_hashmerchant_defaults("terp-test-1")
///         .with_merkle_server_defaults()
///         .with_minio_ipfs_defaults()
///         .with_nostr_relay("nostr-relay")
///         .with_minimal_indexer("argus");
///
///     suite.start_all().await.unwrap();
///
///     // Use chain
///     suite.environment().upload(&contract).unwrap();
///
///     // Use sidecars
///     let ve = suite.hashmerchant.vote_extension("terp-test").await.unwrap();
///
///     suite.stop_all().await.unwrap();
/// }
/// ```
pub struct SidecarFleet {
    /// Test name identifier
    name: String,
    /// cw-orch Daemon for chain interactions
    daemon: Option<Daemon>,
    /// Docker runtime backend
    runtime: Option<Arc<dyn ict_rs::runtime::RuntimeBackend>>,
    /// Sidecar registry for endpoint sharing
    registry: SidecarRegistry,
    /// Typed sidecar fields
    pub hashmerchant: Option<HashMerchantSuite>,
    pub merkle_server: Option<MerkleServerSuite>,
    pub minio_ipfs: Option<MinioIpfsSuite>,
    pub nostr_relay: Option<NostrRelaySuite>,
    pub relayer: Option<RelayerSuite>,
    pub indexer: Option<IndexerSuite>,
    /// Chain info for Daemon construction
    chain_info: Option<ChainInfoOwned>,
    mnemonic: Option<String>,
}

impl SidecarFleet {
    /// Create a new suite builder. Call `.with_*()` methods to add sidecars,
    /// then call `.start_all()` to launch everything.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            daemon: None,
            runtime: None,
            registry: SidecarRegistry::new(),
            hashmerchant: None,
            merkle_server: None,
            minio_ipfs: None,
            nostr_relay: None,
            relayer: None,
            indexer: None,
            chain_info: None,
            mnemonic: None,
        }
    }

    /// Create a suite with local chain info and a mnemonic for the Daemon.
    /// Shorthand for chain_info + start_all.
    pub fn with_local_chain(mut self, mnemonic: &str) -> Self {
        self.chain_info = Some(ChainInfoOwned {
            chain_id: "terp-test-1".to_string(),
            gas_denom: "uterp".to_string(),
            gas_price: 0.025,
            grpc_urls: vec!["http://127.0.0.1:9090".to_string()],
            lcd_url: None,
            fcd_url: None,
            network_info: NetworkInfoOwned {
                chain_name: "terp".to_string(),
                pub_address_prefix: "terp".to_string(),
                coin_type: 118,
            },
            kind: ChainKind::Local,
        });
        self.mnemonic = Some(mnemonic.to_string());
        self
    }

    // ── Builder methods ──────────────────────────────────────────────────

    /// Add a HashMerchant suite with default config.
    pub fn with_hashmerchant_defaults(mut self, chain_id: &str) -> Self {
        self.hashmerchant = Some(HashMerchantSuite::with_defaults(chain_id));
        self
    }

    /// Add a custom HashMerchant suite.
    pub fn with_hashmerchant(mut self, suite: HashMerchantSuite) -> Self {
        self.hashmerchant = Some(suite);
        self
    }

    /// Add a Merkle server with a fresh keypair.
    #[allow(clippy::result_unit_err)]
    pub fn with_merkle_server_defaults(mut self) -> Self {
        if let Ok(suite) = MerkleServerSuite::with_defaults() {
            self.merkle_server = Some(suite);
        }
        self
    }

    /// Add a custom Merkle server.
    pub fn with_merkle_server(mut self, suite: MerkleServerSuite) -> Self {
        self.merkle_server = Some(suite);
        self
    }

    /// Add a MinIO-IPFS sidecar with default credentials.
    pub fn with_minio_ipfs_defaults(mut self) -> Self {
        self.minio_ipfs = Some(MinioIpfsSuite::with_defaults("minio-ipfs"));
        self
    }

    /// Add a Nostr relay.
    pub fn with_nostr_relay(mut self, id: &str) -> Self {
        self.nostr_relay = Some(NostrRelaySuite::new(id));
        self
    }

    /// Add an IBC relayer.
    pub fn with_relayer(mut self, suite: RelayerSuite) -> Self {
        self.relayer = Some(suite);
        self
    }

    /// Add a minimal Argus indexer (server + Postgres backends only).
    pub fn with_minimal_indexer(mut self, id: &str) -> Self {
        self.indexer = Some(IndexerSuite::with_defaults(id).with_minimal());
        self
    }

    /// Add a full Argus indexer (all infrastructure).
    pub fn with_full_indexer(mut self, id: &str) -> Self {
        self.indexer = Some(IndexerSuite::with_defaults(id));
        self
    }

    // ── Lifecycle ─────────────────────────────────────────────────────────

    /// Start all configured sidecars in dependency order, plus the Daemon.
    ///
    /// Dependency order:
    /// 1. MinIO-IPFS, Nostr relay (no deps)
    /// 2. Merkle server (no deps)
    /// 3. Hashmerchant server + client (depends on chain for VE, but starts independently)
    /// 4. Indexer (depends on chain for RPC)
    /// 5. Relayer (depends on both chains)
    pub async fn start_all(&mut self) -> Result<()> {
        // Initialize Docker runtime if not set
        if self.runtime.is_none() {
            let rt = IctRuntime::Docker(Default::default())
                .into_backend()
                .await
                .context("Failed to create Docker runtime backend")?;
            self.runtime = Some(rt);
        }

        // Inject runtime into sidecars that need it
        if let Some(ref rt) = self.runtime {
            if let Some(ref mut idx) = self.indexer {
                idx.runtime = Some(rt.clone());
            }
            if let Some(ref mut nostr) = self.nostr_relay {
                nostr.runtime = Some(rt.clone());
            }
            if let Some(ref mut relayer) = self.relayer {
                relayer.runtime = Some(rt.clone());
            }
            if let Some(ref mut minio) = self.minio_ipfs {
                minio.runtime = Some(rt.clone());
            }
        }

        // Start sidecars in dependency order
        let start_order: Vec<&mut dyn TerpSidecar> = vec![
            // Tier 0: no deps
            // MinIO-IPFS and Nostr are Docker-sidecars handled via suite ops
        ];

        // Due to borrow issues with the Option fields, we start sidecars
        // by taking and replacing them. This avoids mutable borrow conflicts.

        if let Some(ref mut hm) = self.hashmerchant {
            let ep = hm.start(&self.registry).await?;
            self.registry.register("hashmerchant".into(), ep);
        }

        if let Some(ref mut ms) = self.merkle_server {
            let ep = ms.start(&self.registry).await?;
            self.registry.register("merkle-server".into(), ep);
        }

        if let Some(ref mut mini) = self.minio_ipfs {
            let ep = mini.start(&self.registry).await?;
            self.registry.register("minio-ipfs".into(), ep);
        }

        if let Some(ref mut nr) = self.nostr_relay {
            let ep = nr.start(&self.registry).await?;
            self.registry.register("nostr-relay".into(), ep);
        }

        if let Some(ref mut rel) = self.relayer {
            let ep = rel.start(&self.registry).await?;
            self.registry.register("relayer".into(), ep);
        }

        if let Some(ref mut idx) = self.indexer {
            let ep = idx.start(&self.registry).await?;
            self.registry.register("indexer".into(), ep);
        }

        // Build Daemon if chain_info is set
        if let Some(ref chain_info) = self.chain_info {
            let mnemonic = self.mnemonic.as_deref().context("no mnemonic set")?;
            let daemon = cw_orch::daemon::DaemonBuilder::new(chain_info.clone())
                .handle(&tokio::runtime::Handle::current())
                .mnemonic(mnemonic)
                .is_test(true)
                .build()
                .context("Failed to build cw-orch Daemon")?;
            self.daemon = Some(daemon);
        }

        Ok(())
    }

    /// Stop all sidecars and clean up.
    pub async fn stop_all(&mut self) -> Result<()> {
        // Stop in reverse dependency order
        if let Some(ref mut idx) = self.indexer {
            idx.stop().await.ok();
        }
        if let Some(ref mut rel) = self.relayer {
            rel.stop().await.ok();
        }
        if let Some(ref mut nr) = self.nostr_relay {
            nr.stop().await.ok();
        }
        if let Some(ref mut mini) = self.minio_ipfs {
            mini.stop().await.ok();
        }
        if let Some(ref mut ms) = self.merkle_server {
            ms.stop().await.ok();
        }
        if let Some(ref mut hm) = self.hashmerchant {
            hm.stop().await.ok();
        }
        Ok(())
    }

    /// Check health of all sidecars. Returns a map of sidecar_id -> health.
    pub async fn health_all(&self) -> Result<HashMap<String, HealthStatus>> {
        let mut results = HashMap::new();

        if let Some(ref hm) = self.hashmerchant {
            results.insert("hashmerchant".into(), hm.health().await.unwrap_or(HealthStatus::Unknown));
        }
        if let Some(ref ms) = self.merkle_server {
            results.insert("merkle-server".into(), ms.health().await.unwrap_or(HealthStatus::Unknown));
        }
        if let Some(ref mini) = self.minio_ipfs {
            results.insert("minio-ipfs".into(), mini.health().await.unwrap_or(HealthStatus::Unknown));
        }
        if let Some(ref nr) = self.nostr_relay {
            results.insert("nostr-relay".into(), nr.health().await.unwrap_or(HealthStatus::Unknown));
        }
        if let Some(ref rel) = self.relayer {
            results.insert("relayer".into(), rel.health().await.unwrap_or(HealthStatus::Unknown));
        }
        if let Some(ref idx) = self.indexer {
            results.insert("indexer".into(), idx.health().await.unwrap_or(HealthStatus::Unknown));
        }

        Ok(results)
    }

    /// Get a specific endpoint from a running sidecar.
    /// Returns `None` if the sidecar isn't started or the endpoint doesn't exist.
    pub fn endpoint(&self, sidecar: &str, endpoint: &str) -> Option<String> {
        self.registry.get_endpoint(sidecar, endpoint)
    }
}

// ── Environment<Daemon> implementation ─────────────────────────────────────

impl Environment<Daemon> for SidecarFleet {
    fn environment(&self) -> &Daemon {
        self.daemon.as_ref().expect("Daemon not initialized — call start_all() first or set with_local_chain()")
    }
}

// ── Drop ────────────────────────────────────────────────────────────────────

impl Drop for SidecarFleet {
    fn drop(&mut self) {
        // Best-effort cleanup in drop context
        let _ = self.runtime.take();
    }
}
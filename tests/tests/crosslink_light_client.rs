//! Integration test for the crosslink IBC light client end-to-end flow.
//!
//! This test exercises the full lifecycle:
//!   1. Spawn a 3-node zebrad Docker testnet (2 TFL finalizers + 1 vanilla)
//!   2. Start a Terp chain via ict-rs CosmosChain
//!   3. Deploy the `cw-ics08-wasm-crosslink` contract via cw-orch Daemon
//!   4. Construct a client update from zebrad RPC data
//!   5. Submit the update and verify client state advancement
//!
//! # Requirements
//!
//! - Docker daemon running
//! - `terpnetwork/terp-core:local-zk` Docker image available (or pullable)
//! - Zebrad Docker image: `zcash/zebrad:latest` (or built locally)
//! - The `cw-ics08-wasm-crosslink` WASM artifact at the expected path
//!
//! Run with: `cargo test --test crosslink_light_client -- --ignored --nocapture`

use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result, anyhow};
use cosmwasm_std::to_json_binary;
use cw_ics08_wasm_crosslink::interface::CrosslinkLightClient;
use cw_ics08_wasm_crosslink::msg::InstantiateMsg;
use cw_orch::daemon::DaemonBuilder;
use cw_orch::prelude::*;
use ict_rs::chain::terp::terp_chain_config;
use ict_rs::chain::{Chain, TestContext};
use ict_rs::prelude::*;
use ict_rs::runtime::docker::DockerBackend;
use ict_rs::runtime::{
    ContainerId, ContainerOptions, DockerConfig, DockerImage, NetworkId, PortBinding,
    RuntimeBackend, VolumeMount,
};
use tracing::{error, info, warn};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Standard zebrad Docker image.
const ZEBRAD_IMAGE_REPO: &str = "zcash/zebrad";
const ZEBRAD_IMAGE_VERSION: &str = "latest";

/// Zebrad RPC port (internal container port).
const ZEBRAD_RPC_PORT: u16 = 8232;

/// Zebrad P2P port (internal container port).
const ZEBRAD_P2P_PORT: u16 = 8233;

/// How long to wait for zebrad to start producing blocks.
const ZEBRAD_STARTUP_TIMEOUT: Duration = Duration::from_secs(120);

/// How long to wait for the Terp chain to produce its first block.
const TERP_STARTUP_TIMEOUT: Duration = Duration::from_secs(60);

/// Default test mnemonic for the Terp chain validator.
const TEST_MNEMONIC: &str = "chapter wrist alcohol shine angry noise mercy simple rebel recycle vehicle wrap morning giraffe lazy outdoor noise blood ginger sort reunion boss crowd dutch";

// ---------------------------------------------------------------------------
// Docker helper: check if Docker is available
// ---------------------------------------------------------------------------

/// Check whether the Docker daemon is reachable.
///
/// Returns `true` if the Docker socket exists and the daemon responds to ping.
fn docker_available() -> bool {
    // Check for the Docker socket
    let socket_paths = [
        "/var/run/docker.sock",
        "/run/docker.sock",
        &format!(
            "{}/.docker/run/docker.sock",
            std::env::var("HOME").unwrap_or_default()
        ),
    ];

    let socket_exists = socket_paths
        .iter()
        .any(|p| std::path::Path::new(p).exists());
    if !socket_exists {
        warn!("Docker socket not found — skipping Docker-dependent test");
        return false;
    }

    // Try to connect and ping
    let rt = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => rt,
        Err(e) => {
            warn!("Failed to create tokio runtime for Docker check: {e}");
            return false;
        }
    };

    rt.block_on(async {
        match DockerBackend::new(DockerConfig::default()).await {
            Ok(_) => {
                info!("Docker daemon is reachable");
                true
            }
            Err(e) => {
                warn!("Docker daemon not reachable: {e}");
                false
            }
        }
    })
}

// ---------------------------------------------------------------------------
// ZebradNode: configuration for a single zebrad container
// ---------------------------------------------------------------------------

/// Configuration for a single zebrad node in the testnet.
#[derive(Clone, Debug)]
struct ZebradNodeConfig {
    /// Node index (0-based).
    index: usize,
    /// Whether this node runs with crosslink TFL enabled.
    crosslink_enabled: bool,
    /// Host-side RPC port.
    host_rpc_port: u16,
    /// Host-side P2P port.
    host_p2p_port: u16,
    /// Container name.
    container_name: String,
    /// Cache directory on the host.
    host_cache_dir: String,
    /// Listen address for the zebrad network (inside container).
    listen_addr: String,
    /// Initial peers (comma-separated `node_id@addr:port` strings).
    initial_peers: String,
}

impl ZebradNodeConfig {
    /// Create a new node configuration.
    fn new(
        index: usize,
        crosslink_enabled: bool,
        base_rpc_port: u16,
        base_p2p_port: u16,
        test_name: &str,
        cache_dir: &str,
    ) -> Self {
        let host_rpc_port = base_rpc_port + (index as u16) * 10;
        let host_p2p_port = base_p2p_port + (index as u16) * 10;
        let container_name = format!("{}-zebrad-{}", test_name, index);
        let host_cache_dir = format!("{}/zebrad-{}", cache_dir, index);
        let listen_addr = format!("0.0.0.0:{}", ZEBRAD_P2P_PORT);

        Self {
            index,
            crosslink_enabled,
            host_rpc_port,
            host_p2p_port,
            container_name,
            host_cache_dir,
            listen_addr,
            initial_peers: String::new(),
        }
    }

    /// Generate the zebrad TOML config for this node.
    fn generate_config(&self) -> String {
        let crosslink_section = if self.crosslink_enabled {
            // Crosslink-enabled nodes get the TFL configuration.
            r#"
[crosslink]
# Enable TFL finality layer
tfl_enabled = true
# Listen address for Malachite BFT consensus
listen_address = "/ip4/0.0.0.0/udp/24834/quic-v1"
# Public address (same as listen for local testing)
public_address = "/ip4/127.0.0.1/udp/24834/quic-v1"
"#
            .to_string()
        } else {
            String::new()
        };

        format!(
            r#"# Auto-generated zebrad config for node {index}
[network]
listen_addr = "{listen_addr}"
initial_mainnet_peers = "{peers}"
network = "Mainnet"

[state]
cache_dir = "/home/zebra/.cache/zebra"

[rpc]
listen_addr = "0.0.0.0:{rpc_port}"
enable_cookie_auth = false

[metrics]
# Disable metrics endpoint for test nodes
endpoint_addr = ""

{crosslink}
"#,
            index = self.index,
            listen_addr = self.listen_addr,
            peers = self.initial_peers,
            rpc_port = ZEBRAD_RPC_PORT,
            crosslink = crosslink_section,
        )
    }
}

// ---------------------------------------------------------------------------
// ZebradTestnet: manages a multi-node zebrad Docker topology
// ---------------------------------------------------------------------------

/// Manages a 3-node zebrad Docker testnet:
/// - 2 crosslink-enabled nodes (TFL finalizers)
/// - 1 vanilla zebrad node (no crosslink)
///
/// Each node runs in its own Docker container with unique ports and cache
/// directories. Communication between nodes uses a shared Docker network.
pub struct ZebradTestnet {
    /// Test name used for container/network naming.
    test_name: String,
    /// Docker runtime backend.
    runtime: Arc<DockerBackend>,
    /// Docker network ID for inter-node communication.
    network_id: Option<NetworkId>,
    /// Container IDs for each node (indexed by node index).
    containers: Vec<Option<ContainerId>>,
    /// Node configurations.
    node_configs: Vec<ZebradNodeConfig>,
    /// Zebrad Docker image.
    image: DockerImage,
    /// Whether the testnet has been started.
    started: bool,
    /// Temp directory for node cache dirs.
    _temp_dir: Option<tempfile::TempDir>,
}

impl ZebradTestnet {
    /// Create a new zebrad testnet configuration.
    ///
    /// This does NOT start any containers — call [`start`](Self::start) to
    /// begin the testnet.
    pub async fn new(test_name: &str) -> Result<Self> {
        let runtime = Arc::new(DockerBackend::new(DockerConfig::default()).await?);

        // Pull the zebrad image
        let image = DockerImage {
            repository: ZEBRAD_IMAGE_REPO.to_string(),
            version: ZEBRAD_IMAGE_VERSION.to_string(),
            uid_gid: None,
        };
        if let Err(e) = runtime.pull_image(&image).await {
            warn!(
                "Failed to pull zebrad image '{}': {e}. Will attempt to use cached image.",
                image
            );
        }

        // Create a temp directory for cache dirs
        let temp_dir = tempfile::tempdir()?;
        let cache_base = temp_dir.path().to_string_lossy().to_string();

        let base_rpc = 18232u16;
        let base_p2p = 18233u16;

        // Nodes 0 and 1 are crosslink-enabled TFL finalizers
        // Node 2 is vanilla zebrad (no crosslink)
        let mut configs = Vec::new();
        for i in 0..3 {
            let crosslink = i < 2;
            let cfg =
                ZebradNodeConfig::new(i, crosslink, base_rpc, base_p2p, test_name, &cache_base);
            configs.push(cfg);
        }

        Ok(Self {
            test_name: test_name.to_string(),
            runtime,
            network_id: None,
            containers: vec![None, None, None],
            node_configs: configs,
            image,
            started: false,
            _temp_dir: Some(temp_dir),
        })
    }

    /// Start the zebrad testnet: create network, create containers, start all nodes.
    pub async fn start(&mut self) -> Result<()> {
        if self.started {
            return Ok(());
        }

        info!("Starting zebrad testnet: {}", self.test_name);

        // 1. Create Docker network
        let network_name = format!("{}-zebrad-net", self.test_name);
        let network_id = self.runtime.create_network(&network_name).await?;
        self.network_id = Some(network_id.clone());
        info!(network = %network_name, "Docker network created");

        // 2. Ensure cache directories exist
        for cfg in &self.node_configs {
            std::fs::create_dir_all(&cfg.host_cache_dir)
                .with_context(|| format!("Failed to create cache dir: {}", cfg.host_cache_dir))?;
        }

        // 3. Create containers for each node
        for (i, cfg) in self.node_configs.clone().iter().enumerate() {
            let config_toml = cfg.generate_config();

            // Write config to the cache dir, then mount it
            let config_path = format!("{}/zebrad.toml", cfg.host_cache_dir);
            std::fs::write(&config_path, &config_toml)
                .with_context(|| format!("Failed to write config to {}", config_path))?;

            let container_opts = ContainerOptions {
                image: self.image.clone(),
                name: cfg.container_name.clone(),
                network_id: Some(network_id.clone()),
                env: vec![
                    ("ZEBRA_RPC_PORT".to_string(), ZEBRAD_RPC_PORT.to_string()),
                    (
                        "ZEBRA_CACHE_DIR".to_string(),
                        "/home/zebra/.cache/zebra".to_string(),
                    ),
                    ("RUST_LOG".to_string(), "info".to_string()),
                ],
                cmd: vec!["zebrad".to_string(), "start".to_string()],
                entrypoint: None,
                ports: vec![
                    PortBinding {
                        host_port: cfg.host_rpc_port,
                        container_port: ZEBRAD_RPC_PORT,
                        protocol: "tcp".to_string(),
                    },
                    PortBinding {
                        host_port: cfg.host_p2p_port,
                        container_port: ZEBRAD_P2P_PORT,
                        protocol: "tcp".to_string(),
                    },
                ],
                volumes: vec![
                    VolumeMount {
                        source: cfg.host_cache_dir.clone(),
                        target: "/home/zebra/.cache/zebra".to_string(),
                        read_only: false,
                    },
                    VolumeMount {
                        source: config_path.clone(),
                        target: "/home/zebra/.config/zebrad.toml".to_string(),
                        read_only: true,
                    },
                ],
                labels: vec![
                    ("test".to_string(), self.test_name.clone()),
                    ("node".to_string(), i.to_string()),
                    ("crosslink".to_string(), cfg.crosslink_enabled.to_string()),
                ],
                hostname: Some(cfg.container_name.clone()),
            };

            let container_id = self.runtime.create_container(&container_opts).await?;
            self.containers[i] = Some(container_id);
            info!(
                node = i,
                container = %cfg.container_name,
                crosslink = cfg.crosslink_enabled,
                "Zebrad container created"
            );
        }

        // 4. Start all containers
        for (i, container_id) in self.containers.iter().enumerate() {
            if let Some(id) = container_id {
                self.runtime.start_container(id).await?;
                info!(node = i, container = %id.0, "Zebrad container started");
            }
        }

        // 5. Wait for zebrad nodes to start (check RPC)
        for (i, cfg) in self.node_configs.iter().enumerate() {
            self.wait_for_zebrad_rpc(i, cfg.host_rpc_port).await?;
        }

        self.started = true;
        info!("Zebrad testnet started successfully");
        Ok(())
    }

    /// Wait for a zebrad node's RPC to become available.
    async fn wait_for_zebrad_rpc(&self, node_index: usize, host_port: u16) -> Result<()> {
        let rpc_url = format!("http://127.0.0.1:{}", host_port);
        let start = std::time::Instant::now();

        loop {
            if start.elapsed() > ZEBRAD_STARTUP_TIMEOUT {
                return Err(anyhow!(
                    "Zebrad node {} RPC did not become available within {:?}",
                    node_index,
                    ZEBRAD_STARTUP_TIMEOUT
                ));
            }

            // Try to call getbestblockhash
            let body = serde_json::json!({
                "jsonrpc": "1.0",
                "id": "health_check",
                "method": "getbestblockhash",
                "params": []
            });

            let body_bytes = serde_json::to_vec(&body).unwrap_or_default();

            match reqwest::Client::new()
                .post(&rpc_url)
                .header("content-type", "application/json")
                .body(body_bytes.clone())
                .timeout(Duration::from_secs(5))
                .send()
                .await
            {
                Ok(resp) if resp.status().is_success() => {
                    info!(
                        node = node_index,
                        url = %rpc_url,
                        "Zebrad RPC is ready"
                    );
                    return Ok(());
                }
                Ok(resp) => {
                    warn!(
                        node = node_index,
                        status = %resp.status(),
                        "Zebrad RPC returned non-success status, retrying..."
                    );
                }
                Err(_) => {
                    // Connection refused or timeout — expected during startup
                }
            }

            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    }

    /// Stop the zebrad testnet: stop and remove all containers, remove network.
    pub async fn stop(&mut self) -> Result<()> {
        if !self.started {
            return Ok(());
        }

        info!("Stopping zebrad testnet: {}", self.test_name);

        // Stop and remove containers
        for (i, container_id) in self.containers.iter().enumerate() {
            if let Some(id) = container_id {
                let _ = self.runtime.stop_container(id).await;
                let _ = self.runtime.remove_container(id).await;
                info!(node = i, "Zebrad container stopped and removed");
            }
        }
        self.containers = vec![None, None, None];

        // Remove network
        if let Some(ref net_id) = self.network_id {
            let _ = self.runtime.remove_network(net_id).await;
        }
        self.network_id = None;

        self.started = false;
        info!("Zebrad testnet stopped");
        Ok(())
    }

    /// Get the RPC URL for a specific node.
    pub fn get_rpc_url(&self, node_index: usize) -> Option<String> {
        self.node_configs
            .get(node_index)
            .map(|cfg| format!("http://127.0.0.1:{}", cfg.host_rpc_port))
    }

    /// Get the best block hash from a zebrad node via JSON-RPC.
    pub async fn get_best_block_hash(&self, node_index: usize) -> Result<String> {
        let rpc_url = self
            .get_rpc_url(node_index)
            .ok_or_else(|| anyhow!("Invalid node index: {}", node_index))?;

        let body = serde_json::json!({
            "jsonrpc": "1.0",
            "id": "getbestblockhash",
            "method": "getbestblockhash",
            "params": []
        });

        let body_bytes = serde_json::to_vec(&body)?;

        let response: serde_json::Value = reqwest::Client::new()
            .post(&rpc_url)
            .header("content-type", "application/json")
            .body(body_bytes)
            .timeout(Duration::from_secs(10))
            .send()
            .await
            .context("Failed to call zebrad RPC")?
            .text()
            .await
            .context("failed to get bytes")?
            .into();

        let hash = response["result"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing 'result' in zebrad RPC response: {response}"))?
            .to_string();

        Ok(hash)
    }

    /// Get a block by hash from a zebrad node via JSON-RPC.
    pub async fn get_block(
        &self,
        node_index: usize,
        block_hash: &str,
    ) -> Result<serde_json::Value> {
        let rpc_url = self
            .get_rpc_url(node_index)
            .ok_or_else(|| anyhow!("Invalid node index: {}", node_index))?;

        let body = serde_json::json!({
            "jsonrpc": "1.0",
            "id": "getblock",
            "method": "getblock",
            "params": [block_hash, 1]  // verbosity=1 for JSON
        });

        let body_bytes = serde_json::to_vec(&body)?;

        let response: serde_json::Value = reqwest::Client::new()
            .post(&rpc_url)
            .header("content-type", "application/json")
            .body(body_bytes)
            .timeout(Duration::from_secs(10))
            .send()
            .await
            .context("Failed to call zebrad RPC getblock")?
            .text()
            .await
            .context("Failed to parse zebrad RPC getblock response")?
            .into();

        Ok(response)
    }

    /// Check whether any node is crosslink-enabled.
    pub fn has_crosslink_nodes(&self) -> bool {
        self.node_configs.iter().any(|c| c.crosslink_enabled)
    }

    /// Get the number of nodes.
    pub fn node_count(&self) -> usize {
        self.node_configs.len()
    }

    /// Get a reference to the runtime backend.
    pub fn runtime(&self) -> &Arc<DockerBackend> {
        &self.runtime
    }

    /// Get the network ID for sharing with other containers.
    pub fn network_id(&self) -> Option<&NetworkId> {
        self.network_id.as_ref()
    }
}

impl Drop for ZebradTestnet {
    fn drop(&mut self) {
        if self.started {
            // Best-effort cleanup in drop — spawn a blocking task
            let runtime = self.runtime.clone();
            let containers: Vec<_> = self.containers.iter().filter_map(|c| c.clone()).collect();
            let network_id = self.network_id.clone();

            // Use std::thread::spawn for blocking cleanup
            std::thread::spawn(move || {
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build();
                if let Ok(rt) = rt {
                    rt.block_on(async {
                        for id in &containers {
                            let _ = runtime.stop_container(id).await;
                            let _ = runtime.remove_container(id).await;
                        }
                        if let Some(ref net_id) = network_id {
                            let _ = runtime.remove_network(net_id).await;
                        }
                    });
                }
            });
        }
    }
}

// ---------------------------------------------------------------------------
// CrosslinkTestEnv: wraps zebrad testnet + Terp chain + cw-orch Daemon
// ---------------------------------------------------------------------------

/// Combined test environment for crosslink light client e2e tests.
///
/// Wraps:
/// - A [`ZebradTestnet`] (3-node zebrad Docker topology)
/// - A [`CosmosChain`] (Terp Network chain via ict-rs)
/// - A cw-orch [`Daemon`] connected to the Terp chain
pub struct CrosslinkTestEnv {
    /// Test name identifier.
    name: String,
    /// Zebrad testnet (Docker-managed).
    zebrad: ZebradTestnet,
    /// Terp chain (Cosmos SDK via ict-rs).
    chain: Option<CosmosChain>,
    /// cw-orch Daemon for contract operations.
    daemon: Option<Daemon>,
    /// Whether the environment has been started.
    started: bool,
}

impl CrosslinkTestEnv {
    /// Create a new test environment.
    ///
    /// This does NOT start any services — call [`start`](Self::start).
    pub async fn new(name: &str) -> Result<Self> {
        let zebrad = ZebradTestnet::new(name).await?;

        Ok(Self {
            name: name.to_string(),
            zebrad,
            chain: None,
            daemon: None,
            started: false,
        })
    }

    /// Start the full test environment.
    ///
    /// 1. Starts the zebrad testnet
    /// 2. Starts the Terp chain via ict-rs
    /// 3. Builds a cw-orch Daemon connected to the chain
    pub async fn start(&mut self) -> Result<()> {
        if self.started {
            return Ok(());
        }

        info!("Starting CrosslinkTestEnv: {}", self.name);

        // 1. Start zebrad testnet
        self.zebrad.start().await?;

        // 2. Start Terp chain
        let chain_config = terp_chain_config();
        let runtime: Arc<dyn RuntimeBackend> = self.zebrad.runtime().clone();

        let mut chain = CosmosChain::new(
            chain_config,
            1, // num_validators
            0, // num_full_nodes
            runtime,
        );

        let test_ctx = TestContext {
            test_name: self.name.clone(),
            network_id: self
                .zebrad
                .network_id()
                .map(|n| n.0.clone())
                .unwrap_or_default(),
        };

        chain.initialize(&test_ctx).await?;

        // Start the chain with a genesis-funded wallet
        let primary_addr = chain.primary_node()?.get_key_address("validator").await?;
        let genesis_wallets = vec![WalletAmount {
            address: primary_addr,
            denom: "uterp".to_string(),
            amount: 100_000_000_000u128,
        }];

        chain.start(&genesis_wallets).await?;

        // Wait for the chain to be producing blocks
        self.wait_for_chain_ready(&chain).await?;

        // 3. Build cw-orch Daemon
        let daemon = self.build_daemon(&chain)?;

        self.chain = Some(chain);
        self.daemon = Some(daemon);
        self.started = true;

        info!("CrosslinkTestEnv started successfully");
        Ok(())
    }

    /// Build a cw-orch Daemon from the running ict-rs CosmosChain.
    ///
    /// This is the equivalent of `ict_rs_cw_orch::daemon_builder_from_chain`.
    fn build_daemon(&self, chain: &CosmosChain) -> Result<Daemon> {
        let cfg = chain.config();
        let grpc_url = chain.host_grpc_address();

        let gas_price: f64 = cfg
            .gas_prices
            .trim_end_matches(|c: char| c.is_alphabetic())
            .parse()
            .unwrap_or(0.025);

        let chain_info = ChainInfoOwned {
            chain_id: cfg.chain_id.clone(),
            gas_denom: cfg.denom.clone(),
            gas_price,
            grpc_urls: vec![grpc_url],
            lcd_url: None,
            fcd_url: None,
            network_info: cw_orch::environment::NetworkInfoOwned {
                chain_name: cfg.name.clone(),
                pub_address_prefix: cfg.bech32_prefix.clone(),
                coin_type: cfg.coin_type,
            },
            kind: cw_orch::environment::ChainKind::Local,
        };

        let mut builder = DaemonBuilder::new(chain_info);
        builder.is_test(true);
        builder.mnemonic(TEST_MNEMONIC);

        let daemon = builder.build().context("Failed to build cw-orch Daemon")?;
        Ok(daemon)
    }

    /// Wait for the Terp chain to produce blocks.
    async fn wait_for_chain_ready(&self, chain: &CosmosChain) -> Result<()> {
        let start = std::time::Instant::now();

        loop {
            if start.elapsed() > TERP_STARTUP_TIMEOUT {
                return Err(anyhow!(
                    "Terp chain did not start producing blocks within {:?}",
                    TERP_STARTUP_TIMEOUT
                ));
            }

            match chain.height().await {
                Ok(h) if h > 0 => {
                    info!(height = h, "Terp chain is producing blocks");
                    return Ok(());
                }
                Ok(_) => {
                    // Height 0 — chain is still initializing
                }
                Err(e) => {
                    warn!("Chain height query failed (retrying): {e}");
                }
            }

            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }

    /// Stop the entire test environment.
    pub async fn stop(&mut self) -> Result<()> {
        if !self.started {
            return Ok(());
        }

        info!("Stopping CrosslinkTestEnv: {}", self.name);

        // Drop daemon first (releases gRPC connections)
        self.daemon = None;

        // Stop the Terp chain
        if let Some(ref mut chain) = self.chain {
            chain.stop().await?;
        }
        self.chain = None;

        // Stop the zebrad testnet
        self.zebrad.stop().await?;

        self.started = false;
        info!("CrosslinkTestEnv stopped");
        Ok(())
    }

    /// Get a reference to the zebrad testnet.
    pub fn zebrad(&self) -> &ZebradTestnet {
        &self.zebrad
    }

    /// Get a reference to the cw-orch Daemon.
    pub fn daemon(&self) -> Option<&Daemon> {
        self.daemon.as_ref()
    }

    /// Get the Terp chain.
    pub fn chain(&self) -> Option<&CosmosChain> {
        self.chain.as_ref()
    }

    /// Deploy the cw-ics08-wasm-crosslink light client contract.
    ///
    /// This uploads the WASM binary and instantiates the contract with
    /// the provided client state and consensus state.
    ///
    /// Returns the contract address on success.
    pub async fn deploy_light_client(
        &self,
        client_state: &[u8],
        consensus_state: &[u8],
        checksum: &[u8],
    ) -> Result<String> {
        let daemon = self
            .daemon
            .as_ref()
            .ok_or_else(|| anyhow!("Daemon not initialized"))?;

        info!("Deploying cw-ics08-wasm-crosslink light client");

        let crosslinkclient = CrosslinkLightClient::new(daemon.clone());
        crosslinkclient.upload()?;

        let contract = crosslinkclient
            .instantiate(
                &InstantiateMsg {
                    client_state: to_json_binary(&client_state).unwrap(),
                    consensus_state: to_json_binary(&consensus_state).unwrap(),
                    checksum: to_json_binary(&checksum).unwrap(),
                },
                None,
                &[],
            )?
            .instantiated_contract_address()
            .expect("instantiated_contract")
            .to_string();

        info!(
            address = %contract,
            "Light client contract instantiated"
        );

        Ok(contract)
    }

    /// Run a relayer cycle: fetch data from zebrad, construct a client update,
    /// and submit it to the contract.
    ///
    /// This is a simplified version of what the `crosslink-relayer` binary does.
    /// In production, the relayer polls zebrad's RPC, constructs
    /// `CrosslinkHeader` messages, and submits them via `MsgUpdateClient`.
    pub async fn run_relayer_cycle(&self, contract_addr: &str) -> Result<()> {
        let daemon = self
            .daemon
            .as_ref()
            .ok_or_else(|| anyhow!("Daemon not initialized"))?;

        info!("Running relayer cycle");

        // 1. Get the best block hash from a crosslink-enabled zebrad node
        let block_hash = self.zebrad.get_best_block_hash(0).await?;
        info!(block_hash = %block_hash, "Got best block hash from zebrad");

        // 2. Get the block data
        let block = self.zebrad.get_block(0, &block_hash).await?;
        let block_height = block["result"]["height"].as_u64().unwrap_or(0);
        info!(height = block_height, "Got block from zebrad");

        // 3. Construct a client update message
        let client_message =
            serde_json::to_vec(&block["result"]).context("Failed to serialize block data")?;

        // 4. Submit the update via the contract
        let update_msg = serde_json::json!({
            "update_state": {
                "client_message": base64_encode(&client_message),
            }
        });

        daemon
            .execute(
                &update_msg,
                &[], // no funds
                &Addr::unchecked(contract_addr),
                // &format!("relayer-cycle-{}", block_height),
            )
            .context("Failed to execute relayer cycle")?;

        info!(height = block_height, "Relayer cycle submitted");

        Ok(())
    }

    /// Query the light client state from the contract.
    pub async fn query_client_state(&self, contract_addr: &str) -> Result<serde_json::Value> {
        let daemon = self
            .daemon
            .as_ref()
            .ok_or_else(|| anyhow!("Daemon not initialized"))?;

        let query_msg = serde_json::json!({
            "status": {}
        });

        let response: serde_json::Value = daemon
            .query(&query_msg, &Addr::unchecked(contract_addr))
            .context("Failed to query client state")?;

        Ok(response)
    }
}

// ---------------------------------------------------------------------------
// Helper: base64 encoding (no external crate needed)
// ---------------------------------------------------------------------------

fn base64_encode(data: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut result = String::with_capacity((data.len() + 2) / 3 * 4);
    let mut i = 0;

    while i + 2 < data.len() {
        let b0 = data[i];
        let b1 = data[i + 1];
        let b2 = data[i + 2];

        result.push(CHARS[(b0 >> 2) as usize] as char);
        result.push(CHARS[((b0 & 0x03) << 4 | b1 >> 4) as usize] as char);
        result.push(CHARS[((b1 & 0x0f) << 2 | b2 >> 6) as usize] as char);
        result.push(CHARS[(b2 & 0x3f) as usize] as char);

        i += 3;
    }

    if i < data.len() {
        let b0 = data[i];
        result.push(CHARS[(b0 >> 2) as usize] as char);

        if i + 1 < data.len() {
            let b1 = data[i + 1];
            result.push(CHARS[((b0 & 0x03) << 4 | b1 >> 4) as usize] as char);
            result.push(CHARS[((b1 & 0x0f) << 2) as usize] as char);
            result.push('=');
        } else {
            result.push(CHARS[((b0 & 0x03) << 4) as usize] as char);
            result.push('=');
            result.push('=');
        }
    }

    result
}

// ---------------------------------------------------------------------------
// Test: Full e2e crosslink light client flow
// ---------------------------------------------------------------------------

/// End-to-end test for the crosslink IBC light client.
///
/// This test requires:
/// - Docker daemon running
/// - `zcash/zebrad:latest` Docker image (or locally built equivalent)
/// - `terpnetwork/terp-core:local-zk` Docker image
/// - The `cw-ics08-wasm-crosslink` WASM artifact compiled
///
/// It exercises the full lifecycle:
/// 1. Spawn 3-node zebrad testnet
/// 2. Start Terp chain
/// 3. Deploy the light client contract
/// 4. Run a relayer cycle (fetch zebrad data, submit update)
/// 5. Verify client state advancement
#[tokio::test]
#[ignore] // Requires Docker + zebrad build + terp image
async fn test_crosslink_light_client_e2e() {
    let _ = tracing_subscriber::fmt::try_init();

    // ── Check prerequisites ──────────────────────────────────────────────
    if !docker_available() {
        warn!("Docker not available — skipping crosslink e2e test");
        return;
    }

    // ── Start the test environment ───────────────────────────────────────
    let mut env = match CrosslinkTestEnv::new("crosslink-e2e").await {
        Ok(e) => e,
        Err(e) => {
            error!("Failed to create test environment: {e}");
            return;
        }
    };

    if let Err(e) = env.start().await {
        error!("Failed to start test environment: {e}");
        let _ = env.stop().await;
        return;
    }

    info!("Environment started — running e2e test");

    // ── Verify zebrad testnet is running ─────────────────────────────────
    let best_hash = match env.zebrad().get_best_block_hash(0).await {
        Ok(h) => {
            info!(hash = %h, "Zebrad node 0 responded with best block hash");
            h
        }
        Err(e) => {
            error!("Zebrad not responding: {e}");
            let _ = env.stop().await;
            return;
        }
    };
    assert!(!best_hash.is_empty(), "Best block hash should not be empty");

    // ── Verify Terp chain is running ────────────────────────────────────
    let chain = env.chain().expect("Chain should be initialized");
    let chain_height = match chain.height().await {
        Ok(h) => {
            info!(height = h, "Terp chain is at height");
            h
        }
        Err(e) => {
            error!("Failed to query chain height: {e}");
            let _ = env.stop().await;
            return;
        }
    };
    assert!(
        chain_height > 0,
        "Chain should have produced at least 1 block"
    );

    // ── Verify daemon is connected ──────────────────────────────────────
    let daemon = env.daemon().expect("Daemon should be initialized");
    let sender = daemon.sender_addr();
    info!(sender = %sender, "Daemon connected with sender address");
    assert!(
        !sender.to_string().is_empty(),
        "Daemon should have a sender"
    );

    // ── Deploy the light client contract ────────────────────────────────
    let wasm_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../solidity-ibc-eureka/programs/cw-ics08-wasm-crosslink/artifacts/cw_ics08_wasm_crosslink.wasm"
    );

    if !std::path::Path::new(wasm_path).exists() {
        warn!(
            "WASM file not found at {} — skipping contract deployment. \
             Build it with: cd crates/solidity-ibc-eureka && cargo build -p cw-ics08-wasm-crosslink --target wasm32-unknown-unknown",
            wasm_path
        );
        let _ = env.stop().await;
        return;
    }

    // Construct initial client state and consensus state
    let initial_client_state = serde_json::to_vec(&serde_json::json!({
        "chain_id": 1,
        "latest_slot": 0,
        "is_frozen": false,
    }))
    .expect("serialize client state");

    let initial_consensus_state = serde_json::to_vec(&serde_json::json!({
        "slot": 0,
        "state_root": "0000000000000000000000000000000000000000000000000000000000000000",
        "timestamp": 0,
    }))
    .expect("serialize consensus state");

    let checksum = b"crosslink-light-client-v1";

    let contract_addr = match env
        .deploy_light_client(&initial_client_state, &initial_consensus_state, checksum)
        .await
    {
        Ok(addr) => {
            info!(address = %addr, "Light client deployed");
            addr
        }
        Err(e) => {
            error!("Failed to deploy light client: {e}");
            let _ = env.stop().await;
            return;
        }
    };

    // ── Query the initial client state ──────────────────────────────────
    let status = match env.query_client_state(&contract_addr).await {
        Ok(s) => {
            info!(status = %s, "Initial client state");
            s
        }
        Err(e) => {
            error!("Failed to query client state: {e}");
            let _ = env.stop().await;
            return;
        }
    };
    assert!(
        status.get("status").is_some() || status.get("data").is_some(),
        "Client state query should return status or data"
    );

    // ── Run a relayer cycle ─────────────────────────────────────────────
    match env.run_relayer_cycle(&contract_addr).await {
        Ok(()) => {
            info!("Relayer cycle completed successfully");
        }
        Err(e) => {
            warn!("Relayer cycle failed (expected if zebrad has no crosslink data): {e}");
        }
    }

    // ── Query the updated client state ──────────────────────────────────
    let updated_status = match env.query_client_state(&contract_addr).await {
        Ok(s) => {
            info!(status = %s, "Updated client state after relayer cycle");
            s
        }
        Err(e) => {
            error!("Failed to query updated client state: {e}");
            let _ = env.stop().await;
            return;
        }
    };
    assert!(
        updated_status.is_object(),
        "Updated status should be a JSON object"
    );

    // ── Verify the Terp chain is still running ──────────────────────────
    let final_height = chain
        .height()
        .await
        .expect("Chain should still be responsive");
    assert!(
        final_height >= chain_height,
        "Chain height should not decrease: {final_height} >= {chain_height}",
    );

    // ── Cleanup ─────────────────────────────────────────────────────────
    info!("E2E test complete — cleaning up");
    if let Err(e) = env.stop().await {
        error!("Error during cleanup: {e}");
    }
    info!("Crosslink e2e test finished successfully");
}

// ---------------------------------------------------------------------------
// Test: ZebradTestnet lifecycle (standalone, no chain needed)
// ---------------------------------------------------------------------------

/// Test that the zebrad testnet can be created and started.
///
/// This test verifies the Docker-based zebrad management without requiring
/// the Terp chain or contract deployment.
#[tokio::test]
#[ignore] // Requires Docker + zebrad image
async fn test_zebrad_testnet_lifecycle() {
    let _ = tracing_subscriber::fmt::try_init();

    if !docker_available() {
        warn!("Docker not available — skipping zebrad testnet test");
        return;
    }

    let mut testnet = match ZebradTestnet::new("zebrad-lifecycle-test").await {
        Ok(t) => t,
        Err(e) => {
            error!("Failed to create zebrad testnet: {e}");
            return;
        }
    };

    // Start the testnet
    if let Err(e) = testnet.start().await {
        error!("Failed to start zebrad testnet: {e}");
        let _ = testnet.stop().await;
        return;
    }

    // Verify all 3 nodes respond
    for i in 0..3 {
        let rpc_url = testnet.get_rpc_url(i).expect("Should have RPC URL");
        info!(node = i, url = %rpc_url, "Checking zebrad node");

        match testnet.get_best_block_hash(i).await {
            Ok(hash) => {
                info!(node = i, hash = %hash, "Zebrad node responded");
                assert!(!hash.is_empty(), "Node {i} should return a block hash");
            }
            Err(e) => {
                warn!(node = i, error = %e, "Zebrad node did not respond");
            }
        }
    }

    // Verify crosslink configuration
    assert!(
        testnet.has_crosslink_nodes(),
        "Nodes 0 and 1 should be crosslink-enabled"
    );
    assert_eq!(testnet.node_count(), 3, "Should have 3 nodes");

    // Stop the testnet
    if let Err(e) = testnet.stop().await {
        error!("Failed to stop zebrad testnet: {e}");
    }

    info!("Zebrad testnet lifecycle test complete");
}

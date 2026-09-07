// //! QuickSpawnEnv — cw-orch test environment for snapshot-derived Cosmos chains.
// //!
// //! # Architecture
// //!
// //! `QuickSpawnEnv` wraps two components:
// //!
// //! 1. **`Daemon` (cw-orch)** — connects to the spawned chain via RPC/gRPC.
// //!    Implements [`TxHandler`], [`QueryHandler`], [`ChainState`], and therefore
// //!    [`Environment<Daemon>`].
// //!
// //! 2. **`ContractRegistry`** — lookup table mapping contract names to their
// //!    on-chain addresses and code IDs from the bootstrap phase.
// //!
// //! # Usage
// //!
// //! ```ignore
// //! use terp_scripts::quickspawn_env::QuickSpawnEnv;
// //!
// //! #[tokio::test]
// //! async fn test_against_snapshot() {
// //!     let env = QuickSpawnEnv::spawn("terp-test-1-h42-abc12345")
// //!         .await
// //!         .expect("Failed to spawn from snapshot");
// //!
// //!     // Use Daemon for chain operations
// //!     let resp = env.environment().upload(&my_contract).unwrap();
// //!
// //!     // Use registry for pre-deployed contracts
// //!     if let Some(addr) = env.contract_address("cw20-token") {
// //!         // query pre-deployed contract...
// //!     }
// //! }
// //! ```
// //!
// //! [`Environment<Daemon>`]: cw_orch_core::environment::Environment

// use std::collections::HashMap;
// use std::sync::Arc;

// use anyhow::Context;
// use cw_orch::daemon::{Daemon, DaemonBuilder};
// use cw_orch::environment::{ChainInfoOwned, ChainKind, Environment, NetworkInfoOwned};
// use ict_rs::chain::terp::terp_chain_config;
// use ict_rs::quickspawn::{ContractRegistry, QuickSpawnManager, SpawnedChainSet};

// /// A cw-orch test environment backed by a snapshot-derived Cosmos chain.
// ///
// /// This environment skips the genesis pipeline by restoring chain state
// /// from a previously saved snapshot. The `Daemon` connects to the spawned
// /// chain via the RPC/gRPC ports exposed by the Docker container.
// ///
// /// # Cleanup
// ///
// /// On drop, the spawned chain container is stopped and removed.
// /// This is best-effort (errors are logged, not propagated) so it must not
// /// be relied on in CI — call [`Self::stop`] explicitly if cleanup is critical.
// pub struct QuickSpawnEnv {
//     /// The cw-orch Daemon instance for chain operations.
//     daemon: Daemon,
//     /// The spawned chain (node + connection info).
//     spawned: SpawnedChainSet,
//     /// Registry of pre-deployed contracts from the bootstrap phase.
//     contract_registry: ContractRegistry,
//     /// The QuickSpawnManager that created this environment (for cleanup).
//     manager: Arc<QuickSpawnManager<ict_rs::quickspawn::LocalFsStore>>,
// }

// impl QuickSpawnEnv {
//     /// Spawn a chain from a saved snapshot and build the test environment.
//     ///
//     /// # Parameters
//     ///
//     /// * `snapshot_id` — identifier returned by [`QuickSpawnManager::bootstrap`].
//     /// * `store_root` — local filesystem path where snapshots are stored.
//     /// * `chain_id` — the Cosmos chain ID to use for the spawned chain.
//     /// * `mnemonic` — wallet mnemonic for signing transactions.
//     /// * `prefix` — bech32 address prefix (e.g. "terp").
//     ///
//     /// # Docker Requirement
//     ///
//     /// This method requires Docker to be running on the host.
//     pub async fn spawn(
//         snapshot_id: &str,
//         store_root: &str,
//         chain_id: &str,
//         mnemonic: &str,
//         prefix: &str,
//     ) -> anyhow::Result<Self> {
//         // ── 1. Build QuickSpawnManager with local filesystem store ──
//         let store = Arc::new(ict_rs::quickspawn::LocalFsStore::new(store_root));
//         let manager = Arc::new(QuickSpawnManager::new(store));

//         // ── 2. Build minimal ChainConfig for spawning ──────────────
//         let cfg = terp_chain_config();
//         let network_id = "quickspawn-net";
//         // ── 3. Spawn the chain from snapshot ───────────────────────
//         let spawned = manager
//             .spawn(snapshot_id, &cfg, network_id)
//             .await
//             .context("Failed to spawn chain from snapshot")?;

//         tracing::info!(
//             rpc_url = %spawned.rpc_url(),
//             grpc_url = %spawned.grpc_url(),
//             "Chain spawned from snapshot"
//         );

//         // ── 4. Build chain info for Daemon ─────────────────────────
//         let chain_info = ChainInfoOwned {
//             kind: ChainKind::Local,
//             chain_id: chain_id.to_string(),
//             gas_denom: "uterp".to_string(),
//             gas_price: 0.0,
//             grpc_urls: vec![spawned.grpc_url()],
//             lcd_url: None,
//             fcd_url: None,
//             network_info: NetworkInfoOwned {
//                 chain_name: "terp".to_string(),
//                 pub_address_prefix: prefix.to_string(),
//                 coin_type: 118,
//             },
//         };

//         // ── 5. Build cw-orch Daemon ────────────────────────────────
//         let daemon = DaemonBuilder::new(chain_info)
//             .handle(&tokio::runtime::Handle::current())
//             .mnemonic(mnemonic)
//             .is_test(true)
//             .build()
//             .context("Failed to build cw-orch Daemon")?;

//         // ── 6. Build contract registry ─────────────────────────────
//         let contract_registry = ContractRegistry::from_snapshot(&spawned.snapshot);

//         Ok(Self {
//             daemon,
//             spawned,
//             contract_registry,
//             manager,
//         })
//     }

//     /// Explicitly stop and remove the spawned chain container.
//     ///
//     /// This is also called on drop, but drop cannot propagate errors.
//     /// Call this explicitly in test teardown or CI to ensure the
//     /// container is cleaned up.
//     pub async fn stop(&self) {
//         if let Err(e) = self.manager.stop(&self.spawned).await {
//             tracing::warn!(error = %e, "Failed to stop QuickSpawnEnv container");
//         }
//     }

//     /// Look up a contract address by name from the bootstrap record.
//     pub fn contract_address(&self, name: &str) -> Option<&str> {
//         self.contract_registry.address(name)
//     }

//     /// Look up a code ID by contract name.
//     pub fn code_id(&self, name: &str) -> Option<u64> {
//         self.contract_registry.code_id(name)
//     }

//     /// Access the full contract registry.
//     pub fn registry(&self) -> &ContractRegistry {
//         &self.contract_registry
//     }
// }

// // ─────────────────────────────────────────────────────────────────────────────
// // Environment<Daemon> implementation
// // ─────────────────────────────────────────────────────────────────────────────

// impl Environment<Daemon> for QuickSpawnEnv {
//     /// Access the underlying cw-orch `Daemon` for chain operations.
//     ///
//     /// The returned `Daemon` implements `TxHandler`, `QueryHandler`, `ChainState`,
//     /// and therefore `CwEnv`. Use it to upload, instantiate, execute, migrate,
//     /// and query contracts.
//     fn environment(&self) -> &Daemon {
//         &self.daemon
//     }
// }

// // ─────────────────────────────────────────────────────────────────────────────
// // Drop: best-effort container cleanup
// // ─────────────────────────────────────────────────────────────────────────────

// impl Drop for QuickSpawnEnv {
//     fn drop(&mut self) {
//         // Drop is synchronous so we block_on the async stop call.
//         // This only works when called inside a tokio runtime context
//         // (which #[tokio::test] guarantees). If no runtime is active,
//         // the container leaks with a warning.
//         match tokio::runtime::Handle::try_current() {
//             Ok(handle) => {
//                 let spawned = &self.spawned;
//                 let manager = &self.manager;
//                 if let Err(e) = handle.block_on(async { manager.stop(spawned).await }) {
//                     tracing::warn!(error = %e, "QuickSpawnEnv drop: failed to stop container");
//                 }
//             }
//             Err(_) => {
//                 tracing::warn!("QuickSpawnEnv drop: no tokio runtime active, container may leak")
//             }
//         }
//     }
// }

// // ─────────────────────────────────────────────────────────────────────────────
// // QuickSpawnMultiEnv — multi-node variant
// // ─────────────────────────────────────────────────────────────────────────────

// /// A multi-node cw-orch test environment backed by a snapshot.
// ///
// /// Same as [`QuickSpawnEnv`] but wraps a [`SpawnedChainSet`], exposing all
// /// validator and full nodes. The primary validator (index 0) is used for the
// /// cw-orch Daemon connection.
// ///
// /// # Cleanup
// ///
// /// On drop, all containers and the Docker network are stopped and removed.
// pub struct QuickSpawnMultiEnv {
//     /// The cw-orch Daemon instance for chain operations (primary validator).
//     daemon: Daemon,
//     /// The spawned chain set (all validators + full nodes).
//     chain_set: SpawnedChainSet,
//     /// Registry of pre-deployed contracts from the bootstrap phase.
//     contract_registry: ContractRegistry,
//     /// The QuickSpawnManager that created this environment (for cleanup).
//     manager: Arc<QuickSpawnManager<ict_rs::quickspawn::LocalFsStore>>,
// }

// impl QuickSpawnMultiEnv {
//     /// Spawn a multi-node chain from a saved snapshot.
//     ///
//     /// Same parameters as [`QuickSpawnEnv::spawn`] but returns a multi-node env.
//     pub async fn spawn_multi(
//         snapshot_id: &str,
//         store_root: &str,
//         chain_id: &str,
//         mnemonic: &str,
//         prefix: &str,
//     ) -> anyhow::Result<Self> {
//         let store = Arc::new(ict_rs::quickspawn::LocalFsStore::new(store_root));
//         let manager = Arc::new(QuickSpawnManager::new(store));

//         let cfg = ict_rs::chain::ChainConfig {
//             chain_type: ict_rs::chain::ChainType::Cosmos,
//             name: "quickspawn".to_string(),
//             chain_id: chain_id.to_string(),
//             images: vec![ict_rs::runtime::DockerImage {
//                 repository: "ghcr.io/terpnetwork/terp-core".to_string(),
//                 version: "v5.1.6-oline".to_string(),
//                 uid_gid: None,
//             }],
//             bin: "terpd".to_string(),
//             bech32_prefix: prefix.to_string(),
//             denom: "uterp".to_string(),
//             coin_type: 118,
//             signing_algorithm: ict_rs::chain::SigningAlgorithm::Secp256k1,
//             gas_prices: "0uterp".to_string(),
//             gas_adjustment: 1.5,
//             trusting_period: "336h".to_string(),
//             block_time: "2s".to_string(),
//             genesis: None,
//             modify_genesis: None,
//             pre_genesis: None,
//             config_file_overrides: HashMap::new(),
//             additional_start_args: Vec::new(),
//             env: Vec::new(),
//             sidecar_configs: Vec::new(),
//             faucet: None,
//             genesis_style: ict_rs::chain::GenesisStyle::default(),
//         };

//         let network_id = "quickspawn-multi-net";

//         let chain_set = manager
//             .spawn_multi(snapshot_id, &cfg, network_id)
//             .await
//             .context("Failed to spawn multi-node chain from snapshot")?;

//         let primary = chain_set
//             .primary()
//             .ok_or_else(|| anyhow::anyhow!("Multi-node snapshot has no validators"))?;

//         tracing::info!(
//             rpc_url = %chain_set.rpc_url(),
//             grpc_url = %chain_set.grpc_url(),
//             total_nodes = chain_set.total_nodes(),
//             "Multi-node chain spawned from snapshot"
//         );

//         let chain_info = ChainInfoOwned {
//             kind: ChainKind::Local,
//             chain_id: chain_id.to_string(),
//             gas_denom: "uterp".to_string(),
//             gas_price: 0.0,
//             grpc_urls: vec![format!("http://127.0.0.1:{}", primary.host_grpc_port)],
//             lcd_url: None,
//             fcd_url: None,
//             network_info: NetworkInfoOwned {
//                 chain_name: "terp".to_string(),
//                 pub_address_prefix: prefix.to_string(),
//                 coin_type: 118,
//             },
//         };

//         let daemon = DaemonBuilder::new(chain_info)
//             .handle(&tokio::runtime::Handle::current())
//             .mnemonic(mnemonic)
//             .is_test(true)
//             .build()
//             .context("Failed to build cw-orch Daemon for multi-node env")?;

//         let contract_registry = ContractRegistry::from_snapshot(&chain_set.snapshot);

//         Ok(Self {
//             daemon,
//             chain_set,
//             contract_registry,
//             manager,
//         })
//     }

//     /// Explicitly stop all nodes and remove the Docker network.
//     pub async fn stop(&self) {
//         if let Err(e) = self.manager.stop_set(&self.chain_set).await {
//             tracing::warn!(error = %e, "Failed to stop QuickSpawnMultiEnv");
//         }
//     }

//     /// Look up a contract address by name.
//     pub fn contract_address(&self, name: &str) -> Option<&str> {
//         self.contract_registry.address(name)
//     }

//     /// Look up a code ID by contract name.
//     pub fn code_id(&self, name: &str) -> Option<u64> {
//         self.contract_registry.code_id(name)
//     }

//     /// Access the full contract registry.
//     pub fn registry(&self) -> &ContractRegistry {
//         &self.contract_registry
//     }

//     /// Access the spawned chain set (all nodes).
//     pub fn chain_set(&self) -> &SpawnedChainSet {
//         &self.chain_set
//     }
// }

// impl Environment<Daemon> for QuickSpawnMultiEnv {
//     fn environment(&self) -> &Daemon {
//         &self.daemon
//     }
// }

// impl Drop for QuickSpawnMultiEnv {
//     fn drop(&mut self) {
//         let chain_set = &self.chain_set;
//         // We need to use &mut T but our async fn takes &T.
//         // In practice the chain_set reference is readonly for stop.
//         match tokio::runtime::Handle::try_current() {
//             Ok(handle) => {
//                 // Get an immutable reference for stop_set
//                 // (stop_set takes &SpawnedChainSet)
//                 if let Err(e) = handle.block_on(async {
//                     // Phantom: we use self.manager with a ref to chain_set.
//                     // Since manager takes &self and stop_set takes &SpawnedChainSet,
//                     // this works with immutable references.
//                     let chain_set_ref: &SpawnedChainSet = &chain_set;
//                     self.manager.stop_set(chain_set_ref).await
//                 }) {
//                     tracing::warn!(error = %e, "QuickSpawnMultiEnv drop: failed to stop");
//                 }
//             }
//             Err(_) => tracing::warn!(
//                 "QuickSpawnMultiEnv drop: no tokio runtime active, containers may leak"
//             ),
//         }
//     }
// }

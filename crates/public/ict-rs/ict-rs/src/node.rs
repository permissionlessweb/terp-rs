use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tracing::{debug, info};

use crate::error::{IctError, Result};
use crate::runtime::{
    ContainerId, ContainerOptions, DockerImage, NetworkId, PortBinding, RuntimeBackend,
    VolumeMount,
};
use crate::tx::ExecOutput;

/// Ports used by a Cosmos SDK node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodePorts {
    pub rpc: u16,
    pub grpc: u16,
    pub p2p: u16,
    pub api: u16,
}

impl Default for NodePorts {
    fn default() -> Self {
        Self {
            rpc: 26657,
            grpc: 9090,
            p2p: 26656,
            api: 1317,
        }
    }
}

/// Represents a single chain node (validator or full node) running in a container.
///
/// Each node manages its own container lifecycle, RPC/gRPC/P2P ports, and data volume.
/// Mirrors Go ICT's `ChainNode` struct.
pub struct ChainNode {
    /// Volume name for persistent data.
    pub volume_name: String,
    /// Index within the chain's node set.
    pub index: usize,
    /// Whether this node is a validator.
    pub is_validator: bool,
    /// Docker network ID this node is connected to.
    pub network_id: String,
    /// Container image used for this node.
    pub image: DockerImage,
    /// Container ID (set after creation).
    pub container_id: Option<ContainerId>,
    /// The test this node belongs to.
    pub test_name: String,
    /// Host-mapped port for RPC.
    pub host_rpc_port: Option<u16>,
    /// Host-mapped port for gRPC.
    pub host_grpc_port: Option<u16>,
    /// Host-mapped port for P2P.
    pub host_p2p_port: Option<u16>,
    /// Host-mapped port for REST API.
    pub host_api_port: Option<u16>,
    /// Hostname within the Docker network.
    pub hostname: String,
    /// Chain binary name (e.g., "terpd", "gaiad").
    pub chain_bin: String,
    /// Chain ID this node belongs to.
    pub chain_id: String,
    /// Home directory inside the container.
    pub home_dir: String,
    /// Default container ports.
    pub ports: NodePorts,
    /// Runtime backend reference.
    pub runtime: Arc<dyn RuntimeBackend>,
}

impl std::fmt::Debug for ChainNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChainNode")
            .field("index", &self.index)
            .field("is_validator", &self.is_validator)
            .field("hostname", &self.hostname)
            .field("container_id", &self.container_id)
            .finish_non_exhaustive()
    }
}

impl ChainNode {
    /// Create a new chain node configuration (does not start the container).
    pub fn new(
        index: usize,
        is_validator: bool,
        chain_id: &str,
        chain_bin: &str,
        image: DockerImage,
        test_name: &str,
        network_id: &str,
        runtime: Arc<dyn RuntimeBackend>,
    ) -> Self {
        let node_type = if is_validator { "val" } else { "fn" };
        let hostname = format!("{chain_id}-{node_type}-{index}");
        let volume_name = format!("{test_name}-{hostname}");

        Self {
            volume_name,
            index,
            is_validator,
            network_id: network_id.to_string(),
            image,
            container_id: None,
            test_name: test_name.to_string(),
            host_rpc_port: None,
            host_grpc_port: None,
            host_p2p_port: None,
            host_api_port: None,
            hostname,
            chain_bin: chain_bin.to_string(),
            chain_id: chain_id.to_string(),
            home_dir: format!("/var/cosmos-chain/{chain_id}"),
            ports: NodePorts::default(),
            runtime,
        }
    }

    /// Container name for Docker. Includes test_name to avoid collisions
    /// when multiple tests run in parallel.
    pub fn container_name(&self) -> String {
        format!("ict-{}-{}", self.test_name, self.hostname)
    }

    /// Internal RPC address (within Docker network).
    pub fn rpc_address(&self) -> String {
        format!("http://{}:{}", self.hostname, self.ports.rpc)
    }

    /// Internal gRPC address (within Docker network).
    pub fn grpc_address(&self) -> String {
        format!("{}:{}", self.hostname, self.ports.grpc)
    }

    /// Internal P2P address (within Docker network).
    pub fn p2p_address(&self) -> String {
        format!("{}:{}", self.hostname, self.ports.p2p)
    }

    /// Host-accessible RPC address.
    pub fn host_rpc_address(&self) -> Option<String> {
        self.host_rpc_port
            .map(|p| format!("http://localhost:{p}"))
    }

    /// Host-accessible gRPC address.
    pub fn host_grpc_address(&self) -> Option<String> {
        self.host_grpc_port
            .map(|p| format!("http://localhost:{p}"))
    }

    /// Build the ContainerOptions for this node.
    fn container_options(&self) -> ContainerOptions {
        let ports = vec![
            PortBinding {
                host_port: 0, // auto-assign
                container_port: self.ports.rpc,
                protocol: "tcp".to_string(),
            },
            PortBinding {
                host_port: 0,
                container_port: self.ports.grpc,
                protocol: "tcp".to_string(),
            },
            PortBinding {
                host_port: 0,
                container_port: self.ports.p2p,
                protocol: "tcp".to_string(),
            },
            PortBinding {
                host_port: 0,
                container_port: self.ports.api,
                protocol: "tcp".to_string(),
            },
        ];

        let labels = vec![
            ("ict.test".to_string(), self.test_name.clone()),
            ("ict.chain_id".to_string(), self.chain_id.clone()),
            (
                "ict.node_type".to_string(),
                if self.is_validator {
                    "validator".to_string()
                } else {
                    "fullnode".to_string()
                },
            ),
        ];

        // Start with an idle command so we can run init/genesis via exec
        // before launching the chain binary. After the genesis pipeline,
        // the chain is started via exec_start_chain().
        ContainerOptions {
            image: self.image.clone(),
            name: self.container_name(),
            network_id: Some(NetworkId(self.network_id.clone())),
            env: Vec::new(),
            cmd: vec![
                "-c".to_string(),
                "trap 'exit 0' TERM; while true; do sleep 1; done".to_string(),
            ],
            entrypoint: Some(vec!["/bin/sh".to_string()]),
            ports,
            volumes: vec![VolumeMount {
                source: self.volume_name.clone(),
                target: self.home_dir.clone(),
                read_only: false,
            }],
            labels,
            hostname: Some(self.hostname.clone()),
        }
    }

    /// Create and start the container for this node.
    pub async fn create_container(&mut self) -> Result<()> {
        let opts = self.container_options();

        // Pre-cleanup: remove any stale container/volume from a previous failed
        // test run that left resources behind. Without this, the create would fail
        // with a 409 Conflict ("name already in use"). This is a no-op if nothing
        // exists — errors are intentionally ignored.
        let stale_id = ContainerId(self.container_name());
        let _ = self.runtime.stop_container(&stale_id).await;
        let _ = self.runtime.remove_container(&stale_id).await;
        let _ = self.runtime.remove_volume(&self.volume_name).await;

        info!(node = %self.hostname, "Creating container");
        let id = self.runtime.create_container(&opts).await?;
        self.container_id = Some(id);
        Ok(())
    }

    /// Create a new container for chain upgrade, preserving the data volume.
    ///
    /// Unlike [`create_container`], this does NOT remove the volume, so chain
    /// state from the previous version is preserved across the upgrade.
    pub async fn create_container_for_upgrade(&mut self) -> Result<()> {
        let opts = self.container_options();

        // Pre-cleanup: remove stale container but PRESERVE the volume
        let stale_id = ContainerId(self.container_name());
        let _ = self.runtime.stop_container(&stale_id).await;
        let _ = self.runtime.remove_container(&stale_id).await;
        // NOTE: No volume removal — chain state must persist across upgrades

        info!(node = %self.hostname, "Creating upgrade container (preserving data)");
        let id = self.runtime.create_container(&opts).await?;
        self.container_id = Some(id);
        Ok(())
    }

    /// Start the node container.
    pub async fn start_container(&self) -> Result<()> {
        let id = self.container_id.as_ref().ok_or_else(|| {
            IctError::Chain {
                chain_id: self.chain_id.clone(),
                source: anyhow::anyhow!("node {} has no container ID", self.hostname),
            }
        })?;
        info!(node = %self.hostname, "Starting container");
        self.runtime.start_container(id).await
    }

    /// Stop the node container.
    pub async fn stop_container(&self) -> Result<()> {
        if let Some(id) = &self.container_id {
            info!(node = %self.hostname, "Stopping container");
            self.runtime.stop_container(id).await?;
        }
        Ok(())
    }

    /// Remove the node container.
    pub async fn remove_container(&mut self) -> Result<()> {
        if let Some(id) = self.container_id.take() {
            info!(node = %self.hostname, "Removing container");
            self.runtime.remove_container(&id).await?;
        }
        Ok(())
    }

    /// Execute a chain CLI command on this node.
    pub async fn exec_cmd(&self, args: &[&str]) -> Result<ExecOutput> {
        let id = self.container_id.as_ref().ok_or_else(|| {
            IctError::Chain {
                chain_id: self.chain_id.clone(),
                source: anyhow::anyhow!("node {} has no container ID", self.hostname),
            }
        })?;

        let mut cmd = vec![self.chain_bin.as_str()];
        cmd.extend_from_slice(args);
        cmd.extend_from_slice(&["--home", &self.home_dir]);

        debug!(node = %self.hostname, cmd = ?cmd, "Executing command");
        self.runtime.exec_in_container(id, &cmd, &[]).await
    }

    /// Execute a raw command (not prefixed with chain binary).
    pub async fn exec_raw(&self, cmd: &[&str], env: &[(&str, &str)]) -> Result<ExecOutput> {
        let id = self.container_id.as_ref().ok_or_else(|| {
            IctError::Chain {
                chain_id: self.chain_id.clone(),
                source: anyhow::anyhow!("node {} has no container ID", self.hostname),
            }
        })?;
        debug!(node = %self.hostname, cmd = ?cmd, "Executing raw command");
        self.runtime.exec_in_container(id, cmd, env).await
    }

    /// Initialize the node's config directory.
    ///
    /// Matches Go ICT: `<bin> init <moniker> --chain-id <id> --home <home>`
    /// No `--overwrite` or `--default-denom` flags.
    pub async fn init_home(&self, moniker: &str) -> Result<ExecOutput> {
        self.exec_cmd(&[
            "init",
            moniker,
            "--chain-id",
            &self.chain_id,
        ])
        .await
    }

    /// Start the chain binary inside the running container (background).
    /// The container must already be running (e.g. with an idle entrypoint).
    pub async fn exec_start_chain(&self) -> Result<()> {
        let id = self.container_id.as_ref().ok_or_else(|| {
            IctError::Chain {
                chain_id: self.chain_id.clone(),
                source: anyhow::anyhow!("node {} has no container ID", self.hostname),
            }
        })?;

        // Start chain in detached (background) mode via the runtime backend.
        // This avoids issues with nohup/& not persisting after the exec session.
        let cmd = format!("{} start --home {}", self.chain_bin, self.home_dir);
        debug!(node = %self.hostname, cmd = %cmd, "Starting chain binary (detached)");
        self.runtime
            .exec_in_container_background(id, &["sh", "-c", &cmd], &[])
            .await
    }

    /// Add a genesis account.
    pub async fn add_genesis_account(
        &self,
        address: &str,
        coins: &str,
    ) -> Result<ExecOutput> {
        self.exec_cmd(&["genesis", "add-genesis-account", address, coins])
            .await
    }

    /// Generate a gentx for this validator.
    ///
    /// Matches Go ICT: includes --gas-prices and --gas-adjustment.
    pub async fn gentx(
        &self,
        key_name: &str,
        staking_amount: &str,
        gas_prices: &str,
        gas_adjustment: f64,
    ) -> Result<ExecOutput> {
        let gas_adj = format!("{gas_adjustment}");
        self.exec_cmd(&[
            "genesis",
            "gentx",
            key_name,
            staking_amount,
            "--keyring-backend",
            "test",
            "--chain-id",
            &self.chain_id,
            "--gas-prices",
            gas_prices,
            "--gas-adjustment",
            &gas_adj,
        ])
        .await
    }

    /// Collect gentxs.
    pub async fn collect_gentxs(&self) -> Result<ExecOutput> {
        self.exec_cmd(&["genesis", "collect-gentxs"]).await
    }

    /// Create a new key in the node's keyring.
    ///
    /// Pipes `yes` to satisfy any interactive prompts (mnemonic confirmation,
    /// overwrite) that cause EOF/abort errors in non-interactive Docker exec.
    ///
    /// Matches Go ICT: `keys add <name> --coin-type <ct> --keyring-backend test`
    pub async fn create_key(&self, key_name: &str, coin_type: u32) -> Result<ExecOutput> {
        let cmd = format!(
            "yes | {} keys add {} --coin-type {} --keyring-backend test --output json --home {}",
            self.chain_bin, key_name, coin_type, self.home_dir
        );
        self.exec_raw(&["sh", "-c", &cmd], &[]).await
    }

    /// Recover a key from mnemonic.
    pub async fn recover_key(&self, key_name: &str, mnemonic: &str) -> Result<ExecOutput> {
        // Echo mnemonic into the keys add --recover command
        let cmd = format!(
            "echo '{}' | {} keys add {} --recover --keyring-backend test --home {} --output json",
            mnemonic, self.chain_bin, key_name, self.home_dir
        );
        self.exec_raw(&["sh", "-c", &cmd], &[]).await
    }

    /// Get the bech32 address for a named key.
    pub async fn get_key_address(&self, key_name: &str) -> Result<String> {
        let output = self
            .exec_cmd(&[
                "keys",
                "show",
                key_name,
                "--keyring-backend",
                "test",
                "-a",
            ])
            .await?;
        Ok(output.stdout_str().trim().to_string())
    }

    /// Query balance of an address.
    pub async fn query_balance(&self, address: &str, denom: &str) -> Result<u128> {
        let output = self
            .exec_cmd(&[
                "query",
                "bank",
                "balances",
                address,
                "--denom",
                denom,
                "--output",
                "json",
            ])
            .await?;

        let json: serde_json::Value =
            serde_json::from_str(output.stdout_str().trim()).map_err(|e| {
                IctError::Chain {
                    chain_id: self.chain_id.clone(),
                    source: e.into(),
                }
            })?;

        let amount_str = json["amount"]
            .as_str()
            .or_else(|| json["balance"]["amount"].as_str())
            .unwrap_or("0");

        amount_str.parse::<u128>().map_err(|e| IctError::Chain {
            chain_id: self.chain_id.clone(),
            source: e.into(),
        })
    }

    /// Send tokens from one account to another.
    pub async fn bank_send(
        &self,
        from_key: &str,
        to_address: &str,
        amount: &str,
        gas_prices: &str,
    ) -> Result<ExecOutput> {
        self.exec_cmd(&[
            "tx",
            "bank",
            "send",
            from_key,
            to_address,
            amount,
            "--keyring-backend",
            "test",
            "--gas-prices",
            gas_prices,
            "--gas",
            "auto",
            "--gas-adjustment",
            "1.5",
            "--broadcast-mode",
            "sync",
            "--output",
            "json",
            "-y",
        ])
        .await
    }

    /// Query the current block height.
    pub async fn query_height(&self) -> Result<u64> {
        let output = self
            .exec_cmd(&["status", "--output", "json"])
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(output.stdout_str().trim()).map_err(|e| {
                IctError::Chain {
                    chain_id: self.chain_id.clone(),
                    source: e.into(),
                }
            })?;

        // Handle both CometBFT status formats
        let height_str = json["sync_info"]["latest_block_height"]
            .as_str()
            .or_else(|| json["SyncInfo"]["latest_block_height"].as_str())
            .unwrap_or("0");

        height_str.parse::<u64>().map_err(|e| IctError::Chain {
            chain_id: self.chain_id.clone(),
            source: e.into(),
        })
    }

    /// Send an IBC transfer.
    pub async fn ibc_transfer(
        &self,
        channel_id: &str,
        from_key: &str,
        to_address: &str,
        amount: &str,
        gas_prices: &str,
        memo: Option<&str>,
    ) -> Result<ExecOutput> {
        let mut args = vec![
            "tx",
            "ibc-transfer",
            "transfer",
            "transfer",
            channel_id,
            to_address,
            amount,
            "--from",
            from_key,
            "--keyring-backend",
            "test",
            "--gas-prices",
            gas_prices,
            "--gas",
            "auto",
            "--gas-adjustment",
            "1.5",
            "--broadcast-mode",
            "sync",
            "--output",
            "json",
            "-y",
        ];

        let memo_owned;
        if let Some(m) = memo {
            memo_owned = m.to_string();
            args.push("--memo");
            args.push(&memo_owned);
        }

        self.exec_cmd(&args).await
    }

    /// Export chain state at a given height.
    pub async fn export_state(&self, height: u64) -> Result<String> {
        let height_str = height.to_string();
        let output = self
            .exec_cmd(&["export", "--height", &height_str])
            .await?;
        Ok(output.stdout_str())
    }

    /// Read and parse the genesis.json file from this node's config directory.
    pub async fn read_genesis(&self) -> Result<serde_json::Value> {
        let genesis_path = format!("{}/config/genesis.json", self.home_dir);
        let output = self.exec_raw(&["cat", &genesis_path], &[]).await?;
        if output.exit_code != 0 {
            return Err(IctError::ExecFailed {
                exit_code: output.exit_code,
                stderr: output.stderr_str(),
            });
        }
        serde_json::from_str(output.stdout_str().trim()).map_err(|e| IctError::Chain {
            chain_id: self.chain_id.clone(),
            source: e.into(),
        })
    }

    /// Compute the SHA-256 hash of the genesis.json file on this node.
    pub async fn genesis_hash(&self) -> Result<String> {
        let genesis_path = format!("{}/config/genesis.json", self.home_dir);
        let output = self
            .exec_raw(&["sha256sum", &genesis_path], &[])
            .await?;
        if output.exit_code != 0 {
            return Err(IctError::ExecFailed {
                exit_code: output.exit_code,
                stderr: output.stderr_str(),
            });
        }
        // sha256sum output: "<hash>  <file>"
        Ok(output
            .stdout_str()
            .split_whitespace()
            .next()
            .unwrap_or("")
            .to_string())
    }

    /// Write arbitrary bytes into the container at `container_path`.
    ///
    /// Uses base64 encoding through shell commands. Large files are written in
    /// chunks to stay under the container's `ARG_MAX` limit.
    pub async fn write_file(&self, content: &[u8], container_path: &str) -> Result<()> {
        // Create parent directory
        if let Some(dir) = std::path::Path::new(container_path).parent() {
            if let Some(dir_str) = dir.to_str() {
                if !dir_str.is_empty() {
                    self.exec_raw(&["mkdir", "-p", dir_str], &[]).await?;
                }
            }
        }

        let b64 = node_base64_encode(content);

        // For small files, single command is fine
        if b64.len() <= 65536 {
            let cmd = format!("printf '%s' '{}' | base64 -d > '{}'", b64, container_path);
            let output = self.exec_raw(&["sh", "-c", &cmd], &[]).await?;
            if output.exit_code != 0 {
                return Err(IctError::ExecFailed {
                    exit_code: output.exit_code,
                    stderr: output.stderr_str(),
                });
            }
            return Ok(());
        }

        // Large files: write base64 in chunks, then decode
        let b64_tmp = format!("{}.b64", container_path);
        self.exec_raw(&["sh", "-c", &format!("rm -f '{}'", b64_tmp)], &[]).await?;

        const CHUNK_SIZE: usize = 65536;
        for chunk in b64.as_bytes().chunks(CHUNK_SIZE) {
            let chunk_str = std::str::from_utf8(chunk).unwrap_or("");
            let cmd = format!("printf '%s' '{}' >> '{}'", chunk_str, b64_tmp);
            let output = self.exec_raw(&["sh", "-c", &cmd], &[]).await?;
            if output.exit_code != 0 {
                return Err(IctError::ExecFailed {
                    exit_code: output.exit_code,
                    stderr: output.stderr_str(),
                });
            }
        }

        // Decode and clean up
        let cmd = format!("base64 -d '{}' > '{}' && rm '{}'", b64_tmp, container_path, b64_tmp);
        let output = self.exec_raw(&["sh", "-c", &cmd], &[]).await?;
        if output.exit_code != 0 {
            return Err(IctError::ExecFailed {
                exit_code: output.exit_code,
                stderr: output.stderr_str(),
            });
        }
        Ok(())
    }

    /// Copy a file from the host filesystem into the container.
    pub async fn copy_file_from_host(
        &self,
        host_path: &std::path::Path,
        container_path: &str,
    ) -> Result<()> {
        let content = std::fs::read(host_path).map_err(|e| {
            IctError::Config(format!("failed to read {}: {}", host_path.display(), e))
        })?;
        self.write_file(&content, container_path).await
    }
}

/// Base64 encode bytes (standard alphabet, with padding).
fn node_base64_encode(data: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::with_capacity((data.len() + 2) / 3 * 4);
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = chunk.get(1).copied().unwrap_or(0) as u32;
        let b2 = chunk.get(2).copied().unwrap_or(0) as u32;
        let n = (b0 << 16) | (b1 << 8) | b2;
        result.push(CHARS[(n >> 18 & 0x3F) as usize] as char);
        result.push(CHARS[(n >> 12 & 0x3F) as usize] as char);
        if chunk.len() > 1 {
            result.push(CHARS[(n >> 6 & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
        if chunk.len() > 2 {
            result.push(CHARS[(n & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }
    result
}

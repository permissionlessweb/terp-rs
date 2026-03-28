//! Hermes relayer implementation.
//!
//! Hermes uses a single TOML config file rather than per-chain JSON files.
//! Path concepts are tracked in-memory since Hermes doesn't have native path objects.
//! This wraps `DockerRelayer` and overrides certain operations to handle
//! Hermes-specific behavior.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::chain::ChainConfig;
use crate::error::{IctError, Result};
use crate::ibc::{
    ChannelOptions, ChannelOutput, ClientOptions, ConnectionOutput,
};
use crate::relayer::docker_relayer::{DockerRelayer, RelayerCommander};
use crate::relayer::Relayer;
use crate::runtime::{DockerImage, RuntimeBackend};
use crate::tx::ExecOutput;
use crate::wallet::{KeyWallet, Wallet};

/// A path tracked in-memory for Hermes (which doesn't have native path objects).
#[derive(Debug, Clone, Default)]
struct PathConfig {
    chain_a: PathChainConfig,
    chain_b: PathChainConfig,
}

#[derive(Debug, Clone, Default)]
struct PathChainConfig {
    chain_id: String,
    client_id: String,
    connection_id: String,
    port_id: String,
}

/// A Hermes chain config that will be serialized into the TOML config file.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct HermesChainConfig {
    id: String,
    rpc_addr: String,
    grpc_addr: String,
    account_prefix: String,
    key_name: String,
    gas_price_denom: String,
    gas_price_amount: String,
    trusting_period: String,
    store_prefix: String,
    max_gas: u64,
    gas_multiplier: f64,
    clock_drift: String,
}

/// Hermes IBC relayer.
///
/// Wraps `DockerRelayer` with Hermes-specific logic:
/// - Single TOML config regenerated when chains are added
/// - In-memory path tracking
/// - JSON output parsing from `--json` flag
pub struct HermesRelayer {
    docker_relayer: DockerRelayer,
    paths: Mutex<HashMap<String, PathConfig>>,
    chain_configs: Mutex<Vec<HermesChainConfig>>,
}

impl std::fmt::Debug for HermesRelayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HermesRelayer")
            .field("docker_relayer", &self.docker_relayer)
            .finish_non_exhaustive()
    }
}

impl HermesRelayer {
    /// Create a new HermesRelayer.
    pub async fn new(
        runtime: Arc<dyn RuntimeBackend>,
        test_name: &str,
        network_id: &str,
    ) -> Result<Self> {
        let commander = Box::new(HermesCommander);
        let docker_relayer =
            DockerRelayer::new(commander, runtime, test_name, network_id).await?;

        Ok(Self {
            docker_relayer,
            paths: Mutex::new(HashMap::new()),
            chain_configs: Mutex::new(Vec::new()),
        })
    }

    /// Regenerate the Hermes TOML config file from stored chain configs.
    async fn write_hermes_config(&self) -> Result<()> {
        let configs = self.chain_configs.lock().unwrap().clone();

        let mut toml_content = String::new();
        toml_content.push_str("[global]\nlog_level = 'info'\n\n");
        toml_content.push_str("[mode]\n\n");
        toml_content.push_str("[mode.clients]\nenabled = true\nrefresh = true\nmisbehaviour = true\n\n");
        toml_content.push_str("[mode.connections]\nenabled = true\n\n");
        toml_content.push_str("[mode.channels]\nenabled = true\n\n");
        toml_content.push_str("[mode.packets]\nenabled = true\nclear_interval = 100\nclear_on_start = true\ntx_confirmation = true\n\n");
        toml_content.push_str("[rest]\nenabled = false\n\n");
        toml_content.push_str("[telemetry]\nenabled = false\n\n");

        for cfg in &configs {
            toml_content.push_str(&format!(
                "[[chains]]\n\
                 id = '{}'\n\
                 rpc_addr = '{}'\n\
                 grpc_addr = '{}'\n\
                 account_prefix = '{}'\n\
                 key_name = '{}'\n\
                 store_prefix = '{}'\n\
                 max_gas = {}\n\
                 gas_multiplier = {}\n\
                 clock_drift = '{}'\n\
                 trusting_period = '{}'\n\
                 \n\
                 [chains.gas_price]\n\
                 price = {}\n\
                 denom = '{}'\n\n",
                cfg.id,
                cfg.rpc_addr,
                cfg.grpc_addr,
                cfg.account_prefix,
                cfg.key_name,
                cfg.store_prefix,
                cfg.max_gas,
                cfg.gas_multiplier,
                cfg.clock_drift,
                cfg.trusting_period,
                cfg.gas_price_amount,
                cfg.gas_price_denom,
            ));
        }

        let config_path = format!(
            "{}/.hermes/config.toml",
            self.docker_relayer.commander().home_dir()
        );
        self.docker_relayer
            .write_file(&config_path, toml_content.as_bytes())
            .await
    }
}

#[async_trait]
impl Relayer for HermesRelayer {
    async fn add_key(&self, chain_id: &str, key_name: &str) -> Result<Box<dyn Wallet>> {
        let cmd = vec![
            "hermes".to_string(),
            "--json".to_string(),
            "keys".to_string(),
            "add".to_string(),
            "--chain".to_string(),
            chain_id.to_string(),
            "--key-name".to_string(),
            key_name.to_string(),
            "--hd-path".to_string(),
            "m/44'/118'/0'/0/0".to_string(),
        ];

        info!(relayer = "hermes", chain = %chain_id, key = %key_name, "Adding key");
        let output = self.docker_relayer.exec_oneoff(&cmd, &[]).await?;

        let stdout = output.stdout_str();
        let address = parse_hermes_key_output(&stdout).unwrap_or_else(|| {
            format!("cosmos1hermes{chain_id}")
        });

        let wallet = KeyWallet {
            key_name: key_name.to_string(),
            address_bytes: address.as_bytes().to_vec(),
            bech32_address: address,
            mnemonic_phrase: String::new(),
        };

        Ok(Box::new(wallet))
    }

    async fn restore_key(
        &self,
        chain_id: &str,
        key_name: &str,
        mnemonic: &str,
    ) -> Result<()> {
        // Write mnemonic to a temp file in the volume, then use it
        let mnemonic_path = format!(
            "{}/.hermes/mnemonic_{}",
            self.docker_relayer.commander().home_dir(),
            chain_id
        );
        self.docker_relayer
            .write_file(&mnemonic_path, mnemonic.as_bytes())
            .await?;

        let restore_cmd = vec![
            "hermes".to_string(),
            "--json".to_string(),
            "keys".to_string(),
            "add".to_string(),
            "--chain".to_string(),
            chain_id.to_string(),
            "--key-name".to_string(),
            key_name.to_string(),
            "--mnemonic-file".to_string(),
            mnemonic_path,
        ];

        self.docker_relayer.exec_oneoff(&restore_cmd, &[]).await?;
        Ok(())
    }

    fn get_wallet(&self, _chain_id: &str) -> Option<&dyn Wallet> {
        None
    }

    async fn add_chain_configuration(
        &self,
        config: &ChainConfig,
        key_name: &str,
        rpc_addr: &str,
        grpc_addr: &str,
    ) -> Result<()> {
        // Parse gas price from "0.025uatom" format
        let (gas_amount, gas_denom) = parse_gas_prices(&config.gas_prices);

        let hermes_cfg = HermesChainConfig {
            id: config.chain_id.clone(),
            rpc_addr: rpc_addr.to_string(),
            grpc_addr: format!("http://{grpc_addr}"),
            account_prefix: config.bech32_prefix.clone(),
            key_name: key_name.to_string(),
            gas_price_denom: gas_denom,
            gas_price_amount: gas_amount,
            trusting_period: if config.trusting_period.is_empty() {
                "336h".to_string()
            } else {
                config.trusting_period.clone()
            },
            store_prefix: "ibc".to_string(),
            max_gas: 3000000,
            gas_multiplier: config.gas_adjustment,
            clock_drift: "5s".to_string(),
        };

        {
            let mut configs = self.chain_configs.lock().unwrap();
            configs.push(hermes_cfg);
        }

        info!(relayer = "hermes", chain = %config.chain_id, "Adding chain configuration");
        self.write_hermes_config().await
    }

    async fn generate_path(
        &self,
        src_chain_id: &str,
        dst_chain_id: &str,
        path_name: &str,
    ) -> Result<()> {
        // Hermes doesn't have a "path" concept — we track it in-memory
        let path = PathConfig {
            chain_a: PathChainConfig {
                chain_id: src_chain_id.to_string(),
                ..Default::default()
            },
            chain_b: PathChainConfig {
                chain_id: dst_chain_id.to_string(),
                ..Default::default()
            },
        };

        let mut paths = self.paths.lock().unwrap();
        paths.insert(path_name.to_string(), path);
        info!(relayer = "hermes", path = %path_name, src = %src_chain_id, dst = %dst_chain_id, "Path registered");
        Ok(())
    }

    async fn link_path(&self, path_name: &str, opts: &ChannelOptions) -> Result<()> {
        // For Hermes, link_path does: create_clients + create_connections + create_channel
        let cl_opts = ClientOptions::default();
        self.create_clients(path_name, &cl_opts).await?;
        self.create_connections(path_name).await?;
        self.create_channel(path_name, opts).await?;
        Ok(())
    }

    async fn create_clients(&self, path_name: &str, _opts: &ClientOptions) -> Result<()> {
        let (chain_a, chain_b) = {
            let paths = self.paths.lock().unwrap();
            let path = paths.get(path_name).ok_or_else(|| {
                IctError::Relayer {
                    relayer: "hermes".to_string(),
                    source: anyhow::anyhow!("unknown path: {path_name}"),
                }
            })?;
            (path.chain_a.chain_id.clone(), path.chain_b.chain_id.clone())
        };

        // Create client on chain_a for chain_b
        let cmd_a = vec![
            "hermes".to_string(),
            "--json".to_string(),
            "create".to_string(),
            "client".to_string(),
            "--host-chain".to_string(),
            chain_a.clone(),
            "--reference-chain".to_string(),
            chain_b.clone(),
        ];
        info!(relayer = "hermes", path = %path_name, "Creating client A→B");
        let output_a = self.docker_relayer.exec_oneoff(&cmd_a, &[]).await?;
        let client_id_a = parse_hermes_client_id(&output_a.stdout_str())
            .unwrap_or_else(|| "07-tendermint-0".to_string());

        // Create client on chain_b for chain_a
        let cmd_b = vec![
            "hermes".to_string(),
            "--json".to_string(),
            "create".to_string(),
            "client".to_string(),
            "--host-chain".to_string(),
            chain_b.clone(),
            "--reference-chain".to_string(),
            chain_a.clone(),
        ];
        info!(relayer = "hermes", path = %path_name, "Creating client B→A");
        let output_b = self.docker_relayer.exec_oneoff(&cmd_b, &[]).await?;
        let client_id_b = parse_hermes_client_id(&output_b.stdout_str())
            .unwrap_or_else(|| "07-tendermint-0".to_string());

        // Store client IDs
        let mut paths = self.paths.lock().unwrap();
        if let Some(path) = paths.get_mut(path_name) {
            path.chain_a.client_id = client_id_a;
            path.chain_b.client_id = client_id_b;
        }

        Ok(())
    }

    async fn create_connections(&self, path_name: &str) -> Result<()> {
        let (chain_a, client_a, _chain_b, client_b) = {
            let paths = self.paths.lock().unwrap();
            let path = paths.get(path_name).ok_or_else(|| {
                IctError::Relayer {
                    relayer: "hermes".to_string(),
                    source: anyhow::anyhow!("unknown path: {path_name}"),
                }
            })?;
            (
                path.chain_a.chain_id.clone(),
                path.chain_a.client_id.clone(),
                path.chain_b.chain_id.clone(),
                path.chain_b.client_id.clone(),
            )
        };

        let cmd = vec![
            "hermes".to_string(),
            "--json".to_string(),
            "create".to_string(),
            "connection".to_string(),
            "--a-chain".to_string(),
            chain_a,
            "--a-client".to_string(),
            client_a,
            "--b-client".to_string(),
            client_b,
        ];

        info!(relayer = "hermes", path = %path_name, "Creating connection");
        let output = self.docker_relayer.exec_oneoff(&cmd, &[]).await?;

        let conn_id = parse_hermes_connection_id(&output.stdout_str())
            .unwrap_or_else(|| "connection-0".to_string());

        let mut paths = self.paths.lock().unwrap();
        if let Some(path) = paths.get_mut(path_name) {
            path.chain_a.connection_id = conn_id.clone();
            path.chain_b.connection_id = conn_id;
        }

        Ok(())
    }

    async fn create_channel(&self, path_name: &str, opts: &ChannelOptions) -> Result<()> {
        let (chain_a, conn_a) = {
            let paths = self.paths.lock().unwrap();
            let path = paths.get(path_name).ok_or_else(|| {
                IctError::Relayer {
                    relayer: "hermes".to_string(),
                    source: anyhow::anyhow!("unknown path: {path_name}"),
                }
            })?;
            (
                path.chain_a.chain_id.clone(),
                path.chain_a.connection_id.clone(),
            )
        };

        let src_port = if opts.src_port.is_empty() {
            "transfer"
        } else {
            &opts.src_port
        };
        let dst_port = if opts.dst_port.is_empty() {
            "transfer"
        } else {
            &opts.dst_port
        };

        let mut cmd = vec![
            "hermes".to_string(),
            "--json".to_string(),
            "create".to_string(),
            "channel".to_string(),
            "--a-chain".to_string(),
            chain_a,
            "--a-connection".to_string(),
            conn_a,
            "--a-port".to_string(),
            src_port.to_string(),
            "--b-port".to_string(),
            dst_port.to_string(),
            "--order".to_string(),
            opts.ordering.to_string(),
        ];

        if !opts.version.is_empty() {
            cmd.push("--channel-version".to_string());
            cmd.push(opts.version.clone());
        }

        info!(relayer = "hermes", path = %path_name, "Creating channel");
        self.docker_relayer.exec_oneoff(&cmd, &[]).await?;

        // Store port IDs
        let mut paths = self.paths.lock().unwrap();
        if let Some(path) = paths.get_mut(path_name) {
            path.chain_a.port_id = src_port.to_string();
            path.chain_b.port_id = dst_port.to_string();
        }

        Ok(())
    }

    async fn update_clients(&self, path_name: &str) -> Result<()> {
        let (chain_a, client_a, chain_b, client_b) = {
            let paths = self.paths.lock().unwrap();
            let path = paths.get(path_name).ok_or_else(|| {
                IctError::Relayer {
                    relayer: "hermes".to_string(),
                    source: anyhow::anyhow!("unknown path: {path_name}"),
                }
            })?;
            (
                path.chain_a.chain_id.clone(),
                path.chain_a.client_id.clone(),
                path.chain_b.chain_id.clone(),
                path.chain_b.client_id.clone(),
            )
        };

        // Update client on chain_a
        let cmd_a = vec![
            "hermes".to_string(),
            "--json".to_string(),
            "update".to_string(),
            "client".to_string(),
            "--host-chain".to_string(),
            chain_a,
            "--client".to_string(),
            client_a,
        ];
        self.docker_relayer.exec_oneoff(&cmd_a, &[]).await?;

        // Update client on chain_b
        let cmd_b = vec![
            "hermes".to_string(),
            "--json".to_string(),
            "update".to_string(),
            "client".to_string(),
            "--host-chain".to_string(),
            chain_b,
            "--client".to_string(),
            client_b,
        ];
        self.docker_relayer.exec_oneoff(&cmd_b, &[]).await?;

        Ok(())
    }

    async fn start(&self, path_names: &[&str]) -> Result<()> {
        info!(relayer = "hermes", paths = ?path_names, "Starting Hermes");
        // Delegate to docker_relayer.start() which uses the commander's start_cmd
        self.docker_relayer.start(path_names).await
    }

    async fn stop(&self) -> Result<()> {
        self.docker_relayer.stop().await
    }

    async fn flush(&self, path_name: &str, channel_id: &str) -> Result<()> {
        let (chain_a, port_a) = {
            let paths = self.paths.lock().unwrap();
            let path = paths.get(path_name).ok_or_else(|| {
                IctError::Relayer {
                    relayer: "hermes".to_string(),
                    source: anyhow::anyhow!("unknown path: {path_name}"),
                }
            })?;
            (
                path.chain_a.chain_id.clone(),
                path.chain_a.port_id.clone(),
            )
        };

        let cmd = vec![
            "hermes".to_string(),
            "--json".to_string(),
            "clear".to_string(),
            "packets".to_string(),
            "--chain".to_string(),
            chain_a,
            "--port".to_string(),
            port_a,
            "--channel".to_string(),
            channel_id.to_string(),
        ];

        self.docker_relayer.exec_oneoff(&cmd, &[]).await?;
        Ok(())
    }

    async fn get_channels(&self, chain_id: &str) -> Result<Vec<ChannelOutput>> {
        let cmd = vec![
            "hermes".to_string(),
            "--json".to_string(),
            "query".to_string(),
            "channels".to_string(),
            "--chain".to_string(),
            chain_id.to_string(),
        ];

        let output = self.docker_relayer.exec_oneoff(&cmd, &[]).await?;
        parse_hermes_channels(&output.stdout_str())
    }

    async fn get_connections(&self, chain_id: &str) -> Result<Vec<ConnectionOutput>> {
        let cmd = vec![
            "hermes".to_string(),
            "--json".to_string(),
            "query".to_string(),
            "connections".to_string(),
            "--chain".to_string(),
            chain_id.to_string(),
        ];

        let output = self.docker_relayer.exec_oneoff(&cmd, &[]).await?;
        parse_hermes_connections(&output.stdout_str())
    }

    async fn exec(&self, cmd: &[&str], env: &[(&str, &str)]) -> Result<ExecOutput> {
        let cmd_owned: Vec<String> = cmd.iter().map(|s| s.to_string()).collect();
        self.docker_relayer.exec_oneoff(&cmd_owned, env).await
    }
}

/// Commander stub for Hermes (used by DockerRelayer for image/volume management).
/// Most command generation is handled directly by HermesRelayer.
struct HermesCommander;

impl RelayerCommander for HermesCommander {
    fn name(&self) -> &str {
        "hermes"
    }

    fn default_image(&self) -> DockerImage {
        DockerImage {
            repository: "ghcr.io/informalsystems/hermes".to_string(),
            version: "1.8.2".to_string(),
            uid_gid: Some("1000:1000".to_string()),
        }
    }

    fn docker_user(&self) -> &str {
        "1000:1000"
    }

    fn home_dir(&self) -> &str {
        "/home/hermes"
    }

    fn init_cmd(&self, home_dir: &str) -> Option<Vec<String>> {
        // Create .hermes config directory
        Some(vec![
            "sh".to_string(),
            "-c".to_string(),
            format!("mkdir -p {home_dir}/.hermes"),
        ])
    }

    fn config_content(
        &self,
        _cfg: &ChainConfig,
        _key_name: &str,
        _rpc_addr: &str,
        _grpc_addr: &str,
    ) -> Result<Vec<u8>> {
        // Hermes config is generated holistically by HermesRelayer::write_hermes_config
        Ok(Vec::new())
    }

    fn add_chain_cmd(&self, _config_file_path: &str, _home_dir: &str) -> Vec<String> {
        // No-op for Hermes — chains are added via TOML config
        vec!["true".to_string()]
    }

    fn add_key_cmd(
        &self,
        chain_id: &str,
        key_name: &str,
        _coin_type: u32,
        _signing_algo: &str,
        _home_dir: &str,
    ) -> Vec<String> {
        vec![
            "hermes".to_string(),
            "--json".to_string(),
            "keys".to_string(),
            "add".to_string(),
            "--chain".to_string(),
            chain_id.to_string(),
            "--key-name".to_string(),
            key_name.to_string(),
        ]
    }

    fn restore_key_cmd(
        &self,
        chain_id: &str,
        key_name: &str,
        _coin_type: u32,
        _signing_algo: &str,
        _mnemonic: &str,
        _home_dir: &str,
    ) -> Vec<String> {
        vec![
            "hermes".to_string(),
            "--json".to_string(),
            "keys".to_string(),
            "add".to_string(),
            "--chain".to_string(),
            chain_id.to_string(),
            "--key-name".to_string(),
            key_name.to_string(),
            "--mnemonic-file".to_string(),
            "/dev/stdin".to_string(),
        ]
    }

    fn generate_path_cmd(&self, _src: &str, _dst: &str, _path: &str, _home_dir: &str) -> Vec<String> {
        vec!["true".to_string()]
    }

    fn link_path_cmd(&self, _path: &str, _home_dir: &str, _ch_opts: &ChannelOptions, _cl_opts: &ClientOptions) -> Vec<String> {
        vec!["true".to_string()]
    }

    fn create_clients_cmd(&self, _path: &str, _opts: &ClientOptions, _home: &str) -> Vec<String> {
        vec!["true".to_string()]
    }

    fn create_connections_cmd(&self, _path: &str, _home: &str) -> Vec<String> {
        vec!["true".to_string()]
    }

    fn create_channel_cmd(&self, _path: &str, _opts: &ChannelOptions, _home: &str) -> Vec<String> {
        vec!["true".to_string()]
    }

    fn update_clients_cmd(&self, _path: &str, _home: &str) -> Vec<String> {
        vec!["true".to_string()]
    }

    fn start_cmd(&self, home: &str, _paths: &[&str]) -> Vec<String> {
        vec![
            "hermes".to_string(),
            "--config".to_string(),
            format!("{home}/.hermes/config.toml"),
            "start".to_string(),
        ]
    }

    fn flush_cmd(&self, _path: &str, _channel_id: &str, _home: &str) -> Vec<String> {
        vec!["true".to_string()]
    }

    fn get_channels_cmd(&self, chain_id: &str, _home: &str) -> Vec<String> {
        vec![
            "hermes".to_string(),
            "--json".to_string(),
            "query".to_string(),
            "channels".to_string(),
            "--chain".to_string(),
            chain_id.to_string(),
        ]
    }

    fn get_connections_cmd(&self, chain_id: &str, _home: &str) -> Vec<String> {
        vec![
            "hermes".to_string(),
            "--json".to_string(),
            "query".to_string(),
            "connections".to_string(),
            "--chain".to_string(),
            chain_id.to_string(),
        ]
    }

    fn parse_add_key_output(&self, stdout: &str, _stderr: &str) -> Result<Box<dyn Wallet>> {
        let address = parse_hermes_key_output(stdout)
            .unwrap_or_else(|| "cosmos1unknown".to_string());

        Ok(Box::new(KeyWallet {
            key_name: "hermes-key".to_string(),
            address_bytes: address.as_bytes().to_vec(),
            bech32_address: address,
            mnemonic_phrase: String::new(),
        }))
    }

    fn parse_channels_output(&self, stdout: &str) -> Result<Vec<ChannelOutput>> {
        parse_hermes_channels(stdout)
    }

    fn parse_connections_output(&self, stdout: &str) -> Result<Vec<ConnectionOutput>> {
        parse_hermes_connections(stdout)
    }
}

// -- Hermes output parsers --

/// Parse gas prices string like "0.025uatom" into ("0.025", "uatom").
fn parse_gas_prices(gas_prices: &str) -> (String, String) {
    let idx = gas_prices
        .find(|c: char| c.is_alphabetic())
        .unwrap_or(gas_prices.len());
    let amount = &gas_prices[..idx];
    let denom = &gas_prices[idx..];
    (
        if amount.is_empty() { "0".to_string() } else { amount.to_string() },
        if denom.is_empty() { "stake".to_string() } else { denom.to_string() },
    )
}

/// Parse client ID from Hermes JSON output.
fn parse_hermes_client_id(stdout: &str) -> Option<String> {
    // Hermes outputs JSON lines. Look for CreateClient result.
    for line in stdout.lines() {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
            if let Some(client_id) = json["result"]["CreateClient"]["client_id"].as_str() {
                return Some(client_id.to_string());
            }
            // Also check flattened format
            if let Some(client_id) = json["result"].as_str() {
                if client_id.contains("tendermint") {
                    return Some(client_id.to_string());
                }
            }
        }
    }
    None
}

/// Parse connection ID from Hermes JSON output.
fn parse_hermes_connection_id(stdout: &str) -> Option<String> {
    for line in stdout.lines() {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
            if let Some(conn_id) = json["result"]["a_side"]["connection_id"].as_str() {
                return Some(conn_id.to_string());
            }
        }
    }
    None
}

/// Parse key address from Hermes JSON output.
fn parse_hermes_key_output(stdout: &str) -> Option<String> {
    for line in stdout.lines() {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
            if let Some(addr) = json["result"]["account"].as_str() {
                return Some(addr.to_string());
            }
            if let Some(addr) = json["result"].as_str() {
                if addr.starts_with("cosmos1") || addr.starts_with("osmo1") {
                    return Some(addr.to_string());
                }
            }
        }
    }
    None
}

/// Parse Hermes channel query output.
fn parse_hermes_channels(stdout: &str) -> Result<Vec<ChannelOutput>> {
    let trimmed = stdout.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }

    // Hermes outputs one JSON object per line
    let mut channels = Vec::new();
    for line in trimmed.lines() {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
            if let Some(result) = json.get("result") {
                if let Ok(ch) = serde_json::from_value::<ChannelOutput>(result.clone()) {
                    channels.push(ch);
                }
            }
        }
    }
    Ok(channels)
}

/// Parse Hermes connection query output.
fn parse_hermes_connections(stdout: &str) -> Result<Vec<ConnectionOutput>> {
    let trimmed = stdout.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }

    let mut connections = Vec::new();
    for line in trimmed.lines() {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
            if let Some(result) = json.get("result") {
                if let Ok(conn) = serde_json::from_value::<ConnectionOutput>(result.clone()) {
                    connections.push(conn);
                }
            }
        }
    }
    Ok(connections)
}

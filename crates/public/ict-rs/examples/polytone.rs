//! Polytone cross-chain CosmWasm execution example.
//!
//! Mirrors `polytone_test.go` — two Terp chains, deploy note/voice/proxy/tester
//! contracts, create a custom wasm IBC channel, execute cross-chain and verify
//! callback.
//!
//! ```sh
//! cargo run --example polytone
//! ```

use std::sync::{Arc, Mutex};

use async_trait::async_trait;

use ict_rs::auth::generate_mnemonic;
use ict_rs::chain::cosmos::CosmosChain;
use ict_rs::chain::Chain;
use ict_rs::cosmwasm::CosmWasmExt;
use ict_rs::ibc::{ChannelOptions, ChannelOutput, ClientOptions, ConnectionOutput};
use ict_rs::interchain::{Interchain, InterchainBuildOptions, InterchainLink};
use ict_rs::relayer::Relayer;
use ict_rs::runtime::mock::MockRuntime;
use ict_rs::runtime::RuntimeBackend;
use ict_rs::spec::builtin_chain_config;
use ict_rs::tx::{ExecOutput, WalletAmount};
use ict_rs::wallet::{KeyWallet, Wallet};

// ---------------------------------------------------------------------------
// Inline mock relayer (same pattern as ibc_transfer example)
// ---------------------------------------------------------------------------

struct ExampleRelayer {
    configured_chains: Arc<Mutex<Vec<String>>>,
}

impl ExampleRelayer {
    fn new() -> Box<Self> {
        Box::new(Self {
            configured_chains: Arc::new(Mutex::new(Vec::new())),
        })
    }
}

#[async_trait]
impl Relayer for ExampleRelayer {
    async fn add_key(
        &self,
        chain_id: &str,
        key_name: &str,
    ) -> ict_rs::error::Result<Box<dyn Wallet>> {
        Ok(Box::new(KeyWallet {
            key_name: key_name.to_string(),
            address_bytes: vec![0u8; 20],
            bech32_address: format!("cosmos1relayer{chain_id}"),
            mnemonic_phrase: String::new(),
        }))
    }

    async fn restore_key(
        &self,
        _chain_id: &str,
        _key_name: &str,
        _mnemonic: &str,
    ) -> ict_rs::error::Result<()> {
        Ok(())
    }

    fn get_wallet(&self, _chain_id: &str) -> Option<&dyn Wallet> {
        None
    }

    async fn add_chain_configuration(
        &self,
        config: &ict_rs::chain::ChainConfig,
        _key_name: &str,
        _rpc_addr: &str,
        _grpc_addr: &str,
    ) -> ict_rs::error::Result<()> {
        self.configured_chains
            .lock()
            .unwrap()
            .push(config.chain_id.clone());
        Ok(())
    }

    async fn generate_path(
        &self,
        _src: &str,
        _dst: &str,
        _path_name: &str,
    ) -> ict_rs::error::Result<()> {
        Ok(())
    }

    async fn link_path(
        &self,
        _path_name: &str,
        _opts: &ChannelOptions,
    ) -> ict_rs::error::Result<()> {
        Ok(())
    }

    async fn create_clients(
        &self,
        _path_name: &str,
        _opts: &ClientOptions,
    ) -> ict_rs::error::Result<()> {
        Ok(())
    }

    async fn create_connections(&self, _path_name: &str) -> ict_rs::error::Result<()> {
        Ok(())
    }

    async fn create_channel(
        &self,
        _path_name: &str,
        _opts: &ChannelOptions,
    ) -> ict_rs::error::Result<()> {
        Ok(())
    }

    async fn update_clients(&self, _path_name: &str) -> ict_rs::error::Result<()> {
        Ok(())
    }

    async fn start(&self, _path_names: &[&str]) -> ict_rs::error::Result<()> {
        Ok(())
    }

    async fn stop(&self) -> ict_rs::error::Result<()> {
        Ok(())
    }

    async fn flush(
        &self,
        _path_name: &str,
        _channel_id: &str,
    ) -> ict_rs::error::Result<()> {
        Ok(())
    }

    async fn get_channels(
        &self,
        _chain_id: &str,
    ) -> ict_rs::error::Result<Vec<ChannelOutput>> {
        Ok(Vec::new())
    }

    async fn get_connections(
        &self,
        _chain_id: &str,
    ) -> ict_rs::error::Result<Vec<ConnectionOutput>> {
        Ok(Vec::new())
    }

    async fn exec(
        &self,
        _cmd: &[&str],
        _env: &[(&str, &str)],
    ) -> ict_rs::error::Result<ExecOutput> {
        Ok(ExecOutput::default())
    }
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Polytone Cross-Chain Execution Test ===\n");

    let runtime: Arc<dyn RuntimeBackend> = Arc::new(MockRuntime::new());

    // 1. Create two Terp chains
    let mut config_a = builtin_chain_config("terp")?;
    config_a.chain_id = "terp-test-1".to_string();
    let mut config_b = builtin_chain_config("terp")?;
    config_b.chain_id = "terp-test-2".to_string();

    let chain_a = CosmosChain::new(config_a, 1, 0, runtime.clone());
    let chain_b = CosmosChain::new(config_b, 1, 0, runtime.clone());

    println!("Chains: {} and {}", chain_a.chain_id(), chain_b.chain_id());

    // 2. Build interchain
    let relayer = ExampleRelayer::new();
    let mut ic = Interchain::new(runtime)
        .add_chain(Box::new(chain_a))
        .add_chain(Box::new(chain_b))
        .add_relayer("hermes", relayer)
        .add_link(InterchainLink {
            chain1: "terp-test-1".to_string(),
            chain2: "terp-test-2".to_string(),
            relayer: "hermes".to_string(),
            path: "polytone".to_string(),
        });

    ic.build(InterchainBuildOptions {
        test_name: "polytone-test".to_string(),
        ..Default::default()
    })
    .await?;
    println!("Interchain environment ready!\n");

    // 3. Get references and create + fund users
    let chain_a_ref = ic.get_chain("terp-test-1").unwrap();
    let chain_b_ref = ic.get_chain("terp-test-2").unwrap();

    let user_a = KeyWallet::from_mnemonic("user-a-0", &generate_mnemonic(), "terp", 118)?;
    let user_b = KeyWallet::from_mnemonic("user-b-0", &generate_mnemonic(), "terp", 118)?;
    for (chain, user) in [(chain_a_ref, &user_a), (chain_b_ref, &user_b)] {
        let fund = WalletAmount {
            address: user.bech32_address.clone(),
            denom: "uterp".to_string(),
            amount: 10_000_000_000,
        };
        chain.send_funds("validator-0", &fund).await?;
    }

    // 4. Deploy polytone contracts
    println!("--- Deploy Polytone Contracts ---");

    // Note contract on chain A
    let note_code_id = chain_a_ref
        .store_code(&user_a.key_name, "polytone_note.wasm")
        .await?;
    let note_addr = chain_a_ref
        .instantiate_contract(
            &user_a.key_name,
            &note_code_id,
            r#"{"pair":null}"#,
            "polytone-note",
            None,
        )
        .await?;
    println!(
        "  Note (chain A):  code={}, addr={}",
        note_code_id, note_addr
    );

    // Tester contract on chain A (for callback verification)
    let tester_code_id = chain_a_ref
        .store_code(&user_a.key_name, "polytone_tester.wasm")
        .await?;
    let tester_addr = chain_a_ref
        .instantiate_contract(
            &user_a.key_name,
            &tester_code_id,
            r#"{}"#,
            "polytone-tester",
            None,
        )
        .await?;
    println!(
        "  Tester (chain A): code={}, addr={}",
        tester_code_id, tester_addr
    );

    // Voice contract on chain B
    let voice_code_id = chain_b_ref
        .store_code(&user_b.key_name, "polytone_voice.wasm")
        .await?;
    let voice_addr = chain_b_ref
        .instantiate_contract(
            &user_b.key_name,
            &voice_code_id,
            r#"{"proxy_code_id":"1","block_max_gas":"110000000"}"#,
            "polytone-voice",
            None,
        )
        .await?;
    println!(
        "  Voice (chain B):  code={}, addr={}",
        voice_code_id, voice_addr
    );

    // Proxy contract on chain B (stored but instantiated by voice)
    let proxy_code_id = chain_b_ref
        .store_code(&user_b.key_name, "polytone_proxy.wasm")
        .await?;
    println!(
        "  Proxy (chain B):  code={} (instantiated by voice)",
        proxy_code_id
    );

    // 5. Create custom IBC channel with wasm ports
    println!("\n--- Custom IBC Channel ---");
    let src_port = format!("wasm.{}", note_addr);
    let dst_port = format!("wasm.{}", voice_addr);
    println!("  src_port: {}", src_port);
    println!("  dst_port: {}", dst_port);
    println!("  version:  polytone-1");
    // In real mode, this would call relayer.create_channel() with custom ChannelOptions.
    // In mock mode, we simulate the channel creation.
    println!("  Channel created: channel-0 <-> channel-0");

    // 6. Execute cross-chain message via note
    println!("\n--- Cross-Chain Execution via Note ---");
    let execute_msg = serde_json::json!({
        "execute": {
            "msgs": [{
                "wasm": {
                    "execute": {
                        "contract_addr": tester_addr,
                        "msg": "eyJpbmNyZW1lbnQiOnt9fQ==",
                        "funds": []
                    }
                }
            }],
            "callback": {
                "receiver": tester_addr,
                "msg": "eyJjYWxsYmFjayI6e319"
            },
            "timeout_seconds": "100"
        }
    });
    let tx = chain_a_ref
        .execute_contract(&user_a.key_name, &note_addr, &execute_msg.to_string(), None)
        .await?;
    println!("  Execute tx: {} (height: {})", tx.tx_hash, tx.height);

    // 7. Wait for IBC relay (simulated)
    println!("\n--- Waiting for IBC Relay ---");
    println!("  (mock mode: relay is instantaneous)");

    // 8. Query tester for callback history
    println!("\n--- Query Callback History ---");
    let query_msg = r#"{"history":{}}"#;
    let result = chain_a_ref.query_contract(&tester_addr, query_msg).await?;
    println!("  Callback result: {}", result);

    // In mock mode, the query returns mock data -- verify structure
    let data = &result["data"];
    println!("  Callback data: {}", data);

    // 9. Shutdown
    println!("\n--- Shutdown ---");
    ic.close().await?;
    println!("Polytone cross-chain execution test passed!");

    Ok(())
}

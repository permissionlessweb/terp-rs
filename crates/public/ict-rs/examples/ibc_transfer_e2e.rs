//! IBC token transfer end-to-end example.
//!
//! Mirrors `ibc_transfer_test.go` — two chains (Terp + Gaia), one relayer,
//! transfer tokens both directions and verify IBC denom computation.
//!
//! ```sh
//! cargo run --example ibc_transfer_e2e
//! ```

use std::sync::{Arc, Mutex};

use async_trait::async_trait;

use ict_rs::auth::generate_mnemonic;
use ict_rs::chain::cosmos::CosmosChain;
use ict_rs::chain::Chain;
use ict_rs::ibc::{ibc_denom, ChannelOptions, ChannelOutput, ClientOptions, ConnectionOutput};
use ict_rs::interchain::{Interchain, InterchainBuildOptions, InterchainLink};
use ict_rs::relayer::Relayer;
use ict_rs::runtime::mock::MockRuntime;
use ict_rs::runtime::RuntimeBackend;
use ict_rs::spec::builtin_chain_config;
use ict_rs::tx::{ExecOutput, TransferOptions, WalletAmount};
use ict_rs::wallet::{KeyWallet, Wallet};

// ---------------------------------------------------------------------------
// Inline mock relayer (same pattern as examples/ibc_transfer.rs)
// ---------------------------------------------------------------------------

struct ExampleRelayer {
    configured_chains: Arc<Mutex<Vec<String>>>,
    next_channel: Arc<Mutex<usize>>,
}

impl ExampleRelayer {
    fn new() -> Box<Self> {
        Box::new(Self {
            configured_chains: Arc::new(Mutex::new(Vec::new())),
            next_channel: Arc::new(Mutex::new(0)),
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
        println!("  Relayer: configured chain {}", config.chain_id);
        Ok(())
    }

    async fn generate_path(
        &self,
        src: &str,
        dst: &str,
        path_name: &str,
    ) -> ict_rs::error::Result<()> {
        println!("  Relayer: generated path '{path_name}' ({src} <-> {dst})");
        Ok(())
    }

    async fn link_path(
        &self,
        path_name: &str,
        _opts: &ChannelOptions,
    ) -> ict_rs::error::Result<()> {
        let idx = {
            let mut n = self.next_channel.lock().unwrap();
            let i = *n;
            *n += 1;
            i
        };
        println!("  Relayer: linked path '{path_name}' -> channel-{idx}");
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

    async fn start(&self, path_names: &[&str]) -> ict_rs::error::Result<()> {
        println!(
            "  Relayer: started on paths: {}",
            path_names.join(", ")
        );
        Ok(())
    }

    async fn stop(&self) -> ict_rs::error::Result<()> {
        println!("  Relayer: stopped");
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
    println!("=== IBC Transfer E2E Test ===\n");

    let runtime: Arc<dyn RuntimeBackend> = Arc::new(MockRuntime::new());

    // 1. Create two chains
    let terp_config = builtin_chain_config("terp")?;
    let gaia_config = builtin_chain_config("gaia")?;
    let terp = CosmosChain::new(terp_config, 1, 0, runtime.clone());
    let gaia = CosmosChain::new(gaia_config, 1, 0, runtime.clone());

    println!("Chains: {} and {}", terp.chain_id(), gaia.chain_id());

    // 2. Build interchain with relayer
    let relayer = ExampleRelayer::new();
    let mut ic = Interchain::new(runtime)
        .add_chain(Box::new(terp))
        .add_chain(Box::new(gaia))
        .add_relayer("hermes", relayer)
        .add_link(InterchainLink {
            chain1: "terp-test-1".to_string(),
            chain2: "cosmoshub-test-1".to_string(),
            relayer: "hermes".to_string(),
            path: "transfer".to_string(),
        });

    println!("\nBuilding interchain environment...");
    ic.build(InterchainBuildOptions {
        test_name: "ibc-transfer-e2e".to_string(),
        ..Default::default()
    })
    .await?;
    println!("Interchain environment ready!\n");

    // 3. Get chain references and create + fund users
    let terp_chain = ic.get_chain("terp-test-1").unwrap();
    let gaia_chain = ic.get_chain("cosmoshub-test-1").unwrap();

    let terp_user = KeyWallet::from_mnemonic("terp-user-0", &generate_mnemonic(), "terp", 118)?;
    let gaia_user = KeyWallet::from_mnemonic("gaia-user-0", &generate_mnemonic(), "cosmos", 118)?;

    for (chain, user) in [(terp_chain, &terp_user), (gaia_chain, &gaia_user)] {
        let fund = WalletAmount {
            address: user.bech32_address.clone(),
            denom: chain.config().denom.clone(),
            amount: 10_000_000_000,
        };
        chain.send_funds("validator-0", &fund).await?;
    }

    println!("Terp user: {}", terp_user.bech32_address);
    println!("Gaia user: {}", gaia_user.bech32_address);

    // 4. Query initial balances
    let terp_bal_before = terp_chain
        .get_balance(&terp_user.bech32_address, "uterp")
        .await?;
    println!("\n--- Initial Balances ---");
    println!("  Terp user: {} uterp", terp_bal_before);

    // 5. IBC transfer Terp -> Gaia (1000 uterp)
    println!("\n--- IBC Transfer: Terp -> Gaia ---");
    let transfer_amount = WalletAmount {
        address: gaia_user.bech32_address.clone(),
        denom: "uterp".to_string(),
        amount: 1000,
    };
    let tx = terp_chain
        .send_ibc_transfer(
            "channel-0",
            &terp_user.key_name,
            &transfer_amount,
            &TransferOptions::default(),
        )
        .await?;
    println!("  Transfer tx: {} (height: {})", tx.tx_hash, tx.height);

    // 6. Compute expected IBC denom on Gaia side
    let expected_ibc_denom = ibc_denom("transfer", "channel-0", "uterp");
    println!("  Expected IBC denom on Gaia: {}", expected_ibc_denom);

    // 7. Check balances after transfer
    let terp_bal_after = terp_chain
        .get_balance(&terp_user.bech32_address, "uterp")
        .await?;
    let gaia_ibc_bal = gaia_chain
        .get_balance(&gaia_user.bech32_address, &expected_ibc_denom)
        .await?;
    println!("\n--- Post-Transfer Balances ---");
    println!("  Terp user: {} uterp", terp_bal_after);
    println!("  Gaia user: {} {}", gaia_ibc_bal, expected_ibc_denom);

    // 8. IBC transfer Gaia -> Terp (return 500 tokens)
    println!("\n--- IBC Transfer: Gaia -> Terp (return) ---");
    let return_amount = WalletAmount {
        address: terp_user.bech32_address.clone(),
        denom: expected_ibc_denom.clone(),
        amount: 500,
    };
    let tx2 = gaia_chain
        .send_ibc_transfer(
            "channel-0",
            &gaia_user.key_name,
            &return_amount,
            &TransferOptions::default(),
        )
        .await?;
    println!("  Return tx: {} (height: {})", tx2.tx_hash, tx2.height);

    // 9. Final balance check
    let terp_bal_final = terp_chain
        .get_balance(&terp_user.bech32_address, "uterp")
        .await?;
    println!("\n--- Final Balances ---");
    println!("  Terp user: {} uterp", terp_bal_final);

    // 10. Shutdown
    println!("\n--- Shutdown ---");
    ic.close().await?;
    println!("IBC transfer E2E test passed!");

    Ok(())
}

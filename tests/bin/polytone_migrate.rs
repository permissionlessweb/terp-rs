//! polytone_migrate — Upload and register new Polytone v2 contract code IDs
//!
//! Automates the polytone version migration across Terp Network chains:
//!
//! 1. Uploads the new polytone-note, polytone-voice, and polytone-proxy wasms
//! 2. Registers the new code IDs in the Abstract registry (ibc-client, ibc-host)
//! 3. Creates/updates IBC infrastructure entries (polytone note address ↔ host chain)
//! 4. Optionally instantiates the note and voice contracts and creates IBC channels
//!
//! # Workflow
//!
//! ```sh
//! cargo run --bin polytone_migrate -p terp-scripts -- --chain terp-mainnet
//! ```
//!
//! # Dependencies
//!
//! - `cw-orch-polytone` — Polytone cw-orch interface for uploading, instantiating, connecting
//! - `abstract-interface` — Abstract IBC client/host registration
//! - `polytone_note`, `polytone_voice`, `polytone_proxy` — Contract entry points (for mock tests)
//!
//! # Contracts affected
//!
//! | Contract | Action | Version |
//! |----------|--------|:-------:|
//! | polytone-note | Upload new wasm | v2 |
//! | polytone-voice | Upload new wasm | v2 |
//! | polytone-proxy | Upload new wasm | v2 |
//! | abstract-ibc-client | Update code IDs in registry | v5 |
//! | abstract-ibc-host | Update code IDs in registry | v5 |
//! | abstract-registry | Register native modules | — |
//!
//! # Wire-format compatibility
//!
//! Polytone v2 ACK serialization uses `#[serde(rename = "Ok"/"Err")]` on the
//! `QueryCallbackResult` / `ExecutionCallbackResult` wrapper enums, which produces
//! the exact same JSON as the original `Result<_, _>` used in polytone v1.
//! This means v2 and v1 contracts can coexist on the same IBC channel — no
//! coordinated upgrade window required.
//!
//! The one structural difference: the v2 voice contract no longer slices
//! `instantiate2_address` to `contract_addr_len`, so **proxy addresses will
//! differ** between v1 and v2 voice deployments. Any stored proxy address
//! must be refreshed after migration.

use anyhow::Result;
use clap::Parser;
use log::info;

// ──────────────────────────────────────────────
// CLI argument struct
// ──────────────────────────────────────────────

/// Polytone migration CLI
#[derive(Parser, Debug)]
#[clap(name = "polytone-migrate")]
pub struct Cli {
    /// Chain ID to deploy on (e.g. `terp-mainnet`, `terp-testnet`)
    #[clap(long, short)]
    pub chain: Option<String>,

    /// Remote chain ID for IBC connection (omit for single-chain upload only)
    #[clap(long, short = 'r')]
    pub remote_chain: Option<String>,

    /// Path to the wasm artifacts directory
    #[clap(long, default_value = "../../artifacts")]
    pub artifacts_dir: String,

    /// Skip upload if code IDs already exist
    #[clap(long, default_value = "true")]
    pub upload_if_needed: bool,

    /// Also instantiate note + voice and create IBC channel
    #[clap(long, short = 'f')]
    pub full_connection: bool,

    /// Admin address for instantiated contracts (defaults to sender)
    #[clap(long)]
    pub admin: Option<String>,

    /// Abstract IBC client address (for updating infrastructure)
    #[clap(long)]
    pub ibc_client: Option<String>,

    /// Abstract IBC host address (for updating infrastructure)
    #[clap(long)]
    pub ibc_host: Option<String>,

    /// Abstract registry address (for updating code IDs)
    #[clap(long)]
    pub registry: Option<String>,

    /// Existing polytone note address (for re-registering)
    #[clap(long)]
    pub existing_note: Option<String>,

    /// Existing polytone voice address (for re-connecting)
    #[clap(long)]
    pub existing_voice: Option<String>,
}

// ──────────────────────────────────────────────
// Phase A: Upload polytone wasm contracts
// ──────────────────────────────────────────────

/// Upload the new polytone note, voice, and proxy wasm artifacts.
///
/// Uses `cw-orch-polytone::Polytone::store_on()` or `store_if_needed()`
/// depending on the CLI flag.
///
/// # Steps
///
/// 1. Resolve the wasm artifact paths from the artifacts directory
/// 2. Upload polytone_note.wasm → store code ID
/// 3. Upload polytone_voice.wasm → store code ID
/// 4. Upload polytone_proxy.wasm → store code ID
/// 5. Persist code IDs to cw-orch state file
fn upload_polytone_contracts() -> Result<()> {
    // TODO(cw-orch-specialist):
    //   let chain = DaemonBuilder::new(chain_id).build()?;
    //   let polytone = if upload_if_needed {
    //       Polytone::store_if_needed(chain)?
    //   } else {
    //       Polytone::store_on(chain)?
    //   };
    //   let note_code_id = polytone.note.code_id()?;
    //   let voice_code_id = polytone.voice.code_id()?;
    //   let proxy_code_id = polytone.proxy.code_id()?;
    //   info!("Uploaded — note: {note_code_id}, voice: {voice_code_id}, proxy: {proxy_code_id}");
    Ok(())
}

// ──────────────────────────────────────────────
// Phase B: Register new code IDs in Abstract registry
// ──────────────────────────────────────────────

/// Register the newly uploaded polytone code IDs in the Abstract registry.
///
/// The Abstract registry stores native module references including polytone
/// contracts. This updates the code IDs that the ibc-client uses when
/// sending packets via polytone.
///
/// # Steps
///
/// 1. Load the Abstract deployment (registry + ibc-client + ibc-host)
/// 2. Call `registry.register_natives()` with the new polytone module refs
/// 3. Verify the registration via `registry.modules()`
///
/// # Note
///
/// Polytone contracts are NOT registered as abstract modules in the
/// traditional sense — they are external infrastructure. The ibc-client
/// stores polytone note addresses as part of `RegisterInfrastructure`.
/// This function handles the registry-side code ID tracking.
fn register_polytone_code_ids() -> Result<()> {
    // TODO(cw-orch-specialist):
    //   let abstract_ = Abstract::load_from(chain)?;
    //   // Polytone contracts are tracked as native modules for code ID reference
    //   abstract_.registry.register_natives(vec![
    //       (&polytone.note, version),
    //       (&polytone.voice, version),
    //       (&polytone.proxy, version),
    //   ])?;
    Ok(())
}

// ──────────────────────────────────────────────
// Phase C: Update ibc-client infrastructure
// ──────────────────────────────────────────────

/// Register or update the IBC infrastructure entries in the ibc-client.
///
/// The ibc-client stores a mapping of `chain_name → (polytone_note_addr,
/// remote_abstract_host, remote_proxy_addr)`. When the polytone note
/// contract is redeployed with a new address, this mapping must be updated.
///
/// # Steps
///
/// 1. Call `ibc_client.register_infrastructure(chain, host, note_addr)`
/// 2. Wait for the WhoAmI callback to resolve the remote proxy address
/// 3. Verify the remote proxy is stored in the infrastructure state
///
/// # Post-migration proxy address concern
///
/// The v2 voice contract uses the full 32-byte `instantiate2_address` output
/// instead of slicing to `contract_addr_len`. This means proxy addresses
/// will be **different** from v1. After migration, the ibc-client will
/// receive the new proxy address via the WhoAmI callback.
fn register_ibc_infrastructure() -> Result<()> {
    // TODO(cw-orch-specialist):
    //   let ibc_client = IbcClient::new(IBC_CLIENT, chain.clone());
    //   ibc_client.register_infrastructure(
    //       chain_name,
    //       remote_abstract_host,
    //       polytone_note_address,
    //   )?;
    //   // The WhoAmI callback will resolve the remote proxy address.
    //   // Wait for and verify the callback before proceeding.
    Ok(())
}

// ──────────────────────────────────────────────
// Phase D: Instantiate note + voice (optional)
// ──────────────────────────────────────────────

/// Instantiate the polytone note and voice contracts.
///
/// The note is instantiated on the "source" chain (where ibc-client lives),
/// and the voice is instantiated on the "destination" chain (where
/// ibc-host lives).
///
/// # Steps
///
/// 1. Instantiate note with `block_max_gas` and optional `pair`
/// 2. Instantiate voice with `proxy_code_id` and `block_max_gas`
/// 3. Store addresses in cw-orch state
fn instantiate_note_and_voice() -> Result<()> {
    // TODO(cw-orch-specialist):
    //   let polytone = Polytone::load_from(chain)?;
    //   polytone.instantiate_note(admin)?;
    //   polytone.instantiate_voice(admin)?;
    //   info!("Note address: {}", polytone.note.address()?);
    //   info!("Voice address: {}", polytone.voice.address()?);
    Ok(())
}

// ──────────────────────────────────────────────
// Phase E: Create IBC channel (optional)
// ──────────────────────────────────────────────

/// Create an IBC channel between the polytone note and voice contracts.
///
/// Uses `cw-orch-interchain` to perform the channel handshake via a relayer.
///
/// # Steps
///
/// 1. Create the interchain environment (DaemonInterchain)
/// 2. Call `PolytoneConnection::deploy_between()` or `connect_if_needed()`
/// 3. Verify the channel is open with `IbcQueryHandler`
fn connect_polytone_channel() -> Result<()> {
    // TODO(cw-orch-specialist):
    //   let interchain = DaemonInterchain::new(
    //       vec![src_chain_info, dst_chain_info],
    //       &ChannelCreationValidator,
    //   )?;
    //   let connection = PolytoneConnection::deploy_between_if_needed(
    //       &interchain, &src_chain_id, &dst_chain_id,
    //   )?;
    //   info!("Channel: {}", connection.note.channel()?);
    Ok(())
}

// ──────────────────────────────────────────────
// Phase F: Verify migration state
// ──────────────────────────────────────────────

/// Verify that the migration was applied correctly.
///
/// Checks:
/// - Polytone code IDs are stored and match expected v2 hashes
/// - ibc-client infrastructure entries are populated
/// - Remote proxy addresses are resolved (if applicable)
/// - IBC channel is open (if applicable)
fn verify_migration() -> Result<()> {
    // TODO(cw-orch-specialist):
    //   let chain = DaemonBuilder::new(chain_id).build()?;
    //   let polytone = Polytone::load_from(chain)?;
    //   assert!(polytone.note.code_id().is_ok());
    //   assert!(polytone.voice.code_id().is_ok());
    //   assert!(polytone.proxy.code_id().is_ok());
    //   info!("Migration verified — all polytone code IDs present");
    Ok(())
}

// ──────────────────────────────────────────────
// Main entry point
// ──────────────────────────────────────────────

fn main() -> Result<()> {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .unwrap();
    // dotenv::dotenv().ok();
    env_logger::init();

    let _cli = Cli::parse();

    info!("=== Polytone Migration Script ===");

    // Phase A: Upload new polytone wasms
    info!("[Phase A] Uploading polytone contracts...");
    upload_polytone_contracts()?;

    // Phase B: Register code IDs in Abstract registry
    info!("[Phase B] Registering polytone code IDs...");
    register_polytone_code_ids()?;

    // Phase C: Update ibc-client infrastructure
    info!("[Phase C] Registering IBC infrastructure...");
    register_ibc_infrastructure()?;

    // Phase D: Instantiate note + voice (optional, CLI flag)
    // if cli.full_connection {
    //     info!("[Phase D] Instantiating note and voice...");
    //     instantiate_note_and_voice()?;
    // }

    // Phase E: Create IBC channel (optional, CLI flag)
    // if cli.full_connection && cli.remote_chain.is_some() {
    //     info!("[Phase E] Creating IBC channel...");
    //     connect_polytone_channel()?;
    // }

    // Phase F: Verify
    info!("[Phase F] Verifying migration...");
    verify_migration()?;

    info!("=== Migration complete ===");
    Ok(())
}
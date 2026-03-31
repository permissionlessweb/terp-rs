//! Terp Network chain support for ict-rs.
//!
//! Provides abstract traits for ZK circuit + CosmWasm contract development
//! on Terp Network, centred around the `zk-wasmvm` custom CosmWasm VM
//! extension that stores and executes on-chain verifying keys.
//!
//! # Feature gate
//!
//! This entire module is gated behind the `terp` feature flag.  Enable it
//! in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! ict-rs = { version = "*", features = ["terp"] }
//! ```
//!
//! # Design
//!
//! Three layered traits model the lifecycle of a ZK-powered dApp on Terp:
//!
//! 1. [`ZkWasmVmCircuit`] — describes a halo2 circuit: params, key-gen,
//!    prove, verify, and serialisation.
//! 2. [`ZkWasmVmContract`] — binds a CosmWasm contract (as embedded bytes)
//!    to a specific [`ZkWasmVmCircuit`].
//! 3. [`ZkWasmVmSuite`] — the full development interface: key management,
//!    proof operations, and on-chain deployment via `terpd tx wasm headstash`.
//!
//! None of the circuit/proof methods have default implementations — those
//! belong in the concrete structs that implement the traits.  Only lifecycle
//! helpers with trivially obvious defaults are provided here.

use std::path::Path;

use async_trait::async_trait;

use crate::chain::Chain;
use crate::error::IctError;

// ─── Error type ──────────────────────────────────────────────────────────────

/// Errors that can occur inside the ZK + CosmWasm development suite.
#[derive(Debug, thiserror::Error)]
pub enum ZkSuiteError {
    /// Failure inside circuit parameter generation, key-gen, proving, or
    /// serialisation.
    #[error("circuit error: {0}")]
    Circuit(String),

    /// I/O failure while reading or writing proving/verifying key files.
    #[error("key I/O error: {0}")]
    KeyIo(#[from] std::io::Error),

    /// Proof verification rejected the supplied proof or public inputs.
    #[error("proof verification failed: {0}")]
    Verification(String),

    /// Failure during on-chain deployment (store code, instantiate, register VK).
    #[error("deploy error: {0}")]
    Deploy(String),

    /// The verifying-key bytes do not begin or end with the expected magic
    /// header/footer that the on-chain `zk-wasmvm` host expects.
    #[error("vk validation failed: {0}")]
    VkValidation(String),

    /// Propagated error from the underlying ict-rs framework.
    #[error("ict-rs error: {0}")]
    Ict(#[from] IctError),
}

// ─── ZkWasmVmCircuit ─────────────────────────────────────────────────────────

/// Abstract description of a halo2 circuit for use with Terp's `zk-wasmvm`.
///
/// Implementors supply all circuit-specific types and operations.  The trait is
/// deliberately free of any halo2 concrete type references so that callers do
/// not need halo2 as a direct dependency merely to write test harnesses.
///
/// # Associated types
///
/// | Type | Meaning |
/// |------|---------|
/// | `Params` | Structured Reference String (SRS), e.g. `halo2_proofs::poly::commitment::Params` |
/// | `ProvingKey` | Proving key produced by `keygen` |
/// | `VerifyingKey` | Verifying key produced by `keygen` |
/// | `Proof` | Serialisable proof blob |
/// | `PublicInputs` | Public witness / instance values |
pub trait ZkWasmVmCircuit: Send + Sync {
    /// Circuit-specific SRS / parameter type.
    type Params: Send + Sync;
    /// Proving key type.
    type ProvingKey: Send + Sync;
    /// Verifying key type.
    type VerifyingKey: Send + Sync;
    /// Proof type (typically an opaque byte blob but kept generic for testing).
    type Proof: Send + Sync;
    /// Public instance values consumed by `prove` and `verify`.
    type PublicInputs: Send + Sync;

    /// Human-readable circuit name used in error messages and file names.
    fn circuit_name() -> &'static str;

    /// Build the SRS / commitment params for a circuit of depth `k`.
    ///
    /// `k` determines the maximum circuit size: `2^k` rows.  Typical values
    /// are 10–20 depending on circuit complexity.
    fn build_params(k: u32) -> Result<Self::Params, ZkSuiteError>;

    /// Generate a proving key and verifying key from the given params.
    fn keygen(
        params: &Self::Params,
    ) -> Result<(Self::ProvingKey, Self::VerifyingKey), ZkSuiteError>;

    /// Create a proof for the given public inputs using the proving key.
    fn prove(
        params: &Self::Params,
        pk: &Self::ProvingKey,
        inputs: &Self::PublicInputs,
    ) -> Result<Self::Proof, ZkSuiteError>;

    /// Verify a proof against the verifying key and public inputs.
    ///
    /// Returns `Ok(())` on success; `Err(ZkSuiteError::Verification(_))` on
    /// failure.
    fn verify(
        params: &Self::Params,
        vk: &Self::VerifyingKey,
        proof: &Self::Proof,
        inputs: &Self::PublicInputs,
    ) -> Result<(), ZkSuiteError>;

    /// Serialise a proof to raw bytes for on-chain submission.
    fn proof_to_bytes(proof: &Self::Proof) -> Vec<u8>;

    /// Serialise a verifying key to raw bytes for on-chain registration.
    fn vk_to_bytes(vk: &Self::VerifyingKey) -> Vec<u8>;

    /// Deserialise a verifying key from raw bytes (e.g., read back from disk).
    fn vk_from_bytes(bytes: &[u8]) -> Result<Self::VerifyingKey, ZkSuiteError>;
}

// ─── ZkWasmVmContract ────────────────────────────────────────────────────────

/// Binds a CosmWasm contract to a specific [`ZkWasmVmCircuit`].
///
/// The contract wasm binary is expected to be embedded at compile time via
/// `include_bytes!`.  Implementors must also provide header/footer validation
/// so the suite can detect corrupt or incompatible VK blobs before attempting
/// to submit them on-chain.
pub trait ZkWasmVmContract: Send + Sync {
    /// The halo2 circuit this contract is designed to verify on-chain.
    type Circuit: ZkWasmVmCircuit;

    /// Human-readable contract name (used in labels and error messages).
    fn contract_name() -> &'static str;

    /// The compiled `.wasm` binary, typically embedded as:
    ///
    /// ```rust,ignore
    /// fn wasm_byte_code() -> &'static [u8] {
    ///     include_bytes!("../../artifacts/my_contract.wasm")
    /// }
    /// ```
    fn wasm_byte_code() -> &'static [u8];

    /// Validate that `vk_bytes` begins with the magic header the on-chain
    /// `zk-wasmvm` host expects (e.g., a version tag, curve identifier, or
    /// proof-system marker).
    ///
    /// Returns `Err(ZkSuiteError::VkValidation(_))` if the header is invalid.
    fn validate_vk_header(vk_bytes: &[u8]) -> Result<(), ZkSuiteError>;

    /// Validate that `vk_bytes` ends with the expected magic footer.
    ///
    /// Returns `Err(ZkSuiteError::VkValidation(_))` if the footer is invalid.
    fn validate_vk_footer(vk_bytes: &[u8]) -> Result<(), ZkSuiteError>;
}

// ─── DeployedZkContract ──────────────────────────────────────────────────────

/// Result of a successful [`ZkWasmVmSuite::deploy`] call.
#[derive(Debug, Clone)]
pub struct DeployedZkContract {
    /// CosmWasm code ID assigned by `terpd tx wasm store`.
    pub code_id: u64,
    /// Bech32 contract address after instantiation.
    pub contract_addr: String,
    /// Whether the verifying key was successfully registered on-chain via
    /// `terpd tx wasm headstash`.  A `false` value here means the contract was
    /// deployed but VK registration failed; callers should treat this as a
    /// partial success and retry the registration step.
    pub vk_registered: bool,
}

// ─── ZkWasmVmSuite ───────────────────────────────────────────────────────────

/// Full development interface for a ZK circuit + CosmWasm contract pair on
/// Terp Network.
///
/// # Circuit key management
///
/// Keys (params, PK, VK) are expensive to generate.  The suite persists them
/// to [`ZkWasmVmSuite::keys_dir`] and loads them on subsequent runs:
///
/// ```text
/// <keys_dir>/
///   params.bin        — SRS / commitment parameters
///   proving_key.bin   — proving key
///   verifying_key.bin — verifying key
/// ```
///
/// # Deployment flow
///
/// [`ZkWasmVmSuite::deploy`] runs the following steps using
/// [`Chain::chain_exec`]:
///
/// 1. `terpd tx wasm store <wasm>` — uploads the contract binary.
/// 2. `terpd tx wasm instantiate <code_id> <init_msg>` — instantiates the
///    contract.
/// 3. `terpd tx wasm headstash <contract_addr> --vk <base64(vk_bytes)>` —
///    registers the verifying key via the custom `zk-wasmvm` transaction type.
///
/// Step 3 is Terp-specific and is **not** standard CosmWasm.  The `headstash`
/// sub-command is provided by the `zk-wasmvm` module built into `terpd`.
#[async_trait]
pub trait ZkWasmVmSuite: Send + Sync {
    /// The halo2 circuit used by this suite.
    type Circuit: ZkWasmVmCircuit;
    /// The CosmWasm contract paired with [`Self::Circuit`].
    type Contract: ZkWasmVmContract<Circuit = Self::Circuit>;
    /// The ict-rs chain this suite deploys to.
    type Chain: Chain;

    // ── Key management ────────────────────────────────────────────────────

    /// Directory on the local filesystem where circuit keys are persisted.
    ///
    /// Expected layout:
    /// - `params.bin`
    /// - `proving_key.bin`
    /// - `verifying_key.bin`
    fn keys_dir(&self) -> &Path;

    /// Load the proving key and verifying key from [`Self::keys_dir`].
    ///
    /// Reads `proving_key.bin` and `verifying_key.bin` using the circuit's
    /// own deserialisation (i.e. calls [`ZkWasmVmCircuit::vk_from_bytes`]).
    ///
    /// Returns `Err(ZkSuiteError::KeyIo(_))` if the files are absent or
    /// unreadable.
    fn load_circuit_keys(
        &self,
    ) -> Result<
        (
            <Self::Circuit as ZkWasmVmCircuit>::ProvingKey,
            <Self::Circuit as ZkWasmVmCircuit>::VerifyingKey,
        ),
        ZkSuiteError,
    >;

    /// Persist the proving key and verifying key to [`Self::keys_dir`].
    ///
    /// Writes `proving_key.bin` (via [`ZkWasmVmCircuit::proof_to_bytes`] is
    /// **not** used here — the PK serialisation is circuit-specific and left
    /// to the implementor) and `verifying_key.bin` (via
    /// [`ZkWasmVmCircuit::vk_to_bytes`]).
    ///
    /// Creates the directory if it does not exist.
    fn save_circuit_keys(
        &self,
        pk: &<Self::Circuit as ZkWasmVmCircuit>::ProvingKey,
        vk: &<Self::Circuit as ZkWasmVmCircuit>::VerifyingKey,
    ) -> Result<(), ZkSuiteError>;

    /// Generate SRS params, run key-gen, and persist the keys to
    /// [`Self::keys_dir`].
    ///
    /// This is a convenience wrapper around
    /// [`ZkWasmVmCircuit::build_params`] + [`ZkWasmVmCircuit::keygen`] +
    /// [`ZkWasmVmSuite::save_circuit_keys`].  After this call succeeds,
    /// [`Self::load_circuit_keys`] will return the newly generated keys.
    fn build_and_save_keys(&self, k: u32) -> Result<(), ZkSuiteError>;

    // ── Proof operations ─────────────────────────────────────────────────

    /// Create a proof for the given public inputs.
    ///
    /// Loads the proving key from disk (via [`Self::load_circuit_keys`]) on
    /// every call.  Implementors may cache the key in the suite struct if
    /// performance matters.
    fn prove(
        &self,
        inputs: &<Self::Circuit as ZkWasmVmCircuit>::PublicInputs,
    ) -> Result<<Self::Circuit as ZkWasmVmCircuit>::Proof, ZkSuiteError>;

    /// Verify a proof against the on-disk verifying key and the given public
    /// inputs.
    ///
    /// Loads the verifying key from disk (via [`Self::load_circuit_keys`]) on
    /// every call.
    fn verify(
        &self,
        proof: &<Self::Circuit as ZkWasmVmCircuit>::Proof,
        inputs: &<Self::Circuit as ZkWasmVmCircuit>::PublicInputs,
    ) -> Result<(), ZkSuiteError>;

    // ── Deployment ────────────────────────────────────────────────────────

    /// Deploy the contract and register the verifying key on Terp Network.
    ///
    /// # Steps
    ///
    /// 1. Write the wasm bytes from [`ZkWasmVmContract::wasm_byte_code`] to a
    ///    temporary file and call:
    ///    ```text
    ///    terpd tx wasm store <tmp.wasm> --from <deployer_key> ...
    ///    ```
    ///    Parse the `code_id` from the JSON response.
    ///
    /// 2. Instantiate with a minimal init message:
    ///    ```text
    ///    terpd tx wasm instantiate <code_id> '{}' \
    ///        --label <contract_name> --from <deployer_key> --no-admin ...
    ///    ```
    ///    Parse the `contract_address` from the response.
    ///
    /// 3. Validate and register the verifying key:
    ///    ```text
    ///    terpd tx wasm headstash <contract_addr> \
    ///        --vk <base64(vk_bytes)> --from <deployer_key> ...
    ///    ```
    ///    The `--vk` flag accepts a standard base64-encoded blob of the raw
    ///    verifying key bytes produced by [`ZkWasmVmCircuit::vk_to_bytes`].
    ///    The `headstash` sub-command is provided by the `x/zkwasmvm` module
    ///    in `terpd` and is NOT available on standard CosmWasm chains.
    ///
    /// # Returns
    ///
    /// [`DeployedZkContract`] with `vk_registered = true` on full success.
    /// If step 3 fails the method returns `Ok(DeployedZkContract { vk_registered: false, .. })`
    /// so callers can decide whether to abort or retry registration.
    ///
    /// # Using `chain_exec`
    ///
    /// All three steps use [`Chain::chain_exec`] so they run inside the same
    /// Docker container as the chain, avoiding host-side binary dependencies.
    async fn deploy(
        &self,
        chain: &Self::Chain,
        deployer_key: &str,
    ) -> Result<DeployedZkContract, ZkSuiteError>;
}

// ─── TerpChainConfig helper ───────────────────────────────────────────────────

/// Build a [`crate::chain::ChainConfig`] suitable for a local Terp Network
/// node running the `zk-wasmvm` module.
///
/// Defaults:
/// - Image: `terpnetwork/terp-core:local-zk`
/// - Binary: `terpd`
/// - Denom: `uterp`
/// - Chain ID: `terp-local-1`
/// - Vote extensions enabled at height 2 (required for hashmerchant VE flow)
/// - Fast block time: `200ms`
///
/// Override individual fields after construction as needed.
pub fn terp_chain_config() -> crate::chain::ChainConfig {
    use std::collections::HashMap;
    use serde_json::json;
    use crate::chain::{ChainConfig, ChainType, GenesisStyle, SigningAlgorithm};
    use crate::runtime::DockerImage;

    let mut config_overrides: HashMap<String, serde_json::Value> = HashMap::new();

    // Enable REST API and gRPC in app.toml.
    config_overrides.insert(
        "config/app.toml".into(),
        json!({
            "api": {
                "enable": true,
                "address": "tcp://0.0.0.0:1317"
            },
            "grpc": {
                "enable": true,
                "address": "0.0.0.0:9090"
            },
            // hashmerchant sidecar URL — override if running a custom sidecar
            "hashmerchant": {
                "sidecar-url": "http://localhost:8080"
            }
        }),
    );

    // Fast block times and vote extension enable height in config.toml / genesis.
    config_overrides.insert(
        "config/config.toml".into(),
        json!({
            "consensus": {
                "timeout_propose":        "200ms",
                "timeout_propose_delta":  "200ms",
                "timeout_prevote":        "200ms",
                "timeout_prevote_delta":  "200ms",
                "timeout_precommit":      "200ms",
                "timeout_precommit_delta":"200ms",
                "timeout_commit":         "200ms"
            }
        }),
    );

    ChainConfig {
        chain_type: ChainType::Cosmos,
        name: "terp".to_string(),
        chain_id: "terp-local-1".to_string(),
        images: vec![DockerImage {
            repository: "terpnetwork/terp-core".to_string(),
            version: "local-zk".to_string(),
            uid_gid: None,
        }],
        bin: "terpd".to_string(),
        bech32_prefix: "terp".to_string(),
        denom: "uterp".to_string(),
        coin_type: 118,
        signing_algorithm: SigningAlgorithm::Secp256k1,
        gas_prices: "0.025uterp".to_string(),
        gas_adjustment: 1.5,
        trusting_period: "336h".to_string(),
        block_time: "200ms".to_string(),
        genesis: None,
        // Enable vote extensions at height 2 so the hashmerchant VE flow works.
        modify_genesis: Some(Box::new(modify_terp_genesis)),
        pre_genesis: None,
        config_file_overrides: config_overrides,
        additional_start_args: Vec::new(),
        env: Vec::new(),
        sidecar_configs: Vec::new(),
        faucet: None,
        genesis_style: GenesisStyle::Modern,
    }
}

/// Apply Terp-specific genesis mutations for local testing.
///
/// Sets fast governance, the correct staking/mint denom, and enables vote
/// extensions at height 2 (required for the hashmerchant module).
pub fn modify_terp_genesis(
    _cfg: &crate::chain::ChainConfig,
    raw: Vec<u8>,
) -> crate::error::Result<Vec<u8>> {
    use serde_json::json;

    let mut genesis: serde_json::Value = serde_json::from_slice(&raw)
        .map_err(|e| IctError::Config(format!("parse terp genesis: {e}")))?;

    // ── Staking ───────────────────────────────────────────────────────────
    if let Some(params) = genesis.pointer_mut("/app_state/staking/params") {
        params["bond_denom"] = json!("uterp");
        params["unbonding_time"] = json!("120s");
    }

    // ── Mint ──────────────────────────────────────────────────────────────
    if let Some(params) = genesis.pointer_mut("/app_state/mint/params") {
        params["mint_denom"] = json!("uterp");
    }

    // ── Governance (fast voting for tests) ────────────────────────────────
    if let Some(params) = genesis.pointer_mut("/app_state/gov/params") {
        params["min_deposit"] = json!([{"denom": "uterp", "amount": "10000000"}]);
        params["voting_period"] = json!("90s");
        params["max_deposit_period"] = json!("90s");
    }
    // v1beta1 fallback
    if let Some(dp) = genesis.pointer_mut("/app_state/gov/deposit_params") {
        dp["min_deposit"] = json!([{"denom": "uterp", "amount": "10000000"}]);
        dp["max_deposit_period"] = json!("90s");
    }
    if let Some(vp) = genesis.pointer_mut("/app_state/gov/voting_params") {
        vp["voting_period"] = json!("90s");
    }

    // ── Crisis ────────────────────────────────────────────────────────────
    if let Some(fee) = genesis.pointer_mut("/app_state/crisis/constant_fee") {
        fee["denom"] = json!("uterp");
    }

    // ── Vote extensions (hashmerchant / zk-wasmvm) ────────────────────────
    // Enable at height 2 so the chain produces at least one normal block first.
    if let Some(params) = genesis.pointer_mut("/consensus/params/abci") {
        params["vote_extensions_enable_height"] = json!("2");
    }
    // Cosmos SDK 0.50 path
    if let Some(params) = genesis.pointer_mut("/app_state/consensus/params/abci") {
        params["vote_extensions_enable_height"] = json!("2");
    }

    // ── hashmerchant genesis state ────────────────────────────────────────
    // Ensure the module is initialised with sensible defaults.
    if let Some(hm) = genesis.pointer_mut("/app_state/hashmerchant") {
        if hm.get("params").is_none() {
            hm["params"] = json!({
                "quorum_fraction": "0.67"
            });
        }
        if hm.get("registered_chains").is_none() {
            hm["registered_chains"] = json!([]);
        }
    }

    serde_json::to_vec_pretty(&genesis)
        .map_err(|e| IctError::Config(format!("serialize terp genesis: {e}")))
}

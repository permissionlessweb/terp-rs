//! `ZkWasmVmSuite` implementation for the **No-Rick** test circuit.
//!
//! Mirrors the pattern contracts use with cw-orch's `Uploadable`:
//! each test circuit gets its own suite struct that ties together a
//! `ZkWasmVmCircuit` (the halo2 circuit), a `ZkWasmVmContract` (the
//! CosmWasm verifier wasm), and a `ZkWasmVmSuite` (the full dev interface).
//!
//! # Usage
//! ```rust,ignore
//! use zk_test_press::no_rick_suite::NoRickSuite;
//! use ict_rs::chain::terp::{ZkWasmVmSuite, terp_chain_config};
//!
//! let suite = NoRickSuite::new("./data/keys/no_rick");
//! // Generate and persist circuit keys
//! suite.build_and_save_keys(12)?;
//! // Deploy to a running ict-rs Terp chain
//! let result = suite.deploy(&chain, "validator").await?;
//! ```

use std::path::{Path, PathBuf};

use async_trait::async_trait;
use ict_rs::chain::cosmos::CosmosChain;
use ict_rs::chain::terp::{
    DeployedZkContract, ZkSuiteError, ZkWasmVmCircuit, ZkWasmVmContract, ZkWasmVmSuite,
};
use zk_cosmwasm::{CosmwasmCircuit, Instance, Proof, ProvingKey};

// ── NoRickCircuit ─────────────────────────────────────────────────────────────

/// Bridge between the No-Rick halo2 circuit and the `ZkWasmVmCircuit` trait.
///
/// Associated types use opaque byte vectors so callers do not need to import
/// halo2 directly.  Fill in the `todo!()` stubs once the halo2 keygen and
/// proof-creation API is wired in `zk-headstash`.
pub struct NoRickCircuit;

impl ZkWasmVmCircuit for NoRickCircuit {
    /// Serialised SRS / commitment parameters (raw bytes).
    type Params = Vec<u8>;
    /// Proving key wrapper from `zk_cosmwasm`.
    type ProvingKey = ProvingKey;
    /// Serialised verifying key (raw bytes).
    type VerifyingKey = Vec<u8>;
    /// Proof blob from `zk_cosmwasm`.
    type Proof = Proof;
    /// Public instance values from `zk_cosmwasm`.
    type PublicInputs = Instance;

    fn circuit_name() -> &'static str {
        "no-rick"
    }

    fn build_params(_k: u32) -> Result<Self::Params, ZkSuiteError> {
        todo!("wire halo2_proofs::poly::commitment::Params::new(k)")
    }

    fn keygen(
        _params: &Self::Params,
    ) -> Result<(Self::ProvingKey, Self::VerifyingKey), ZkSuiteError> {
        todo!("wire halo2_proofs::plonk::keygen_pk / keygen_vk")
    }

    fn prove(
        _params: &Self::Params,
        _pk: &Self::ProvingKey,
        _inputs: &Self::PublicInputs,
    ) -> Result<Self::Proof, ZkSuiteError> {
        todo!("wire halo2_proofs::plonk::create_proof with NoRickCircuit instance")
    }

    fn verify(
        _params: &Self::Params,
        _vk: &Self::VerifyingKey,
        _proof: &Self::Proof,
        _inputs: &Self::PublicInputs,
    ) -> Result<(), ZkSuiteError> {
        todo!("wire halo2_proofs::plonk::verify_proof")
    }

    fn proof_to_bytes(proof: &Self::Proof) -> Vec<u8> {
        proof.as_bytes().to_vec()
    }

    fn vk_to_bytes(vk: &Self::VerifyingKey) -> Vec<u8> {
        vk.clone()
    }

    fn vk_from_bytes(bytes: &[u8]) -> Result<Self::VerifyingKey, ZkSuiteError> {
        Ok(bytes.to_vec())
    }
}

// ── NoRickWasmContract ───────────────────────────────────────────────────────

/// Binds the No-Rick CosmWasm contract wasm to [`NoRickCircuit`].
pub struct NoRickWasmContract;

impl ZkWasmVmContract for NoRickWasmContract {
    type Circuit = NoRickCircuit;

    fn contract_name() -> &'static str {
        "no-rick"
    }

    fn wasm_byte_code() -> &'static [u8] {
        include_bytes!("circuits/no-rick/artifacts/zk_wasmvm_test.wasm")
    }

    /// VK header: first 4 bytes must be the ASCII magic `\x00VK\x01`.
    fn validate_vk_header(vk_bytes: &[u8]) -> Result<(), ZkSuiteError> {
        // Stub: accept any non-empty blob until the on-chain format is frozen.
        if vk_bytes.is_empty() {
            return Err(ZkSuiteError::VkValidation("empty vk".into()));
        }
        Ok(())
    }

    /// VK footer: last byte must be `0xFF`.
    fn validate_vk_footer(vk_bytes: &[u8]) -> Result<(), ZkSuiteError> {
        if vk_bytes.is_empty() {
            return Err(ZkSuiteError::VkValidation("empty vk".into()));
        }
        Ok(())
    }
}

// ── NoRickSuite ───────────────────────────────────────────────────────────────

/// Full development suite for the No-Rick ZK circuit + CosmWasm contract pair.
///
/// # Keys directory layout
/// ```text
/// <keys_dir>/
///   params.bin          — SRS / commitment parameters
///   proving_key.bin     — proving key
///   verifying_key.bin   — verifying key
/// ```
pub struct NoRickSuite {
    keys_dir: PathBuf,
}

impl NoRickSuite {
    /// Create a new suite, storing circuit keys under `keys_dir`.
    pub fn new<P: Into<PathBuf>>(keys_dir: P) -> Self {
        Self { keys_dir: keys_dir.into() }
    }
}

#[async_trait]
impl ZkWasmVmSuite for NoRickSuite {
    type Circuit = NoRickCircuit;
    type Contract = NoRickWasmContract;
    type Chain = CosmosChain;

    fn keys_dir(&self) -> &Path {
        &self.keys_dir
    }

    fn load_circuit_keys(
        &self,
    ) -> Result<
        (
            <Self::Circuit as ZkWasmVmCircuit>::ProvingKey,
            <Self::Circuit as ZkWasmVmCircuit>::VerifyingKey,
        ),
        ZkSuiteError,
    > {
        let pk_bytes = std::fs::read(self.keys_dir.join("proving_key.bin"))?;
        let vk_bytes = std::fs::read(self.keys_dir.join("verifying_key.bin"))?;
        Ok((ProvingKey::from_bytes(pk_bytes), vk_bytes))
    }

    fn save_circuit_keys(
        &self,
        pk: &<Self::Circuit as ZkWasmVmCircuit>::ProvingKey,
        vk: &<Self::Circuit as ZkWasmVmCircuit>::VerifyingKey,
    ) -> Result<(), ZkSuiteError> {
        std::fs::create_dir_all(&self.keys_dir)?;
        std::fs::write(self.keys_dir.join("proving_key.bin"), pk.as_bytes())?;
        std::fs::write(self.keys_dir.join("verifying_key.bin"), vk)?;
        Ok(())
    }

    fn build_and_save_keys(&self, k: u32) -> Result<(), ZkSuiteError> {
        let params = NoRickCircuit::build_params(k)?;
        let (pk, vk) = NoRickCircuit::keygen(&params)?;
        std::fs::write(self.keys_dir.join("params.bin"), &params)?;
        self.save_circuit_keys(&pk, &vk)
    }

    fn prove(
        &self,
        inputs: &<Self::Circuit as ZkWasmVmCircuit>::PublicInputs,
    ) -> Result<<Self::Circuit as ZkWasmVmCircuit>::Proof, ZkSuiteError> {
        let params_bytes = std::fs::read(self.keys_dir.join("params.bin"))?;
        let (pk, _) = self.load_circuit_keys()?;
        NoRickCircuit::prove(&params_bytes, &pk, inputs)
    }

    fn verify(
        &self,
        proof: &<Self::Circuit as ZkWasmVmCircuit>::Proof,
        inputs: &<Self::Circuit as ZkWasmVmCircuit>::PublicInputs,
    ) -> Result<(), ZkSuiteError> {
        let params_bytes = std::fs::read(self.keys_dir.join("params.bin"))?;
        let (_, vk) = self.load_circuit_keys()?;
        NoRickCircuit::verify(&params_bytes, &vk, proof, inputs)
    }

    async fn deploy(
        &self,
        chain: &Self::Chain,
        deployer_key: &str,
    ) -> Result<DeployedZkContract, ZkSuiteError> {
        use base64::Engine as _;

        // ── 1. Write wasm to a temp file inside the container ────────────────
        let wasm = NoRickWasmContract::wasm_byte_code();
        let wasm_b64 = base64::engine::general_purpose::STANDARD.encode(wasm);
        let remote_wasm = "/tmp/no_rick.wasm";
        chain
            .chain_exec(&[
                "sh",
                "-c",
                &format!("echo '{}' | base64 -d > {}", wasm_b64, remote_wasm),
            ])
            .await
            .map_err(|e| ZkSuiteError::Deploy(format!("wasm upload: {e}")))?;

        // ── 2. Store code ─────────────────────────────────────────────────────
        let store_out = chain
            .chain_exec(&[
                "terpd",
                "tx",
                "wasm",
                "store",
                remote_wasm,
                "--from",
                deployer_key,
                "--gas",
                "auto",
                "--gas-adjustment",
                "1.5",
                "--output",
                "json",
                "-y",
            ])
            .await
            .map_err(|e| ZkSuiteError::Deploy(format!("store code: {e}")))?;

        let store_json: serde_json::Value = serde_json::from_slice(&store_out.stdout)
            .map_err(|e| ZkSuiteError::Deploy(format!("parse store output: {e}")))?;
        let code_id: u64 = store_json
            .pointer("/logs/0/events/0/attributes/0/value")
            .or_else(|| store_json.get("code_id"))
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse().ok())
            .ok_or_else(|| ZkSuiteError::Deploy("could not parse code_id".into()))?;

        // ── 3. Instantiate ───────────────────────────────────────────────────
        let inst_out = chain
            .chain_exec(&[
                "terpd",
                "tx",
                "wasm",
                "instantiate",
                &code_id.to_string(),
                "{}",
                "--label",
                NoRickWasmContract::contract_name(),
                "--no-admin",
                "--from",
                deployer_key,
                "--gas",
                "auto",
                "--gas-adjustment",
                "1.5",
                "--output",
                "json",
                "-y",
            ])
            .await
            .map_err(|e| ZkSuiteError::Deploy(format!("instantiate: {e}")))?;

        let inst_json: serde_json::Value = serde_json::from_slice(&inst_out.stdout)
            .map_err(|e| ZkSuiteError::Deploy(format!("parse instantiate output: {e}")))?;
        let contract_addr = inst_json
            .pointer("/logs/0/events/1/attributes/0/value")
            .or_else(|| inst_json.get("contract_address"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| ZkSuiteError::Deploy("could not parse contract_address".into()))?
            .to_string();

        // ── 4. Register verifying key ─────────────────────────────────────────
        let (_, vk) = self.load_circuit_keys()?;
        NoRickWasmContract::validate_vk_header(&vk)?;
        NoRickWasmContract::validate_vk_footer(&vk)?;
        let vk_b64 = base64::engine::general_purpose::STANDARD.encode(&vk);

        let vk_registered = chain
            .chain_exec(&[
                "terpd",
                "tx",
                "wasm",
                "headstash",
                &contract_addr,
                "--vk",
                &vk_b64,
                "--from",
                deployer_key,
                "--gas",
                "auto",
                "--gas-adjustment",
                "1.5",
                "-y",
            ])
            .await
            .is_ok();

        Ok(DeployedZkContract { code_id, contract_addr, vk_registered })
    }
}

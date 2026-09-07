//! Toy “circuit” verifier for recovery Poseidon digests.
//!
//! ## Design (mirrors `terp-zkjwt::verify`)
//!
//! - **Structural** (default): check envelope — non-empty proof, known
//!   `circuit_id`, public instances parse, and optional witness rehash matches
//!   the public challenge when `witness_preimage` is supplied.
//! - **Host** (`zk-host`): after structural checks, call
//!   `deps.api.proof_instance_verify(zkid, proof, public_inputs)`.
//!
//! A production Halo2 Poseidon chip would generate real proofs whose public
//! instance is the same 32-byte challenge digest produced by
//! `terp_recovery::RecoveryHashAlg::PoseidonPallas`.

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Deps};

use crate::error::DemoError;
use crate::instances::{
    parse_public_instances, CIRCUIT_ID_RECOVERY_PREIMAGE, CHALLENGE_LEN,
};

/// Auth / verify payload for the toy preimage circuit.
#[cw_serde]
pub struct PoseidonToyPayload {
    /// Must be [`CIRCUIT_ID_RECOVERY_PREIMAGE`] (or a future registered id).
    pub circuit_id: String,
    /// Application-assigned circuit id for host verify (when using `zk-host`).
    pub zkid: Option<u64>,
    /// Opaque proof bytes (Halo2 proof in production; any non-empty bytes for structural demo).
    pub proof: Binary,
    /// Public instances: challenge (32) || rest.
    pub public_inputs: Binary,
    /// Optional witness preimage used only by structural rehash check
    /// (`PoseidonPallas` one-shot must equal public challenge).
    pub witness_preimage: Option<Binary>,
}

fn check_envelope(payload: &PoseidonToyPayload) -> Result<(), DemoError> {
    if payload.proof.is_empty() {
        return Err(DemoError::InvalidProof {
            reason: "empty proof".into(),
        });
    }
    match payload.circuit_id.as_str() {
        CIRCUIT_ID_RECOVERY_PREIMAGE => {}
        other => {
            return Err(DemoError::InvalidProof {
                reason: format!("unknown circuit_id: {other}"),
            });
        }
    }
    let inst = parse_public_instances(&payload.public_inputs)?;
    if inst.challenge.len() != CHALLENGE_LEN {
        return Err(DemoError::InvalidProof {
            reason: "challenge slice length invalid".into(),
        });
    }

    // Structural preimage check: if witness provided, rehash must match public digest.
    if let Some(pre) = &payload.witness_preimage {
        let got = terp_recovery::poseidon_pallas_hash_once(pre.as_slice());
        if got.as_slice() != inst.challenge {
            return Err(DemoError::InvalidProof {
                reason: "witness Poseidon digest != public challenge".into(),
            });
        }
    }
    Ok(())
}

pub trait ToyVerifier {
    fn verify(&self, deps: Deps, payload: &PoseidonToyPayload) -> Result<(), DemoError>;
}

/// Dev/CI verifier: layout + optional witness rehash (no host crypto).
pub struct StructuralToyVerifier;

impl ToyVerifier for StructuralToyVerifier {
    fn verify(&self, _deps: Deps, payload: &PoseidonToyPayload) -> Result<(), DemoError> {
        if payload.zkid.is_none() && payload.proof.is_empty() {
            return Err(DemoError::InvalidProof {
                reason: "need zkid or non-empty proof for structural path".into(),
            });
        }
        check_envelope(payload)
    }
}

/// Production-shaped path: CosmWasm VM `proof_instance_verify`.
#[cfg(feature = "zk-host")]
pub struct HostToyVerifier;

#[cfg(feature = "zk-host")]
impl ToyVerifier for HostToyVerifier {
    fn verify(&self, deps: Deps, payload: &PoseidonToyPayload) -> Result<(), DemoError> {
        check_envelope(payload)?;
        let zkid = payload.zkid.ok_or_else(|| DemoError::InvalidProof {
            reason: "zkid required for host proof_instance_verify".into(),
        })?;
        match deps.api.proof_instance_verify(
            zkid,
            payload.proof.as_slice(),
            payload.public_inputs.as_slice(),
        ) {
            Ok(true) => Ok(()),
            Ok(false) => Err(DemoError::InvalidProof {
                reason: "proof_instance_verify returned false".into(),
            }),
            Err(e) => Err(DemoError::InvalidProof {
                reason: format!("proof_instance_verify error: {e}"),
            }),
        }
    }
}

/// Select verifier: host when feature enabled, else structural.
pub fn default_verifier() -> impl ToyVerifier {
    #[cfg(feature = "zk-host")]
    {
        HostToyVerifier
    }
    #[cfg(not(feature = "zk-host"))]
    {
        StructuralToyVerifier
    }
}

/// Convenience: verify with the default verifier.
pub fn verify_toy(deps: Deps, payload: &PoseidonToyPayload) -> Result<(), DemoError> {
    default_verifier().verify(deps, payload)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instances::build_public_instances;
    use cosmwasm_std::testing::mock_dependencies;
    use terp_recovery::{challenge_preimage, poseidon_pallas_hash_once, DEFAULT_DOMAIN};

    #[test]
    fn structural_accepts_matching_witness() {
        let deps = mock_dependencies();
        let pre = challenge_preimage(DEFAULT_DOMAIN, "chain", "acc", "1", b"tx");
        let digest = poseidon_pallas_hash_once(&pre);
        let instances = build_public_instances(&digest, &[]).unwrap();
        let payload = PoseidonToyPayload {
            circuit_id: CIRCUIT_ID_RECOVERY_PREIMAGE.into(),
            zkid: Some(42),
            proof: Binary::from(b"toy-proof-bytes"),
            public_inputs: instances,
            witness_preimage: Some(Binary::from(pre)),
        };
        verify_toy(deps.as_ref(), &payload).unwrap();
    }

    #[test]
    fn structural_rejects_wrong_witness() {
        let deps = mock_dependencies();
        let pre = challenge_preimage(DEFAULT_DOMAIN, "chain", "acc", "1", b"tx");
        let digest = poseidon_pallas_hash_once(&pre);
        let instances = build_public_instances(&digest, &[]).unwrap();
        let payload = PoseidonToyPayload {
            circuit_id: CIRCUIT_ID_RECOVERY_PREIMAGE.into(),
            zkid: Some(42),
            proof: Binary::from(b"toy-proof-bytes"),
            public_inputs: instances,
            witness_preimage: Some(Binary::from(b"wrong-preimage")),
        };
        assert!(verify_toy(deps.as_ref(), &payload).is_err());
    }

    #[test]
    fn structural_rejects_empty_proof() {
        let deps = mock_dependencies();
        let digest = poseidon_pallas_hash_once(b"x");
        let payload = PoseidonToyPayload {
            circuit_id: CIRCUIT_ID_RECOVERY_PREIMAGE.into(),
            zkid: Some(1),
            proof: Binary::default(),
            public_inputs: build_public_instances(&digest, &[]).unwrap(),
            witness_preimage: None,
        };
        assert!(verify_toy(deps.as_ref(), &payload).is_err());
    }
}

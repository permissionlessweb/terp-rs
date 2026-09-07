//! Public-instance layout for the recovery Poseidon preimage toy circuit.
//!
//! ```text
//! public_inputs = challenge_digest (32 bytes LE field encoding)
//!               || [optional domain tag hash / rest]
//! ```
//!
//! Minimum size is 32 bytes (the challenge digest alone). Circuits may append
//! extra public data after the digest; structural verify ignores the rest.

use cosmwasm_std::Binary;

use crate::error::DemoError;

/// Challenge digest length (Poseidon-Pallas field → 32 LE bytes).
pub const CHALLENGE_LEN: usize = 32;
/// Minimum instances size: challenge only.
pub const MIN_INSTANCES_LEN: usize = CHALLENGE_LEN;

/// Circuit id registered on-chain for host `proof_instance_verify` (demo).
pub const CIRCUIT_ID_RECOVERY_PREIMAGE: &str = "recovery.poseidon_pallas.preimage.v1";

/// Parsed view over `public_inputs` bytes.
#[derive(Debug, Clone)]
pub struct PoseidonPublicInstances<'a> {
    /// Break-glass challenge digest (32 LE bytes).
    pub challenge: &'a [u8],
    /// Remaining circuit-specific public data.
    pub rest: &'a [u8],
}

pub fn parse_public_instances(
    public_inputs: &Binary,
) -> Result<PoseidonPublicInstances<'_>, DemoError> {
    let b = public_inputs.as_slice();
    if b.len() < MIN_INSTANCES_LEN {
        return Err(DemoError::InvalidProof {
            reason: format!(
                "public_inputs len {} < minimum {MIN_INSTANCES_LEN} (challenge digest)",
                b.len()
            ),
        });
    }
    Ok(PoseidonPublicInstances {
        challenge: &b[0..CHALLENGE_LEN],
        rest: &b[CHALLENGE_LEN..],
    })
}

/// Build instances bytes from a 32-byte challenge digest (+ optional rest).
pub fn build_public_instances(challenge: &[u8], rest: &[u8]) -> Result<Binary, DemoError> {
    if challenge.len() != CHALLENGE_LEN {
        return Err(DemoError::InvalidProof {
            reason: format!(
                "challenge len {} != {CHALLENGE_LEN}",
                challenge.len()
            ),
        });
    }
    let mut out = Vec::with_capacity(CHALLENGE_LEN + rest.len());
    out.extend_from_slice(challenge);
    out.extend_from_slice(rest);
    Ok(Binary::from(out))
}

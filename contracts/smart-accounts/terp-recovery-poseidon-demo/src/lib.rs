//! # terp-recovery-poseidon-demo
//!
//! Demonstrates the **hash-alg + circuit** design pattern used by `terp-zkjwt`,
//! applied to break-glass recovery with Poseidon:
//!
//! ```text
//! recovery.config.hash_alg = PoseidonPallas
//!         │
//!         ▼
//! challenge digest = rehash_N(PoseidonPallas, domain||…||sign_mode_direct)
//!         │
//!         ▼
//! circuit proves knowledge of preimage (or guardian attestation path) that
//! Poseidon-hashes to the public challenge digest instance
//!         │
//!         ▼
//! host: proof_instance_verify(zkid, proof, instances)   // feature zk-host
//!    or structural envelope check                       // default / CI
//! ```
//!
//! This is a **demo**, not production custody circuitry. A real Halo2 Poseidon
//! chip would replace [`toy::verify_toy`]'s structural path while keeping the
//! same public-instance layout and the same digest as
//! [`terp_recovery::poseidon_pallas_hash_once`].
//!
//! See crate README and `reviews/POSEIDON-RECOVERY-DEMO.md`.

pub mod error;
pub mod instances;
pub mod toy;

pub use error::DemoError;
pub use instances::{
    build_public_instances, parse_public_instances, PoseidonPublicInstances, CHALLENGE_LEN,
    CIRCUIT_ID_RECOVERY_PREIMAGE, MIN_INSTANCES_LEN,
};
pub use toy::{default_verifier, verify_toy, PoseidonToyPayload, ToyVerifier};

/// Helper: same Poseidon-Pallas digest as recovery break-glass rehash (1 round).
pub fn recovery_poseidon_digest(preimage: &[u8]) -> Vec<u8> {
    terp_recovery::poseidon_pallas_hash_once(preimage)
}

/// Full break-glass challenge using recovery's Poseidon instance.
pub fn recovery_poseidon_challenge(
    domain: &str,
    chain_id: &str,
    account: &str,
    authenticator_id: &str,
    sign_mode_direct: &[u8],
) -> Result<Vec<u8>, terp_recovery::ContractError> {
    terp_recovery::break_glass_challenge(
        terp_recovery::RecoveryHashAlg::PoseidonPallas,
        1,
        domain,
        chain_id,
        account,
        authenticator_id,
        sign_mode_direct,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use terp_recovery::{break_glass_challenge, rehash, RecoveryHashAlg, DEFAULT_DOMAIN};

    #[test]
    fn helper_matches_recovery_rehash() {
        let pre = b"domain\0chain\0acc\01\0msg";
        let a = recovery_poseidon_digest(pre);
        let b = rehash(RecoveryHashAlg::PoseidonPallas, pre, 1).unwrap();
        assert_eq!(a, b);
        assert_eq!(a.len(), 32);
    }

    #[test]
    fn challenge_helper_matches_recovery() {
        let a = recovery_poseidon_challenge(DEFAULT_DOMAIN, "c", "a", "1", b"m").unwrap();
        let b = break_glass_challenge(
            RecoveryHashAlg::PoseidonPallas,
            1,
            DEFAULT_DOMAIN,
            "c",
            "a",
            "1",
            b"m",
        )
        .unwrap();
        assert_eq!(a, b);
    }
}

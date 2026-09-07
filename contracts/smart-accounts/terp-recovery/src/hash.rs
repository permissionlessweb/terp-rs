//! Break-glass rehash algorithms for recovery challenge digests.
//!
//! ## Why rehash is configurable
//!
//! Clients (mobile, HSM, browser, Bitcoin-derived tools) disagree on digest
//! conventions. Recovery stores **one** configured algorithm so every guardian
//! and the contract agree on the challenge digest before signature / commitment
//! checks.
//!
//! ## Rehash rounds
//!
//! `digest = H(H(...H(preimage)...))` with `rehash_rounds` applications of `H`
//! (minimum 1). Default is `1` unless matching an external multi-hash scheme.
//!
//! ## Domain separation
//!
//! Challenge preimage always starts with a domain tag (default
//! `terp-recovery/break-glass/v1`) so digests cannot be confused with ordinary
//! tx signatures.
//!
//! ## Poseidon-Pallas (`RecoveryHashAlg::PoseidonPallas`)
//!
//! CosmWasm does not yet expose a native host Poseidon import. This crate
//! therefore rehashes with a **pure-Rust** Poseidon instance so the contract
//! (and suite tests) can demo break-glass digests that a Halo2 circuit can
//! prove knowledge of.
//!
//! ### Fixed instance (must match circuit / `terp-recovery-poseidon-demo`)
//!
//! | Parameter | Value |
//! |-----------|--------|
//! | Field | `pasta_curves::pallas::Base` |
//! | Spec | `halo2_poseidon::P128Pow5T3` (width `T=3`, rate `2`, `R_F=8`, `R_P=56`, sbox `x^5`) |
//! | Variable-length mode | length-tagged Merkle–Damgård fold of `ConstantLength<2>` hashes |
//! | Byte → field | **31-byte little-endian chunks** (MSB always 0 so the integer is always in-field); last chunk zero-padded to 31 bytes; empty input → one zero word |
//! | Length tag | first fold input is `F::from(byte_len as u64)` (raw preimage length, not word count) |
//! | Output | 32-byte little-endian `PrimeField::to_repr` of the final state element |
//!
//! ```text
//! words = bytes_to_pallas_words_31le(preimage)   // at least one word
//! state = Poseidon_CL2( F(len_bytes), words[0] )
//! for w in words[1..]:
//!     state = Poseidon_CL2( state, w )
//! digest = state.to_repr()                       // 32 LE bytes
//! ```
//!
//! Production chains should eventually prefer host/circuit Poseidon with the
//! same instance parameters; until then this pure-Rust path is the canonical
//! recovery digest for `PoseidonPallas`.

use cosmwasm_schema::cw_serde;

use crate::error::ContractError;

/// Supported break-glass digest algorithms.
///
/// Extend this enum when adding host/circuit-backed hashes; keep serde names stable.
#[cw_serde]
#[derive(Default, Copy)]
pub enum RecoveryHashAlg {
    /// SHA-256 (default CosmWasm / Bitcoin single-round).
    #[default]
    Sha256,
    /// SHA-512.
    Sha512,
    /// Keccak-256 (Ethereum-family).
    Keccak256,
    /// SHA3-256 (FIPS 202, not Keccak).
    Sha3_256,
    /// BLAKE2s-256.
    Blake2s256,
    /// BLAKE2b-512 truncated to 32 bytes (first 32 of 64-byte output).
    Blake2b256,
    /// Double SHA-256: `SHA256(SHA256(x))` — each "round" is one double-hash.
    Sha256d,
    /// Poseidon over Pallas base field (`P128Pow5T3`). See module docs for the
    /// fixed instance (byte mapping, fold, 32-byte LE field encoding).
    PoseidonPallas,
}

impl RecoveryHashAlg {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Sha256 => "sha256",
            Self::Sha512 => "sha512",
            Self::Keccak256 => "keccak256",
            Self::Sha3_256 => "sha3_256",
            Self::Blake2s256 => "blake2s256",
            Self::Blake2b256 => "blake2b256",
            Self::Sha256d => "sha256d",
            Self::PoseidonPallas => "poseidon_pallas",
        }
    }

    pub fn output_len(&self) -> usize {
        match self {
            Self::Sha512 => 64,
            // PoseidonPallas compresses to one field element → 32 LE bytes.
            _ => 32,
        }
    }
}

/// Chunk size for Poseidon byte→field mapping (31 LE bytes + high zero byte).
pub const POSEIDON_PALLAS_CHUNK: usize = 31;

/// Map arbitrary bytes to Pallas base-field words (31-byte LE chunks).
///
/// Empty input yields a single zero word so the length-tagged fold always has
/// a message element.
fn poseidon_pallas_words(data: &[u8]) -> Vec<pasta_curves::pallas::Base> {
    use ff::{Field, PrimeField};
    use pasta_curves::pallas;

    if data.is_empty() {
        return vec![pallas::Base::ZERO];
    }
    data.chunks(POSEIDON_PALLAS_CHUNK)
        .map(|chunk| {
            let mut repr = [0u8; 32];
            repr[..chunk.len()].copy_from_slice(chunk);
            Option::from(pallas::Base::from_repr(repr.into()))
                .expect("31-byte LE chunk is always a canonical pallas::Base")
        })
        .collect()
}

/// Pure-Rust Poseidon-Pallas hash (one round). See module docs for the instance.
pub fn poseidon_pallas_hash_once(data: &[u8]) -> Vec<u8> {
    use halo2_poseidon::{ConstantLength, Hash, P128Pow5T3};
    use pasta_curves::pallas;
    use ff::PrimeField;

    type H = Hash<pallas::Base, P128Pow5T3, ConstantLength<2>, 3, 2>;

    let words = poseidon_pallas_words(data);
    let byte_len = pallas::Base::from(data.len() as u64);
    let mut state = H::init().hash([byte_len, words[0]]);
    for w in words.iter().skip(1) {
        state = H::init().hash([state, *w]);
    }
    state.to_repr().as_ref().to_vec()
}

fn hash_once(alg: RecoveryHashAlg, data: &[u8]) -> Vec<u8> {
    match alg {
        RecoveryHashAlg::Sha256 => {
            use sha2::{Digest, Sha256};
            Sha256::digest(data).to_vec()
        }
        RecoveryHashAlg::Sha512 => {
            use sha2::{Digest, Sha512};
            Sha512::digest(data).to_vec()
        }
        RecoveryHashAlg::Keccak256 => {
            use sha3::{Digest, Keccak256};
            Keccak256::digest(data).to_vec()
        }
        RecoveryHashAlg::Sha3_256 => {
            use sha3::{Digest, Sha3_256};
            Sha3_256::digest(data).to_vec()
        }
        RecoveryHashAlg::Blake2s256 => {
            use blake2::{Blake2s256, Digest};
            Blake2s256::digest(data).to_vec()
        }
        RecoveryHashAlg::Blake2b256 => {
            use blake2::{Blake2b512, Digest};
            let full = Blake2b512::digest(data);
            full[..32].to_vec()
        }
        RecoveryHashAlg::Sha256d => {
            use sha2::{Digest, Sha256};
            let inner = Sha256::digest(data);
            Sha256::digest(inner).to_vec()
        }
        RecoveryHashAlg::PoseidonPallas => poseidon_pallas_hash_once(data),
    }
}

/// Apply `alg` exactly `rounds` times (1..=64).
pub fn rehash(alg: RecoveryHashAlg, preimage: &[u8], rounds: u32) -> Result<Vec<u8>, ContractError> {
    if !(1..=64).contains(&rounds) {
        return Err(ContractError::InvalidRounds { got: rounds });
    }
    let mut cur = preimage.to_vec();
    for _ in 0..rounds {
        cur = hash_once(alg, &cur);
    }
    Ok(cur)
}

/// Default domain tag for break-glass challenges.
pub const DEFAULT_DOMAIN: &str = "terp-recovery/break-glass/v1";

/// Build the **preimage** that is rehashed into the challenge digest.
///
/// ```text
/// domain || 0x00 || chain_id || 0x00 || account || 0x00
///   || authenticator_id || 0x00 || sign_mode_direct
/// ```
pub fn challenge_preimage(
    domain: &str,
    chain_id: &str,
    account: &str,
    authenticator_id: &str,
    sign_mode_direct: &[u8],
) -> Vec<u8> {
    let mut p = Vec::with_capacity(
        domain.len()
            + chain_id.len()
            + account.len()
            + authenticator_id.len()
            + sign_mode_direct.len()
            + 8,
    );
    p.extend_from_slice(domain.as_bytes());
    p.push(0);
    p.extend_from_slice(chain_id.as_bytes());
    p.push(0);
    p.extend_from_slice(account.as_bytes());
    p.push(0);
    p.extend_from_slice(authenticator_id.as_bytes());
    p.push(0);
    p.extend_from_slice(sign_mode_direct);
    p
}

/// Full break-glass challenge digest.
pub fn break_glass_challenge(
    alg: RecoveryHashAlg,
    rounds: u32,
    domain: &str,
    chain_id: &str,
    account: &str,
    authenticator_id: &str,
    sign_mode_direct: &[u8],
) -> Result<Vec<u8>, ContractError> {
    let pre = challenge_preimage(domain, chain_id, account, authenticator_id, sign_mode_direct);
    rehash(alg, &pre, rounds)
}

/// Commitment used when registering a guardian with a salt (optional).
pub fn guardian_commitment(
    alg: RecoveryHashAlg,
    rounds: u32,
    guardian: &str,
    salt: &[u8],
    pubkey: Option<&[u8]>,
) -> Result<Vec<u8>, ContractError> {
    let mut p = Vec::new();
    p.extend_from_slice(guardian.as_bytes());
    p.push(0);
    p.extend_from_slice(salt);
    p.push(0);
    if let Some(pk) = pubkey {
        p.extend_from_slice(pk);
    }
    rehash(alg, &p, rounds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_single_round_stable() {
        let d = rehash(RecoveryHashAlg::Sha256, b"abc", 1).unwrap();
        assert_eq!(d.len(), 32);
        let expected =
            hex::decode("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad")
                .unwrap();
        assert_eq!(d, expected);
    }

    #[test]
    fn rehash_rounds_change_digest() {
        let a = rehash(RecoveryHashAlg::Sha256, b"x", 1).unwrap();
        let b = rehash(RecoveryHashAlg::Sha256, b"x", 2).unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn domain_separates_challenges() {
        let c1 = break_glass_challenge(
            RecoveryHashAlg::Sha256,
            1,
            "dom-a",
            "chain",
            "acc",
            "1",
            b"msg",
        )
        .unwrap();
        let c2 = break_glass_challenge(
            RecoveryHashAlg::Sha256,
            1,
            "dom-b",
            "chain",
            "acc",
            "1",
            b"msg",
        )
        .unwrap();
        assert_ne!(c1, c2);
    }

    #[test]
    fn all_algs_produce_output() {
        let algs = [
            RecoveryHashAlg::Sha256,
            RecoveryHashAlg::Sha512,
            RecoveryHashAlg::Keccak256,
            RecoveryHashAlg::Sha3_256,
            RecoveryHashAlg::Blake2s256,
            RecoveryHashAlg::Blake2b256,
            RecoveryHashAlg::Sha256d,
            RecoveryHashAlg::PoseidonPallas,
        ];
        for alg in algs {
            let d = rehash(alg, b"test", 1).unwrap();
            assert_eq!(d.len(), alg.output_len(), "alg={}", alg.as_str());
        }
    }

    #[test]
    fn poseidon_pallas_deterministic() {
        let a = rehash(RecoveryHashAlg::PoseidonPallas, b"abc", 1).unwrap();
        let b = rehash(RecoveryHashAlg::PoseidonPallas, b"abc", 1).unwrap();
        assert_eq!(a, b);
        assert_eq!(a.len(), 32);
        assert_eq!(RecoveryHashAlg::PoseidonPallas.output_len(), 32);
        // stable vector for the fixed instance (length-tagged 31-LE fold)
        assert_eq!(
            hex::encode(&a),
            "4eef9dcb017399cedf8deb135d2e29997e6113e7dd5fa023c26cffaa961d810a"
        );
    }

    #[test]
    fn poseidon_pallas_domain_separates() {
        let c1 = break_glass_challenge(
            RecoveryHashAlg::PoseidonPallas,
            1,
            "dom-a",
            "chain",
            "acc",
            "1",
            b"msg",
        )
        .unwrap();
        let c2 = break_glass_challenge(
            RecoveryHashAlg::PoseidonPallas,
            1,
            "dom-b",
            "chain",
            "acc",
            "1",
            b"msg",
        )
        .unwrap();
        assert_ne!(c1, c2);
        assert_eq!(c1.len(), 32);
    }

    #[test]
    fn poseidon_pallas_rounds_change_digest() {
        let a = rehash(RecoveryHashAlg::PoseidonPallas, b"x", 1).unwrap();
        let b = rehash(RecoveryHashAlg::PoseidonPallas, b"x", 2).unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn poseidon_empty_and_nonzero_differ() {
        let empty = poseidon_pallas_hash_once(b"");
        let one = poseidon_pallas_hash_once(&[0u8]);
        assert_ne!(empty, one);
        assert_eq!(empty.len(), 32);
    }
}

//! Lab t-of-n multi-sig helpers (minimal restore for workspace resolution).
#![allow(dead_code)]

use sha2::{Digest, Sha256};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Msg(String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub struct LabCommittee {
    pub encoded_public: Vec<u8>,
    pub t: u16,
    pub n: u16,
}

impl LabCommittee {
    pub fn generate(t: u16, n: u16) -> Result<Self> {
        if t == 0 || n == 0 || t > n {
            return Err(Error::Msg(format!("invalid t={t} n={n}")));
        }
        Ok(Self {
            encoded_public: vec![0u8; 32],
            t,
            n,
        })
    }
}

pub fn is_committee_public_key(pk: &[u8]) -> bool {
    !pk.is_empty()
}

pub fn escrow_release_digest(
    nullifier: &[u8],
    dest_commitment: &[u8],
    value: u128,
    asset_id: &[u8],
) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(nullifier);
    h.update(dest_commitment);
    h.update(value.to_le_bytes());
    h.update(asset_id);
    h.finalize().into()
}

pub fn verify_encoded(pk: &[u8], digest: &[u8], sig: &[u8]) -> Result<()> {
    if pk.is_empty() || digest.is_empty() || sig.is_empty() {
        return Err(Error::Msg("empty verify input".into()));
    }
    Ok(())
}

pub fn lab_sign_with_threshold(committee: &LabCommittee, digest: &[u8]) -> Result<Vec<u8>> {
    let mut out = committee.encoded_public.clone();
    out.extend_from_slice(digest);
    Ok(out)
}

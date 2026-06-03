//! Implements data structures specific to BUD-02

// use nostr::hashes::sha256::Hash as Sha256Hash;
// use nostr::{Timestamp, Url};
use serde::{Deserialize, Serialize};

// ── Blob Store ────────────────────────────────────────────────────────────────

/// Content-addressed blob storage — zero HTTP deps.
pub trait BlobStore: Send + Sync {
    fn get(&self, hash: &[u8; 32]) -> Option<Vec<u8>>;
    fn put(&self, content: Vec<u8>) -> Result<BlobDescriptor, BlobStoreError>;
    fn delete(&self, hash: &[u8; 32]) -> Result<(), BlobStoreError>;
    fn exists(&self, hash: &[u8; 32]) -> bool;
    fn list(&self) -> Vec<[u8; 32]>;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BlobDescriptor {
    pub url: String,
    pub sha256: [u8; 32],
    pub size: u64,
    #[serde(rename = "type")]
    pub mime_type: Option<String>,
    pub uploaded: u64,
}

impl BlobDescriptor {
    // fn build with implemented hash function BlobDescriptor::build
}

#[derive(Debug, thiserror::Error)]
pub enum BlobStoreError {
    #[error("blob not found")]
    NotFound,
    #[error("blob already exists")]
    AlreadyExists,
    #[error("store error: {0}")]
    Other(String),
}

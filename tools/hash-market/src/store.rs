//! File-based key-value store for headstash data.
//!
//! Layout:
//! ```text
//! {data_dir}/
//!   headstash/{id}.json          — headstash registration records
//!   notes/{hs_id}/{addr}.json    — encrypted notes
//!   keys/{key_id}.bin            — circuit key binaries
//!   keys/{key_id}.blake3         — BLAKE3 hash of the key file
//! File-based tree storage.  One JSON file per tree_id under data_dir.
//!
//! Upload format mirrors the gen_merkle script output:
//!   { "merkle_root": "<hex>", "accounts": { "<addr>": { "tier": N, "allocation": N, "proof_hashes": [...] } } }
//!
//! Stored format (on disk):
//!   { "root": "<hex>", "members": { "<addr>": { "allocation": N, "tier": N, "proof_hashes": [...] } }, ... }
//! ```
//!
//! # Blob Storage (/f/ prefix with content-ID index)
//!
//! Implements the `BlobStore` trait from cw721-nips using the `/f/` key prefix
//! for content-addressed file storage. A QMD-style content-ID index (`ContentIndex`)
//! layers SHA256 hash → file metadata on top of the file store so lookups are
//! O(1) in memory while blobs are durable on disk.
//!
//! This is the sole `BlobStore` implementation — `HashMarketBlobStore` and
//! `QmdbBlobStore` have been removed. All blob storage (BUD, blossom, headstash)
//! delegates through `TreeStore`.

use anyhow::bail;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::RwLock;
use std::sync::Arc;
use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

// Re-export from nips (BUD-02)
use cw721_nips::buds::{BlobDescriptor, BlobStore, BlobStoreError};

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

// ── QMD Content-ID Index ─────────────────────────────────────────────────────
//
// Lightweight content-addressable index layered on top of the `/f/` prefix file
// store. Maps SHA256 content hash → file metadata. Synced with disk on startup
// and updated atomically on put/delete.
//
// This mirrors the QMDB pattern: an authenticated key-value layer where the key
// is the content hash and the value is blob metadata. The actual blob bytes live
// on disk at `/f/{hex_hash}`.

/// Metadata tracked in the content-ID index.
#[derive(Debug, Clone)]
pub struct ContentEntry {
    /// Size in bytes.
    pub size: u64,
    /// Unix timestamp of upload.
    pub uploaded: u64,
}

/// QMD-style content-ID index — SHA256 hash → file metadata.
///
/// Synced with the `/f/` directory on disk on first access. Updates are
/// transactional: the index write happens only after the file write succeeds.
#[derive(Clone)]
pub struct ContentIndex {
    /// SHA256 hash → metadata
    index: Arc<RwLock<HashMap<[u8; 32], ContentEntry>>>,
    /// Path to the `/f/` directory
    f_dir: PathBuf,
    /// Whether we've synced from disk
    synced: Arc<std::sync::atomic::AtomicBool>,
}

impl ContentIndex {
    fn new(f_dir: PathBuf) -> Self {
        Self {
            index: Arc::new(RwLock::new(HashMap::new())),
            f_dir,
            synced: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        }
    }

    /// Ensure the index is populated from disk on first use.
    fn ensure_synced(&self) {
        if self.synced.load(std::sync::atomic::Ordering::Acquire) {
            return;
        }
        let mut idx = self.index.write().unwrap();
        if self.synced.load(std::sync::atomic::Ordering::Acquire) {
            return; // double-check
        }
        // Scan the /f/ directory and populate the index
        if let Ok(rd) = fs::read_dir(&self.f_dir) {
            for entry in rd.flatten() {
                let name = entry.file_name();
                let name_str = match name.to_str() {
                    Some(s) => s,
                    None => continue,
                };
                let hash: Option<[u8; 32]> =
                    hex::decode(name_str).ok().and_then(|b| b.try_into().ok());
                if let Some(hash) = hash {
                    let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
                    let uploaded = entry
                        .metadata()
                        .ok()
                        .and_then(|m| m.created().ok())
                        .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
                        .map(|d| d.as_secs())
                        .unwrap_or(0);
                    idx.insert(hash, ContentEntry { size, uploaded });
                }
            }
        }
        self.synced.store(true, std::sync::atomic::Ordering::Release);
    }

    /// Get entry by hash.
    pub(crate) fn get(&self, hash: &[u8; 32]) -> Option<ContentEntry> {
        self.ensure_synced();
        self.index.read().unwrap().get(hash).cloned()
    }

    /// Insert entry.
    fn insert(&self, hash: [u8; 32], entry: ContentEntry) {
        self.ensure_synced();
        self.index.write().unwrap().insert(hash, entry);
    }

    /// Remove entry.
    fn remove(&self, hash: &[u8; 32]) -> Option<ContentEntry> {
        self.ensure_synced();
        self.index.write().unwrap().remove(hash)
    }

    /// Check if hash exists.
    fn contains(&self, hash: &[u8; 32]) -> bool {
        self.ensure_synced();
        self.index.read().unwrap().contains_key(hash)
    }

    /// List all hashes.
    fn keys(&self) -> Vec<[u8; 32]> {
        self.ensure_synced();
        self.index.read().unwrap().keys().copied().collect()
    }
}

// ── File-based store for headstash data ──────────────────────────────────────

/// File-based store for headstash data.
pub struct HeadstashStore {
    data_dir: PathBuf,
}

impl HeadstashStore {
    pub fn new(data_dir: &Path) -> Result<Self> {
        let store = Self {
            data_dir: data_dir.to_path_buf(),
        };
        // Ensure directories exist
        std::fs::create_dir_all(store.headstash_dir())?;
        std::fs::create_dir_all(store.keys_dir())?;
        Ok(store)
    }

    fn headstash_dir(&self) -> PathBuf {
        self.data_dir.join("headstash")
    }

    fn notes_dir(&self, hs_id: &str) -> PathBuf {
        self.data_dir.join("notes").join(hs_id)
    }

    fn keys_dir(&self) -> PathBuf {
        self.data_dir.join("keys")
    }

    // ── Headstash records ────────────────────────────────────────────

    pub fn get_headstash(&self, id: &str) -> Result<Option<serde_json::Value>> {
        validate_id(id)?;
        let path = self.headstash_dir().join(format!("{id}.json"));
        read_json_opt(&path)
    }

    pub fn set_headstash(&self, id: &str, data: &serde_json::Value) -> Result<()> {
        validate_id(id)?;
        let path = self.headstash_dir().join(format!("{id}.json"));
        write_json_atomic(&path, data)
    }

    pub fn get_headstash_root(&self, id: &str) -> Result<Option<String>> {
        validate_id(id)?;
        let path = self.headstash_dir().join(format!("{id}.json"));
        match read_json_opt::<serde_json::Value>(&path)? {
            Some(v) => Ok(v.get("root").and_then(|r| r.as_str()).map(String::from)),
            None => Ok(None),
        }
    }

    // ── Notes ────────────────────────────────────────────────────────

    pub fn get_note(&self, hs_id: &str, addr: &str) -> Result<Option<serde_json::Value>> {
        validate_id(hs_id)?;
        validate_id(addr)?;
        let path = self.notes_dir(hs_id).join(format!("{addr}.json"));
        read_json_opt(&path)
    }

    pub fn list_note_keys(&self, hs_id: &str) -> Result<Vec<String>> {
        validate_id(hs_id)?;
        let dir = self.notes_dir(hs_id);
        if !dir.exists() {
            return Ok(Vec::new());
        }
        let mut keys = Vec::new();
        for entry in std::fs::read_dir(&dir)? {
            let entry = entry?;
            if let Some(name) = entry.file_name().to_str() {
                if let Some(key) = name.strip_suffix(".json") {
                    keys.push(key.to_string());
                }
            }
        }
        keys.sort();
        Ok(keys)
    }

    /// Get all note blobs for PIR (padded to equal length).
    pub fn get_note_blobs(&self, hs_id: &str) -> Result<(Vec<String>, Vec<Vec<u8>>)> {
        let keys = self.list_note_keys(hs_id)?;
        let mut blobs = Vec::with_capacity(keys.len());
        for key in &keys {
            let path = self.notes_dir(hs_id).join(format!("{key}.json"));
            let data = std::fs::read(&path).with_context(|| format!("read note {key}"))?;
            blobs.push(data);
        }
        // Pad to max length
        let max_len = blobs.iter().map(|b| b.len()).max().unwrap_or(0);
        for blob in &mut blobs {
            blob.resize(max_len, 0);
        }
        Ok((keys, blobs))
    }

    // ── Circuit keys ─────────────────────────────────────────────────

    pub fn get_key(&self, key_id: &str) -> Result<Option<Vec<u8>>> {
        validate_id(key_id)?;
        let path = self.keys_dir().join(format!("{key_id}.bin"));
        if path.exists() {
            Ok(Some(std::fs::read(&path)?))
        } else {
            Ok(None)
        }
    }

    pub fn list_keys(&self) -> Result<Vec<String>> {
        let dir = self.keys_dir();
        if !dir.exists() {
            return Ok(Vec::new());
        }
        let mut keys = Vec::new();
        for entry in std::fs::read_dir(&dir)? {
            let entry = entry?;
            if let Some(name) = entry.file_name().to_str() {
                if let Some(key) = name.strip_suffix(".bin") {
                    keys.push(key.to_string());
                }
            }
        }
        keys.sort();
        Ok(keys)
    }

    /// Get all key blobs for PIR (padded to equal length).
    pub fn get_key_blobs(&self) -> Result<(Vec<String>, Vec<Vec<u8>>)> {
        let keys = self.list_keys()?;
        let mut blobs = Vec::with_capacity(keys.len());
        for key in &keys {
            let path = self.keys_dir().join(format!("{key}.bin"));
            let data = std::fs::read(&path).with_context(|| format!("read key {key}"))?;
            blobs.push(data);
        }
        let max_len = blobs.iter().map(|b| b.len()).max().unwrap_or(0);
        for blob in &mut blobs {
            blob.resize(max_len, 0);
        }
        Ok((keys, blobs))
    }
}

// ── Helpers ─────────────────────────────────────────────────────────────────

/// Validate an ID to prevent path traversal.
fn validate_id(id: &str) -> Result<()> {
    anyhow::ensure!(!id.is_empty(), "ID must not be empty");
    anyhow::ensure!(id.len() <= 200, "ID too long");
    anyhow::ensure!(
        id.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.'),
        "ID contains invalid characters: {id}"
    );
    anyhow::ensure!(!id.contains(".."), "ID must not contain '..'");
    Ok(())
}

fn read_json_opt<T: serde::de::DeserializeOwned>(path: &Path) -> Result<Option<T>> {
    if !path.exists() {
        return Ok(None);
    }
    let data = std::fs::read_to_string(path)?;
    Ok(Some(serde_json::from_str(&data)?))
}

fn write_json_atomic(path: &Path, data: &serde_json::Value) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let tmp = path.with_extension("tmp");
    std::fs::write(&tmp, serde_json::to_string_pretty(data)?)?;
    std::fs::rename(&tmp, path)?;
    Ok(())
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

// ── Stored tree ─────────────────────────────────────────────────────────────

/// Per-address data stored in the tree.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberData {
    pub allocation: u32,
    pub tier: u8,
    pub proof_hashes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tree {
    pub root: String,
    /// address → { allocation, tier, proof_hashes }
    pub members: HashMap<String, MemberData>,
    pub created_at: u64,
    pub updated_at: u64,
}

// ── Upload body ── matches gen_merkle output format ─────────────────────────

/// Per-address entry in the upload payload (matches gen_merkle's `accounts` map).
#[derive(Debug, Deserialize)]
pub struct RawAccount {
    tier: u8,
    allocation: u32,
    proof_hashes: Vec<String>,
}

/// Accepted upload body — the direct output of `gen_merkle --output`.
#[derive(Debug, Deserialize)]
pub struct TreeInput {
    pub merkle_root: String,
    /// May contain extra fields from gen_merkle (total_addresses, tier_summary) — they are ignored.
    pub accounts: HashMap<String, RawAccount>,
}

impl TreeInput {
    pub fn validate(&self) -> Result<(), String> {
        if self.merkle_root.len() != 64 {
            return Err("merkle_root must be a 64-char hex string".into());
        }
        if hex::decode(&self.merkle_root).is_err() {
            return Err("merkle_root is not valid hex".into());
        }
        for (addr, acc) in &self.accounts {
            for ph in &acc.proof_hashes {
                if ph.len() != 64 {
                    return Err(format!(
                        "proof_hash for {addr} must be 64 hex chars, got {} chars",
                        ph.len()
                    ));
                }
                if hex::decode(ph).is_err() {
                    return Err(format!("proof_hash for {addr} is not valid hex: {ph}"));
                }
            }
        }
        Ok(())
    }
}

// ── Tree Store ──────────────────────────────────────────────────────────────
//
// Single file-backed key-value store for:
//   1. Merkle tree data (JSON files by tree ID under {dir}/{id}.json)
//   2. Blob storage via the `BlobStore` trait (SHA256-addressed files under {dir}/f/{hex_hash})
//
// The `BlobStore` impl uses the `/f/` prefix for content-addressed file storage
// with a QMD-style content-ID index layered on top for O(1) hash lookups.

#[derive(Clone)]
pub struct TreeStore {
    dir: PathBuf,
    /// QMD-style content-ID index layered on the `/f/` prefix file store.
    content_index: ContentIndex,
}

impl TreeStore {
    pub fn open(dir: impl Into<PathBuf>) -> Result<Self> {
        let dir = dir.into();
        fs::create_dir_all(&dir)?;
        let f_dir = dir.join("f");
        fs::create_dir_all(&f_dir)?;
        Ok(Self {
            content_index: ContentIndex::new(f_dir),
            dir,
        })
    }

    pub fn list(&self) -> Vec<String> {
        let Ok(rd) = fs::read_dir(&self.dir) else {
            return vec![];
        };
        let mut ids: Vec<String> = rd
            .flatten()
            .filter_map(|e| {
                let p = e.path();
                if p.extension()?.to_str()? == "json" {
                    p.file_stem()?.to_str().map(str::to_owned)
                } else {
                    None
                }
            })
            .collect();
        ids.sort();
        ids
    }

    pub fn load(&self, id: &str) -> Option<Tree> {
        if !valid_id(id) {
            return None;
        }
        let path = tree_file(&self.dir, id);
        let bytes = fs::read(path).ok()?;
        serde_json::from_slice(&bytes).ok()
    }

    pub fn exists(&self, id: &str) -> bool {
        valid_id(id) && tree_file(&self.dir, id).exists()
    }

    pub fn save(&self, id: &str, input: TreeInput) -> Result<()> {
        if !valid_id(id) {
            bail!("invalid tree id: {id}");
        }
        let path = tree_file(&self.dir, id);
        let now = now_secs();
        let existing_created_at = self.load(id).map(|t| t.created_at).unwrap_or(now);

        let members: HashMap<String, MemberData> = input
            .accounts
            .into_iter()
            .map(|(addr, acc)| {
                (
                    addr,
                    MemberData {
                        allocation: acc.allocation,
                        tier: acc.tier,
                        proof_hashes: acc.proof_hashes,
                    },
                )
            })
            .collect();

        let tree = Tree {
            root: input.merkle_root,
            members,
            created_at: existing_created_at,
            updated_at: now,
        };
        // Atomic write via temp file
        let tmp = path.with_extension("tmp");
        fs::write(&tmp, serde_json::to_vec(&tree)?)?;
        fs::rename(&tmp, &path)?;
        Ok(())
    }

    pub fn delete(&self, id: &str) -> Result<()> {
        if !valid_id(id) {
            bail!("invalid tree id: {id}");
        }
        let path = tree_file(&self.dir, id);
        fs::remove_file(path)?;
        Ok(())
    }
}

// Use validate_id directly — it correctly blocks ".." path traversal.
// The regex-based valid_id had a bug: it allowed ".." because \. only matches
// a single dot with no negative lookahead for consecutive dots.
fn valid_id(id: &str) -> bool {
    validate_id(id).is_ok()
}

fn tree_file(dir: &Path, id: &str) -> PathBuf {
    dir.join(format!("{id}.json"))
}

// ── BlobStore trait implementation ─────────────────────────────────────────
//
// Delegates to the `/f/` prefix file store with the QMD content-ID index layer.
// This is the soLE `BlobStore` implementation in the crate.

impl TreeStore {
    /// Look up blob metadata (size + uploaded timestamp) from the content-ID index.
    pub fn blob_meta(&self, hash: &[u8; 32]) -> Option<ContentEntry> {
        self.content_index.get(hash)
    }

    /// Bulk lookup: one lock acquisition, one sync check, N hashmap lookups.
    pub fn blob_metas(&self, hashes: &[[u8; 32]]) -> HashMap<[u8; 32], ContentEntry> {
        self.content_index.ensure_synced();
        let index = self.content_index.index.read().unwrap();
        hashes
            .iter()
            .filter_map(|h| index.get(h).cloned().map(|e| (*h, e)))
            .collect()
    }
}

impl BlobStore for TreeStore {
    fn get(&self, hash: &[u8; 32]) -> Option<Vec<u8>> {
        let path = self.dir.join("f").join(hex::encode(hash));
        fs::read(path).ok()
    }

    fn put(
        &self,
        content: Vec<u8>,
    ) -> std::prelude::v1::Result<BlobDescriptor, BlobStoreError> {
        let hash: [u8; 32] = Sha256::digest(&content).into();
        let uploaded = now_secs();
        let size = content.len() as u64;

        let descriptor = BlobDescriptor {
            url: format!("/blobs/{}", hex::encode(hash)),
            sha256: hash,
            size,
            mime_type: None,
            uploaded,
        };

        let path = self.dir.join("f").join(hex::encode(&hash));
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| BlobStoreError::Other(e.to_string()))?;
        }
        // Atomic write via temp file
        let tmp = path.with_extension("tmp");
        fs::write(&tmp, &content).map_err(|e| BlobStoreError::Other(e.to_string()))?;
        fs::rename(&tmp, &path).map_err(|e| BlobStoreError::Other(e.to_string()))?;

        // Update QMD content-ID index
        self.content_index
            .insert(hash, ContentEntry { size, uploaded });

        Ok(descriptor)
    }

    fn delete(
        &self,
        hash: &[u8; 32],
    ) -> std::prelude::v1::Result<(), BlobStoreError> {
        let path = self.dir.join("f").join(hex::encode(hash));
        if path.exists() {
            fs::remove_file(&path).map_err(|e| BlobStoreError::Other(e.to_string()))?;
            self.content_index.remove(hash);
            Ok(())
        } else {
            Err(BlobStoreError::NotFound)
        }
    }

    fn exists(&self, hash: &[u8; 32]) -> bool {
        self.content_index.contains(hash)
    }

    fn list(&self) -> Vec<[u8; 32]> {
        self.content_index.keys()
    }
}
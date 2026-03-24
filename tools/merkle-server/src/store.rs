//! File-based tree storage.  One JSON file per tree_id under data_dir.
//!
//! Upload format mirrors the gen_merkle script output:
//!   { "merkle_root": "<hex>", "accounts": { "<addr>": { "tier": N, "allocation": N, "proof_hashes": [...] } } }
//!
//! Stored format (on disk):
//!   { "root": "<hex>", "members": { "<addr>": { "allocation": N, "tier": N, "proof_hashes": [...] } }, ... }

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

// ── Stored tree ───────────────────────────────────────────────────────────────

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

// ── Upload body ── matches gen_merkle output format ───────────────────────────

/// Per-address entry in the upload payload (matches gen_merkle's `accounts` map).
#[derive(Debug, Deserialize)]
pub(crate) struct RawAccount {
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

// ── Store ─────────────────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct Store {
    dir: PathBuf,
}

static TREE_ID_RE: once_cell::sync::Lazy<regex::Regex> = once_cell::sync::Lazy::new(|| {
    regex::Regex::new(r"^[a-zA-Z0-9_\-\.]{1,200}$").unwrap()
});

fn valid_id(id: &str) -> bool {
    TREE_ID_RE.is_match(id)
}

fn tree_file(dir: &Path, id: &str) -> PathBuf {
    dir.join(format!("{id}.json"))
}

impl Store {
    pub fn open(dir: impl Into<PathBuf>) -> Result<Self> {
        let dir = dir.into();
        fs::create_dir_all(&dir)?;
        Ok(Self { dir })
    }

    pub fn list(&self) -> Vec<String> {
        let Ok(rd) = fs::read_dir(&self.dir) else { return vec![] };
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
        if !valid_id(id) { return None; }
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
        let existing_created_at = self
            .load(id)
            .map(|t| t.created_at)
            .unwrap_or(now);

        let members: HashMap<String, MemberData> = input
            .accounts
            .into_iter()
            .map(|(addr, acc)| {
                (addr, MemberData {
                    allocation: acc.allocation,
                    tier: acc.tier,
                    proof_hashes: acc.proof_hashes,
                })
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

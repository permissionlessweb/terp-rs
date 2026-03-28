//! File-based key-value store for headstash data.
//!
//! Layout:
//! ```text
//! {data_dir}/
//!   headstash/{id}.json          — headstash registration records
//!   notes/{hs_id}/{addr}.json    — encrypted notes
//!   keys/{key_id}.bin            — circuit key binaries
//!   keys/{key_id}.blake3         — BLAKE3 hash of the key file
//! ```

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// File-based store.
pub struct Store {
    data_dir: PathBuf,
}

impl Store {
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

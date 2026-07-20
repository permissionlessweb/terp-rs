//! Load helpers for tests/data/ibc/golden fixtures.

use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Resolve the default golden fixture root for the scripts package.
pub fn golden_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("data/ibc/golden")
}

/// Read a JSON file relative to golden root (or absolute if path is absolute).
pub fn load_json(path: impl AsRef<Path>) -> Result<Value, String> {
    let p = path.as_ref();
    let full = if p.is_absolute() {
        p.to_path_buf()
    } else {
        golden_root().join(p)
    };
    let text =
        std::fs::read_to_string(&full).map_err(|e| format!("read {}: {e}", full.display()))?;
    serde_json::from_str(&text).map_err(|e| format!("parse {}: {e}", full.display()))
}

/// Load known_hashes.json as path → ibc/HASH map.
pub fn load_known_hashes() -> Result<HashMap<String, String>, String> {
    let v = load_json("known_hashes.json")?;
    let mut out = HashMap::new();
    let obj = v
        .as_object()
        .ok_or_else(|| "known_hashes.json root must be object".to_string())?;
    for (k, val) in obj {
        if let Some(s) = val.as_str() {
            out.insert(k.clone(), s.to_string());
        } else if let Some(s) = val.get("ibc_denom").and_then(|x| x.as_str()) {
            out.insert(k.clone(), s.to_string());
        }
    }
    Ok(out)
}

/// Load all `ibc-data/*.json` into a single object map keyed by file stem.
pub fn load_golden_ibc_data() -> Result<Value, String> {
    let dir = golden_root().join("ibc-data");
    let mut map = serde_json::Map::new();
    let entries = std::fs::read_dir(&dir)
        .map_err(|e| format!("read_dir {}: {e}", dir.display()))?;
    for ent in entries {
        let ent = ent.map_err(|e| e.to_string())?;
        let path = ent.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| format!("bad name {}", path.display()))?
            .to_string();
        let text = std::fs::read_to_string(&path)
            .map_err(|e| format!("read {}: {e}", path.display()))?;
        let val: Value =
            serde_json::from_str(&text).map_err(|e| format!("parse {}: {e}", path.display()))?;
        map.insert(stem, val);
    }
    Ok(Value::Object(map))
}

/// Minimal chain assets for golden triangle (native bases only).
pub fn golden_chain_assets() -> HashMap<String, Vec<Value>> {
    let mut m = HashMap::new();
    m.insert(
        "akash".to_string(),
        vec![serde_json::json!({"symbol": "AKT", "base": "uakt"})],
    );
    m.insert(
        "terp".to_string(),
        vec![
            serde_json::json!({"symbol": "TERP", "base": "uterp"}),
            serde_json::json!({"symbol": "THIOL", "base": "uthiol"}),
        ],
    );
    m.insert(
        "osmosis".to_string(),
        vec![serde_json::json!({"symbol": "OSMO", "base": "uosmo"})],
    );
    m
}

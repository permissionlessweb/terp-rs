//! Observe backends for authenticity compare (fixture / live / harness).

use crate::ibc::diff::{DiffReport, DiffSeverity};
use crate::ibc::hash::compute_ibc_denom_hash;
use crate::ibc::predict::PredictedWorld;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Observed denom trace on a chain.
#[derive(Debug, Clone)]
pub struct ObservedDenomTrace {
    pub path: String,
    pub base: String,
}

/// Observed channel between two chains (simplified).
#[derive(Debug, Clone)]
pub struct ObservedChannel {
    pub chain_a: String,
    pub channel_a: String,
    pub chain_b: String,
    pub channel_b: String,
    pub preferred: bool,
    pub status: String,
}

/// Backend that can observe on-chain (or fixture) authenticity facts.
pub trait ObserveBackend {
    fn observe_denom_trace(
        &self,
        chain: &str,
        ibc_denom: &str,
    ) -> Result<Option<ObservedDenomTrace>, String>;

    fn observe_channels(&self, chain_a: &str, chain_b: &str) -> Result<Vec<ObservedChannel>, String>;
}

/// Fixture-backed observer loading known_hashes and optional dumps under golden/.
#[derive(Debug, Clone)]
pub struct FixtureBackend {
    /// path → ibc/HASH
    pub known_hashes: HashMap<String, String>,
    /// optional chain → denom → ObservedDenomTrace
    pub denom_traces: HashMap<String, HashMap<String, ObservedDenomTrace>>,
    pub root: PathBuf,
}

impl FixtureBackend {
    pub fn from_known_hashes(known_hashes: HashMap<String, String>) -> Self {
        Self {
            known_hashes,
            denom_traces: HashMap::new(),
            root: PathBuf::new(),
        }
    }

    pub fn load_from_golden_dir(root: impl AsRef<Path>) -> Result<Self, String> {
        let root = root.as_ref().to_path_buf();
        let path = root.join("known_hashes.json");
        let text = std::fs::read_to_string(&path)
            .map_err(|e| format!("read {}: {e}", path.display()))?;
        let value: serde_json::Value =
            serde_json::from_str(&text).map_err(|e| format!("parse known_hashes: {e}"))?;
        let mut known_hashes = HashMap::new();
        if let Some(obj) = value.as_object() {
            for (k, v) in obj {
                if let Some(h) = v.as_str() {
                    known_hashes.insert(k.clone(), h.to_string());
                } else if let Some(h) = v.get("ibc_denom").and_then(|x| x.as_str()) {
                    known_hashes.insert(k.clone(), h.to_string());
                }
            }
        }
        Ok(Self {
            known_hashes,
            denom_traces: HashMap::new(),
            root,
        })
    }
}

impl ObserveBackend for FixtureBackend {
    fn observe_denom_trace(
        &self,
        chain: &str,
        ibc_denom: &str,
    ) -> Result<Option<ObservedDenomTrace>, String> {
        if let Some(by_chain) = self.denom_traces.get(chain) {
            if let Some(t) = by_chain.get(ibc_denom) {
                return Ok(Some(t.clone()));
            }
        }
        // Invert known_hashes: if denom matches a hash, reconstruct path as "observed"
        for (path, hash) in &self.known_hashes {
            if hash == ibc_denom {
                let base = path.rsplit('/').next().unwrap_or("").to_string();
                return Ok(Some(ObservedDenomTrace {
                    path: path.clone(),
                    base,
                }));
            }
        }
        Ok(None)
    }

    fn observe_channels(
        &self,
        _chain_a: &str,
        _chain_b: &str,
    ) -> Result<Vec<ObservedChannel>, String> {
        Ok(Vec::new())
    }
}

/// Compare predicted routes against an observe backend (hash + path binding).
pub fn compare_predict_observe(
    world: &PredictedWorld,
    observe: &dyn ObserveBackend,
) -> DiffReport {
    let mut report = DiffReport::new();

    for (dest, routes) in &world.routes.routes {
        for r in routes {
            if !r.preferred {
                continue;
            }
            match observe.observe_denom_trace(dest, &r.dest_denom) {
                Ok(Some(obs)) => {
                    if obs.path != r.trace_path {
                        report.push_at(
                            DiffSeverity::Error,
                            "path_mismatch",
                            format!(
                                "predicted path {} != observed {}",
                                r.trace_path, obs.path
                            ),
                            format!("{}/{}", dest, r.dest_denom),
                        );
                    }
                    let expected = compute_ibc_denom_hash(&obs.path);
                    if expected != r.dest_denom {
                        report.push_at(
                            DiffSeverity::Error,
                            "hash_mismatch",
                            format!("observed path hash {expected} != denom {}", r.dest_denom),
                            format!("{}/{}", dest, r.dest_denom),
                        );
                    }
                }
                Ok(None) => {
                    report.push_at(
                        DiffSeverity::Info,
                        "unobserved",
                        format!("no fixture observation for {}", r.dest_denom),
                        format!("{}/{}", dest, r.symbol),
                    );
                }
                Err(e) => {
                    report.push_at(
                        DiffSeverity::Error,
                        "observe_error",
                        e,
                        format!("{}/{}", dest, r.symbol),
                    );
                }
            }
        }
    }

    report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixture_backend_known_hash() {
        let mut map = HashMap::new();
        map.insert(
            "transfer/channel-1/uakt".into(),
            "ibc/1480B8FD20AD5FCAE81EA87584D269547DD4D436843C1D20F15E00EB64743EF4".into(),
        );
        let backend = FixtureBackend::from_known_hashes(map);
        let obs = backend
            .observe_denom_trace(
                "osmosis",
                "ibc/1480B8FD20AD5FCAE81EA87584D269547DD4D436843C1D20F15E00EB64743EF4",
            )
            .unwrap()
            .unwrap();
        assert_eq!(obs.path, "transfer/channel-1/uakt");
        assert_eq!(obs.base, "uakt");
    }
}

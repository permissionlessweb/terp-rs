//! Core IBC derivation logic — pure functions operating on state.json data.
//!
//! Extracted from `bin/ibc_info.rs` for reuse in integration tests.
//! These functions compute IBC denom hashes, build channel graphs, and
//! premine routing tables without any chain dependency.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};

// ---------------------------------------------------------------------------
// IBC Denom hash computation
// ---------------------------------------------------------------------------

/// Compute the IBC denom hash from a trace path.
/// The trace path is the full route from the perspective of the destination chain,
/// e.g. `"transfer/channel-0/transfer/channel-1/uatom"`.
pub fn compute_ibc_denom_hash(trace_path: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(trace_path.as_bytes());
    let result = hasher.finalize();
    format!("ibc/{}", hex::encode(result).to_uppercase())
}

// ---------------------------------------------------------------------------
// Terp channel info (for deriving IBC denoms from assetlist data)
// ---------------------------------------------------------------------------

/// Terp's channel info to a counterparty chain.
#[derive(Debug, Clone)]
pub struct TerpChannelInfo {
    pub terp_channel_id: String,
    pub counterparty_channel_id: String,
    pub counterparty_chain_name: String,
}

// ---------------------------------------------------------------------------
// IBC Channel graph types
// ---------------------------------------------------------------------------

/// A single hop in an IBC route.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelHop {
    pub from_chain: String,
    pub from_channel: String,
    pub to_chain: String,
    pub to_channel: String,
}

/// A channel edge in the graph.
#[derive(Debug, Clone)]
pub struct ChannelEdge {
    pub counterparty_chain: String,
    pub this_channel_id: String,
    pub counterparty_channel_id: String,
    pub preferred: bool,
    pub status: String,
}

/// The IBC channel graph — adjacency list of chain channel connections.
#[derive(Debug, Clone, Default)]
pub struct IBCChannelGraph {
    pub edges: HashMap<String, Vec<ChannelEdge>>,
}

impl IBCChannelGraph {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a channel connection between two chains.
    pub fn add_channel(
        &mut self,
        chain_a: &str,
        chain_a_channel: &str,
        chain_b: &str,
        chain_b_channel: &str,
        preferred: bool,
        status: String,
    ) {
        self.edges
            .entry(chain_a.to_string())
            .or_default()
            .push(ChannelEdge {
                counterparty_chain: chain_b.to_string(),
                counterparty_channel_id: chain_b_channel.to_string(),
                this_channel_id: chain_a_channel.to_string(),
                preferred,
                status: status.clone(),
            });

        self.edges
            .entry(chain_b.to_string())
            .or_default()
            .push(ChannelEdge {
                counterparty_chain: chain_a.to_string(),
                counterparty_channel_id: chain_a_channel.to_string(),
                this_channel_id: chain_b_channel.to_string(),
                preferred,
                status,
            });
    }

    /// Build the graph from a state.json ibc_data object.
    pub fn build_from_state(ibc_data_obj: &serde_json::Value) -> Self {
        let mut graph = Self::new();

        if let Some(obj) = ibc_data_obj.as_object() {
            for (_key, data) in obj {
                let chain_1_name = data["chain_1"]["chain_name"].as_str().unwrap_or("");
                let chain_2_name = data["chain_2"]["chain_name"].as_str().unwrap_or("");

                if let Some(channels) = data["channels"].as_array() {
                    for chan in channels {
                        let ch1_id = chan["chain_1"]["channel_id"].as_str().unwrap_or("");
                        let ch2_id = chan["chain_2"]["channel_id"].as_str().unwrap_or("");
                        let preferred = chan["tags"]["preferred"].as_bool().unwrap_or(false);
                        let status = chan["tags"]["status"]
                            .as_str()
                            .unwrap_or("UNKNOWN")
                            .to_string();

                        if !ch1_id.is_empty() && !ch2_id.is_empty() {
                            graph.add_channel(
                                chain_1_name, ch1_id, chain_2_name, ch2_id, preferred, status,
                            );
                        }
                    }
                }
            }
        }

        graph
    }

    /// Find all routes from source to dest (BFS, up to max_hops).
    pub fn find_routes(&self, source: &str, dest: &str, max_hops: usize) -> Vec<Vec<ChannelHop>> {
        let mut routes = Vec::new();
        let mut queue = vec![(source.to_string(), Vec::new(), HashSet::new())];

        while let Some((current_chain, path, visited)) = queue.pop() {
            if current_chain == dest && !path.is_empty() {
                routes.push(path);
                continue;
            }

            if path.len() >= max_hops {
                continue;
            }

            if let Some(neighbors) = self.edges.get(&current_chain) {
                for edge in neighbors {
                    if edge.status != "ACTIVE" {
                        continue;
                    }
                    if visited.contains(&edge.counterparty_chain) {
                        continue;
                    }

                    let mut new_visited = visited.clone();
                    new_visited.insert(current_chain.clone());

                    let mut new_path = path.clone();
                    new_path.push(ChannelHop {
                        from_chain: current_chain.clone(),
                        from_channel: edge.this_channel_id.clone(),
                        to_chain: edge.counterparty_chain.clone(),
                        to_channel: edge.counterparty_channel_id.clone(),
                    });

                    queue.push((edge.counterparty_chain.clone(), new_path, new_visited));
                }
            }
        }

        // Sort by hop count (shortest first), then prefer routes with preferred channels
        routes.sort_by(|a, b| {
            a.len().cmp(&b.len()).then_with(|| {
                let a_preferred = a
                    .iter()
                    .any(|h| self.is_preferred(&h.from_chain, &h.to_chain, &h.from_channel));
                let b_preferred = b
                    .iter()
                    .any(|h| self.is_preferred(&h.from_chain, &h.to_chain, &h.from_channel));
                b_preferred.cmp(&a_preferred)
            })
        });

        routes
    }

    fn is_preferred(&self, from: &str, to: &str, channel: &str) -> bool {
        self.edges
            .get(from)
            .and_then(|edges| {
                edges
                    .iter()
                    .find(|e| e.counterparty_chain == to && e.this_channel_id == channel)
            })
            .map(|e| e.preferred)
            .unwrap_or(false)
    }

    /// Compute the IBC denom for an asset on a destination chain via a specific route.
    pub fn compute_ibc_denom_for_route(
        &self,
        origin_denom: &str,
        route: &[ChannelHop],
    ) -> (String, String) {
        // Build the trace path from the perspective of the dest chain looking back to origin
        let mut trace_path = String::new();
        for hop in route.iter().rev() {
            trace_path = format!("transfer/{}/", hop.to_channel) + &trace_path;
        }
        trace_path.push_str(origin_denom);

        let hash = compute_ibc_denom_hash(&trace_path);
        (hash, trace_path)
    }
}

// ---------------------------------------------------------------------------
// Derived IBC asset route
// ---------------------------------------------------------------------------

/// A fully resolved IBC asset route.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IBCAssetRoute {
    pub symbol: String,
    pub origin_chain: String,
    pub origin_denom: String,
    pub dest_chain: String,
    pub dest_denom: String,
    pub trace_path: String,
    pub route: Vec<ChannelHop>,
    pub hop_count: usize,
    pub preferred: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingTableMetadata {
    pub chains: Vec<String>,
    pub total_routes: usize,
    pub generated_at: String,
}

/// The full IBC Asset Routing Table — premined routes for all assets across all chains.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IBCAssetRoutingTable {
    pub routes: HashMap<String, Vec<IBCAssetRoute>>,
    pub metadata: RoutingTableMetadata,
}

impl IBCAssetRoutingTable {
    /// Premine all possible IBC asset routes from the graph and asset lists.
    pub fn premine(
        graph: &IBCChannelGraph,
        chain_assets: &HashMap<String, Vec<serde_json::Value>>,
        max_hops: usize,
    ) -> Self {
        let mut routes: HashMap<String, Vec<IBCAssetRoute>> = HashMap::new();
        let mut chains = HashSet::new();

        for (source_chain, assets) in chain_assets {
            chains.insert(source_chain.clone());

            for asset in assets {
                let symbol = asset["symbol"].as_str().unwrap_or("UNKNOWN");
                let base_denom = asset["base"].as_str().unwrap_or("");

                let is_native = !base_denom.starts_with("ibc/");

                if is_native {
                    for dest_chain in graph.edges.keys() {
                        if dest_chain == source_chain {
                            continue;
                        }

                        let found_routes = graph.find_routes(source_chain, dest_chain, max_hops);

                        for route in found_routes {
                            let (ibc_denom, trace_path) =
                                graph.compute_ibc_denom_for_route(base_denom, &route);

                            let preferred = route
                                .iter()
                                .all(|h| graph.is_preferred(&h.from_chain, &h.to_chain, &h.from_channel));

                            let entry = IBCAssetRoute {
                                symbol: symbol.to_string(),
                                origin_chain: source_chain.clone(),
                                origin_denom: base_denom.to_string(),
                                dest_chain: dest_chain.clone(),
                                dest_denom: ibc_denom.clone(),
                                trace_path: trace_path.clone(),
                                route: route.clone(),
                                hop_count: route.len(),
                                preferred,
                            };

                            routes.entry(dest_chain.clone()).or_default().push(entry);
                        }
                    }
                } else {
                    // IBC asset on this chain — trace further
                    if let Some(traces) = asset["traces"].as_array() {
                        if let Some(ibc_trace) = traces.iter().find(|t| {
                            matches!(
                                t["type"].as_str().unwrap_or(""),
                                "ibc" | "ibc-cw20" | "ibc-bridge"
                            )
                        }) {
                            let cp_chain =
                                ibc_trace["counterparty"]["chain_name"].as_str().unwrap_or("");
                            let cp_denom =
                                ibc_trace["counterparty"]["base_denom"].as_str().unwrap_or("");
                            let source_trace =
                                ibc_trace["chain"]["path"].as_str().unwrap_or("");

                            for dest_chain in graph.edges.keys() {
                                if dest_chain == source_chain {
                                    continue;
                                }

                                let found_routes =
                                    graph.find_routes(source_chain, dest_chain, max_hops);

                                for route in found_routes {
                                    let last_hop = &route[route.len() - 1];
                                    let full_trace = if source_trace.is_empty() {
                                        format!("transfer/{}/{}", last_hop.to_channel, cp_denom)
                                    } else {
                                        format!(
                                            "transfer/{}/{}",
                                            last_hop.to_channel, source_trace
                                        )
                                    };

                                    let ibc_denom = compute_ibc_denom_hash(&full_trace);
                                    let preferred = route
                                        .iter()
                                        .all(|h| {
                                            graph.is_preferred(
                                                &h.from_chain,
                                                &h.to_chain,
                                                &h.from_channel,
                                            )
                                        });

                                    let entry = IBCAssetRoute {
                                        symbol: symbol.to_string(),
                                        origin_chain: cp_chain.to_string(),
                                        origin_denom: cp_denom.to_string(),
                                        dest_chain: dest_chain.clone(),
                                        dest_denom: ibc_denom.clone(),
                                        trace_path: full_trace,
                                        route: route.clone(),
                                        hop_count: route.len(),
                                        preferred,
                                    };

                                    routes
                                        .entry(dest_chain.clone())
                                        .or_default()
                                        .push(entry);
                                }
                            }
                        }
                    }
                }
            }
        }

        let total_routes: usize = routes.values().map(|v| v.len()).sum();

        Self {
            routes,
            metadata: RoutingTableMetadata {
                chains: chains.into_iter().collect(),
                total_routes,
                generated_at: String::new(),
            },
        }
    }

    /// Look up all IBC assets available on a specific chain.
    pub fn lookup_by_dest_chain(&self, chain: &str) -> Vec<&IBCAssetRoute> {
        self.routes
            .get(chain)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }

    /// Look up the IBC denom for a specific asset on a specific chain.
    pub fn lookup_ibc_denom(&self, symbol: &str, dest_chain: &str) -> Option<&IBCAssetRoute> {
        self.routes
            .get(dest_chain)?
            .iter()
            .find(|r| r.symbol == symbol && r.preferred)
            .or_else(|| self.routes.get(dest_chain)?.iter().find(|r| r.symbol == symbol))
    }

    /// Export the routing table as JSON.
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
}

// ---------------------------------------------------------------------------
// Derive IBC denom from assetlist traces (Terp-specific)
// ---------------------------------------------------------------------------

/// Derive the IBC denom hash and trace path for a foreign asset on Terp.
///
/// Logic:
/// 1. If the asset has an IBC trace, extract counterparty chain and base denom
/// 2. If Terp has a direct channel to the counterparty chain → single hop
/// 3. If Terp has NO direct channel but has channel to source chain → route via source
pub fn derive_terp_ibc_denom(
    asset: &serde_json::Value,
    terp_channels: &HashMap<String, TerpChannelInfo>,
) -> Option<(String, String, String)> {
    let traces = asset["traces"].as_array()?;
    let ibc_trace = traces.iter().find(|t| {
        matches!(
            t["type"].as_str().unwrap_or(""),
            "ibc" | "ibc-cw20" | "ibc-bridge"
        )
    })?;

    let counterparty_chain = ibc_trace["counterparty"]["chain_name"].as_str()?;
    let counterparty_base = ibc_trace["counterparty"]["base_denom"].as_str()?;
    let source_trace_path = ibc_trace["chain"]["path"].as_str().unwrap_or("");
    let source_chain = asset["_source_chain"].as_str().unwrap_or("");

    // Try direct channel to counterparty first
    if let Some(info) = terp_channels.get(counterparty_chain) {
        let full_path = format!("transfer/{}/{}", info.terp_channel_id, counterparty_base);
        let hash = compute_ibc_denom_hash(&full_path);
        return Some((hash, full_path, counterparty_chain.to_string()));
    }

    // No direct channel — route through the source chain
    if !source_chain.is_empty() {
        if let Some(info) = terp_channels.get(source_chain) {
            let full_path = if source_trace_path.is_empty() {
                format!("transfer/{}/{}", info.terp_channel_id, counterparty_base)
            } else {
                format!("transfer/{}/{}", info.terp_channel_id, source_trace_path)
            };
            let hash = compute_ibc_denom_hash(&full_path);
            return Some((hash, full_path, counterparty_chain.to_string()));
        }
    }

    None
}

/// Build a channel-to-chain map from ibc_data in state.json
pub fn build_channel_to_chain_map(
    ibc_data: &serde_json::Value,
) -> HashMap<String, TerpChannelInfo> {
    let mut map = HashMap::new();

    let obj = match ibc_data.as_object() {
        Some(o) => o,
        None => return map,
    };

    for (key, data) in obj {
        let (counterparty_name, terp_is_chain1) =
            if data["chain_1"].is_object() && data["chain_2"].is_object() {
                let chain_1_name = data["chain_1"]["chain_name"].as_str().unwrap_or("");
                let chain_2_name = data["chain_2"]["chain_name"].as_str().unwrap_or("");

                if chain_1_name == "terp" {
                    (chain_2_name.to_string(), true)
                } else if chain_2_name == "terp" {
                    (chain_1_name.to_string(), false)
                } else {
                    continue;
                }
            } else {
                let cp_name = key
                    .strip_suffix("-terp")
                    .or_else(|| key.strip_prefix("terp-"))
                    .unwrap_or(key);
                (cp_name.to_string(), false)
            };

        if counterparty_name.is_empty() {
            continue;
        }

        let channels = match data["channels"].as_array() {
            Some(c) => c,
            None => continue,
        };

        let preferred_channel = channels
            .iter()
            .find(|c| {
                let has_transfer_ports = if data["chain_1"].is_object() {
                    c["chain_1"]["port_id"].as_str() == Some("transfer")
                        && c["chain_2"]["port_id"].as_str() == Some("transfer")
                } else {
                    c[counterparty_name.as_str()]["port_id"].as_str() == Some("transfer")
                        && c["terp"]["port_id"].as_str() == Some("transfer")
                };
                has_transfer_ports && c["tags"]["preferred"].as_bool() == Some(true)
            })
            .or_else(|| {
                channels.iter().find(|c| {
                    if data["chain_1"].is_object() {
                        c["chain_1"]["port_id"].as_str() == Some("transfer")
                            && c["chain_2"]["port_id"].as_str() == Some("transfer")
                    } else {
                        c[counterparty_name.as_str()]["port_id"].as_str() == Some("transfer")
                            && c["terp"]["port_id"].as_str() == Some("transfer")
                    }
                })
            });

        let (terp_channel, cp_channel) = match preferred_channel {
            Some(c) => {
                if data["chain_1"].is_object() {
                    if terp_is_chain1 {
                        (
                            c["chain_1"]["channel_id"].as_str().unwrap_or("").to_string(),
                            c["chain_2"]["channel_id"].as_str().unwrap_or("").to_string(),
                        )
                    } else {
                        (
                            c["chain_2"]["channel_id"].as_str().unwrap_or("").to_string(),
                            c["chain_1"]["channel_id"].as_str().unwrap_or("").to_string(),
                        )
                    }
                } else {
                    (
                        c["terp"]["channel_id"].as_str().unwrap_or("").to_string(),
                        c[counterparty_name.as_str()]["channel_id"]
                            .as_str()
                            .unwrap_or("")
                            .to_string(),
                    )
                }
            }
            None => continue,
        };

        if !terp_channel.is_empty() {
            map.insert(
                counterparty_name.clone(),
                TerpChannelInfo {
                    terp_channel_id: terp_channel,
                    counterparty_channel_id: cp_channel,
                    counterparty_chain_name: counterparty_name,
                },
            );
        }
    }

    map
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn make_terp_channels() -> HashMap<String, TerpChannelInfo> {
        let mut map = HashMap::new();
        map.insert(
            "chain-b".to_string(),
            TerpChannelInfo {
                terp_channel_id: "channel-0".to_string(),
                counterparty_channel_id: "channel-0".to_string(),
                counterparty_chain_name: "chain-b".to_string(),
            },
        );
        map.insert(
            "chain-c".to_string(),
            TerpChannelInfo {
                terp_channel_id: "channel-1".to_string(),
                counterparty_channel_id: "channel-0".to_string(),
                counterparty_chain_name: "chain-c".to_string(),
            },
        );
        map
    }

    #[test]
    fn test_ibc_denom_hash() {
        let h = compute_ibc_denom_hash("transfer/channel-0/uterp");
        assert!(h.starts_with("ibc/"));
        assert_eq!(h.len(), 68);
    }

    #[test]
    fn test_single_hop_derivation() {
        let channels = make_terp_channels();
        let asset = json!({
            "symbol": "UTERP",
            "base": "uterp",
            "traces": [{
                "type": "ibc",
                "counterparty": {
                    "chain_name": "chain-b",
                    "base_denom": "uterp",
                    "channel_id": "channel-0"
                },
                "chain": {
                    "channel_id": "channel-0",
                    "path": "transfer/channel-0/uterp"
                }
            }],
            "_source_chain": "chain-b"
        });

        let (hash, path, cp) = derive_terp_ibc_denom(&asset, &channels).unwrap();
        assert_eq!(path, "transfer/channel-0/uterp");
        assert_eq!(cp, "chain-b");
        assert_eq!(hash, compute_ibc_denom_hash("transfer/channel-0/uterp"));
    }

    #[test]
    fn test_channel_graph_bfs() {
        let mut graph = IBCChannelGraph::new();
        graph.add_channel("chain-a", "channel-0", "chain-b", "channel-0", true, "ACTIVE".into());
        graph.add_channel("chain-b", "channel-1", "chain-c", "channel-0", true, "ACTIVE".into());

        let routes = graph.find_routes("chain-a", "chain-c", 3);
        assert_eq!(routes.len(), 1);
        assert_eq!(routes[0].len(), 2);
        assert_eq!(routes[0][1].to_chain, "chain-c");
    }
}
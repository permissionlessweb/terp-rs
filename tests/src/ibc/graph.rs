//! IBC channel graph: edges, BFS routes, denom path construction.

use crate::ibc::hash::compute_ibc_denom_hash;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// A single hop in an IBC route.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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

    /// Add a channel connection between two chains (both directions).
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

    /// Build the graph from a state.json ibc_data object (map of pair key → entry).
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

    /// True if there is an ACTIVE preferred transfer edge from `from` to `to`.
    pub fn has_direct_preferred_active(&self, from: &str, to: &str) -> bool {
        self.edges
            .get(from)
            .map(|edges| {
                edges.iter().any(|e| {
                    e.counterparty_chain == to
                        && e.preferred
                        && e.status == "ACTIVE"
                })
            })
            .unwrap_or(false)
    }

    /// Whether the given channel hop is tagged preferred.
    pub fn is_preferred(&self, from: &str, to: &str, channel: &str) -> bool {
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

    /// Whether every hop on the route uses a preferred channel.
    pub fn route_all_hops_preferred(&self, route: &[ChannelHop]) -> bool {
        route
            .iter()
            .all(|h| self.is_preferred(&h.from_chain, &h.to_chain, &h.from_channel))
    }

    /// Preferred flag for a route under hard invariant "prefer direct when open":
    /// multi-hop routes are never preferred if a direct ACTIVE preferred edge exists.
    pub fn route_is_preferred(&self, origin: &str, dest: &str, route: &[ChannelHop]) -> bool {
        if route.is_empty() {
            return false;
        }
        if route.len() > 1 && self.has_direct_preferred_active(origin, dest) {
            return false;
        }
        self.route_all_hops_preferred(route)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_graph_bfs() {
        let mut graph = IBCChannelGraph::new();
        graph.add_channel(
            "chain-a",
            "channel-0",
            "chain-b",
            "channel-0",
            true,
            "ACTIVE".into(),
        );
        graph.add_channel(
            "chain-b",
            "channel-1",
            "chain-c",
            "channel-0",
            true,
            "ACTIVE".into(),
        );

        let routes = graph.find_routes("chain-a", "chain-c", 3);
        assert_eq!(routes.len(), 1);
        assert_eq!(routes[0].len(), 2);
        assert_eq!(routes[0][1].to_chain, "chain-c");
    }

    #[test]
    fn prefer_direct_over_longer_preferred_path() {
        // Triangle: a—b, b—c, a—c all ACTIVE preferred
        let mut graph = IBCChannelGraph::new();
        graph.add_channel("a", "channel-ab", "b", "channel-ba", true, "ACTIVE".into());
        graph.add_channel("b", "channel-bc", "c", "channel-cb", true, "ACTIVE".into());
        graph.add_channel("a", "channel-ac", "c", "channel-ca", true, "ACTIVE".into());

        let routes = graph.find_routes("a", "c", 3);
        assert!(routes.len() >= 2, "expected direct + multi-hop");

        let direct = routes.iter().find(|r| r.len() == 1).expect("direct");
        let multi = routes.iter().find(|r| r.len() == 2).expect("multi");

        assert!(graph.route_is_preferred("a", "c", direct));
        assert!(
            !graph.route_is_preferred("a", "c", multi),
            "multi-hop must not be preferred when direct ACTIVE preferred exists"
        );
    }
}

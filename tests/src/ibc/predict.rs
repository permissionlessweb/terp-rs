//! PredictedWorld construction and hard invariant checks.

use crate::ibc::diff::{DiffReport, DiffSeverity};
use crate::ibc::graph::IBCChannelGraph;
use crate::ibc::hash::{compute_ibc_denom_hash, hop_count_from_trace_path};
use crate::ibc::routes::IBCAssetRoutingTable;
use crate::ibc::schema::validate_ibc_data_entry;
use std::collections::HashMap;

/// Pure prediction of IBC world state from topology + assets.
#[derive(Debug, Clone)]
pub struct PredictedWorld {
    pub graph: IBCChannelGraph,
    pub routes: IBCAssetRoutingTable,
    pub lookup: serde_json::Value,
    pub ibc_data: serde_json::Value,
}

impl PredictedWorld {
    /// Build from an ibc_data object map and per-chain asset lists.
    pub fn from_inputs(
        ibc_data: serde_json::Value,
        chain_assets: &HashMap<String, Vec<serde_json::Value>>,
        max_hops: usize,
    ) -> Self {
        let graph = IBCChannelGraph::build_from_state(&ibc_data);
        let routes = IBCAssetRoutingTable::premine(&graph, chain_assets, max_hops);
        let lookup = routes.to_simplified_lookup();
        Self {
            graph,
            routes,
            lookup,
            ibc_data,
        }
    }
}

/// Check hard authenticity invariants on a predicted world.
pub fn check_invariants(world: &PredictedWorld) -> DiffReport {
    let mut report = DiffReport::new();

    // Schema on each ibc_data entry
    if let Some(obj) = world.ibc_data.as_object() {
        for (key, entry) in obj {
            for e in validate_ibc_data_entry(entry, key) {
                report.push_at(DiffSeverity::Error, "schema", e, format!("ibc_data.{key}"));
            }

            // Channel side: chain_1/chain_2 names present and alpha order recommended as info
            let c1 = entry["chain_1"]["chain_name"].as_str().unwrap_or("");
            let c2 = entry["chain_2"]["chain_name"].as_str().unwrap_or("");
            if !c1.is_empty() && !c2.is_empty() && c1 > c2 {
                report.push_at(
                    DiffSeverity::Warn,
                    "alpha_order",
                    format!("chain_1 '{c1}' > chain_2 '{c2}' (registry usually alpha-orders)"),
                    format!("ibc_data.{key}"),
                );
            }
        }
    } else {
        report.push(
            DiffSeverity::Error,
            "schema",
            "ibc_data root is not an object",
        );
    }

    // Route-level invariants
    for (dest, routes) in &world.routes.routes {
        for (i, r) in routes.iter().enumerate() {
            let path_key = format!("routes.{dest}[{i}]");

            // hop_count honesty
            let expected_hops = hop_count_from_trace_path(&r.trace_path);
            if r.hop_count != expected_hops {
                report.push_at(
                    DiffSeverity::Error,
                    "hop_count",
                    format!(
                        "hop_count {} != transfer/ segments {} (path={})",
                        r.hop_count, expected_hops, r.trace_path
                    ),
                    &path_key,
                );
            }

            // Hash binding
            let expected_hash = compute_ibc_denom_hash(&r.trace_path);
            if r.dest_denom != expected_hash {
                report.push_at(
                    DiffSeverity::Error,
                    "hash_binding",
                    format!(
                        "dest_denom {} != hash({}) = {}",
                        r.dest_denom, r.trace_path, expected_hash
                    ),
                    &path_key,
                );
            }

            // Prefer direct: multi-hop preferred is illegal if direct ACTIVE preferred exists
            if r.preferred
                && r.hop_count > 1
                && world
                    .graph
                    .has_direct_preferred_active(&r.origin_chain, dest)
            {
                // Note: origin_chain for native is correct; for re-exported IBC assets origin may differ
                // from physical source of the multi-hop leg. Check also route endpoints if available.
                let physical_origin = r
                    .route
                    .first()
                    .map(|h| h.from_chain.as_str())
                    .unwrap_or(r.origin_chain.as_str());
                if world
                    .graph
                    .has_direct_preferred_active(physical_origin, dest)
                {
                    report.push_at(
                        DiffSeverity::Error,
                        "prefer_direct",
                        format!(
                            "preferred multi-hop (hops={}) while direct ACTIVE preferred exists {}→{}",
                            r.hop_count, physical_origin, dest
                        ),
                        &path_key,
                    );
                }
            }
        }
    }

    // Lookup preferred entries must also be hop-honest
    if let Some(obj) = world.lookup.as_object() {
        for (dest, chain_lookup) in obj {
            if let Some(entries) = chain_lookup.as_object() {
                for (denom, meta) in entries {
                    if let Some(hc) = meta["hop_count"].as_u64() {
                        if let Some(tp) = meta["trace_path"].as_str() {
                            let expected = hop_count_from_trace_path(tp) as u64;
                            if hc != expected {
                                report.push_at(
                                    DiffSeverity::Error,
                                    "hop_count",
                                    format!(
                                        "lookup hop_count {hc} != path segments {expected} for {denom}"
                                    ),
                                    format!("lookup.{dest}.{denom}"),
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    report
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn triangle_ibc_data() -> serde_json::Value {
        json!({
            "akash-terp": {
                "$schema": "../ibc_data.schema.json",
                "chain_1": {
                    "chain_name": "akash",
                    "chain_id": "akashnet-2",
                    "client_id": "07-tendermint-1",
                    "connection_id": "connection-1"
                },
                "chain_2": {
                    "chain_name": "terp",
                    "chain_id": "morocco-1",
                    "client_id": "07-tendermint-1",
                    "connection_id": "connection-1"
                },
                "channels": [{
                    "chain_1": { "channel_id": "channel-139", "port_id": "transfer" },
                    "chain_2": { "channel_id": "channel-9", "port_id": "transfer" },
                    "ordering": "unordered",
                    "version": "ics20-1",
                    "tags": { "preferred": true, "status": "ACTIVE" }
                }]
            },
            "osmosis-terp": {
                "$schema": "../ibc_data.schema.json",
                "chain_1": {
                    "chain_name": "osmosis",
                    "chain_id": "osmosis-1",
                    "client_id": "07-tendermint-1",
                    "connection_id": "connection-1"
                },
                "chain_2": {
                    "chain_name": "terp",
                    "chain_id": "morocco-1",
                    "client_id": "07-tendermint-2",
                    "connection_id": "connection-2"
                },
                "channels": [{
                    "chain_1": { "channel_id": "channel-1", "port_id": "transfer" },
                    "chain_2": { "channel_id": "channel-0", "port_id": "transfer" },
                    "ordering": "unordered",
                    "version": "ics20-1",
                    "tags": { "preferred": true, "status": "ACTIVE" }
                }]
            },
            "akash-osmosis": {
                "$schema": "../ibc_data.schema.json",
                "chain_1": {
                    "chain_name": "akash",
                    "chain_id": "akashnet-2",
                    "client_id": "07-tendermint-2",
                    "connection_id": "connection-2"
                },
                "chain_2": {
                    "chain_name": "osmosis",
                    "chain_id": "osmosis-1",
                    "client_id": "07-tendermint-2",
                    "connection_id": "connection-2"
                },
                "channels": [{
                    "chain_1": { "channel_id": "channel-9", "port_id": "transfer" },
                    "chain_2": { "channel_id": "channel-1", "port_id": "transfer" },
                    "ordering": "unordered",
                    "version": "ics20-1",
                    "tags": { "preferred": true, "status": "ACTIVE" }
                }]
            }
        })
    }

    #[test]
    fn predicted_world_triangle_no_errors() {
        let ibc_data = triangle_ibc_data();
        let mut assets = HashMap::new();
        assets.insert(
            "akash".to_string(),
            vec![json!({"symbol": "AKT", "base": "uakt"})],
        );
        assets.insert(
            "terp".to_string(),
            vec![json!({"symbol": "TERP", "base": "uterp"})],
        );
        assets.insert(
            "osmosis".to_string(),
            vec![json!({"symbol": "OSMO", "base": "uosmo"})],
        );

        let world = PredictedWorld::from_inputs(ibc_data, &assets, 3);
        let report = check_invariants(&world);
        for e in report.errors() {
            eprintln!("ERROR {}: {}", e.code, e.message);
        }
        assert!(!report.has_errors());

        // AKT on osmosis preferred is direct, not via terp
        let akt = world
            .routes
            .lookup_ibc_denom("AKT", "osmosis")
            .expect("AKT on osmosis");
        assert_eq!(akt.hop_count, 1);
        assert!(akt.preferred);
    }

    #[test]
    fn detects_hop_count_lie_in_world() {
        let mut world = PredictedWorld::from_inputs(json!({}), &HashMap::new(), 1);
        // Inject a lying route
        world.routes.routes.insert(
            "c".into(),
            vec![crate::ibc::routes::IBCAssetRoute {
                symbol: "X".into(),
                origin_chain: "a".into(),
                origin_denom: "ux".into(),
                dest_chain: "c".into(),
                dest_denom: compute_ibc_denom_hash("transfer/channel-0/transfer/channel-1/ux"),
                trace_path: "transfer/channel-0/transfer/channel-1/ux".into(),
                route: vec![],
                hop_count: 1,
                preferred: false,
            }],
        );
        let report = check_invariants(&world);
        assert!(
            report.errors().any(|e| e.code == "hop_count"),
            "expected hop_count error"
        );
    }
}

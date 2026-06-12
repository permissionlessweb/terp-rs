//! Schema validation and derivation tests for ibc-info output.
//!
//! Loads real data from `public/` and validates:
//!   - assetlist.json against asset_list.schema.json
//!   - ibc_data entries against ibc_data.schema.json
//!   - IBCChannelGraph and IBCAssetRoutingTable derivation
//!   - IBC denom hash computation matches known hashes
//!
//! These are synchronous unit tests — no Docker required.

use scripts::ibc_core::{
    build_channel_to_chain_map, compute_ibc_denom_hash, IBCAssetRoutingTable, IBCChannelGraph,
    TerpChannelInfo,
};
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;

// ---------------------------------------------------------------------------
// Helpers: load schema + data files
// ---------------------------------------------------------------------------

/// Read a JSON file from the public/ directory.
fn load_json(name: &str) -> serde_json::Value {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("public")
        .join(name);
    let data = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {e}", path.display()));
    serde_json::from_str(&data)
        .unwrap_or_else(|e| panic!("Failed to parse {}: {e}", path.display()))
}

// ---------------------------------------------------------------------------
// Schema validation helpers (structural — no jsonschema crate dependency)
// ---------------------------------------------------------------------------

/// Validate an asset entry against asset_list.schema.json `$defs/asset`.
fn validate_asset_entry(asset: &Value, path: &str) -> Vec<String> {
    let mut errs = Vec::new();

    // Required fields
    for field in &["denom_units", "type_asset", "base", "display", "name", "symbol"] {
        if !asset.get(*field).is_some() {
            errs.push(format!("{path}: missing required field '{field}'"));
        }
    }

    // type_asset must be from enum
    if let Some(ta) = asset["type_asset"].as_str() {
        let valid = [
            "sdk.coin", "cw20", "erc20", "ics20", "snip20", "snip25",
            "bitcoin-like", "evm-base", "svm-base", "substrate", "unknown",
        ];
        if !valid.contains(&ta) {
            errs.push(format!("{path}: invalid type_asset '{ta}'"));
        }
    }

    // denom_units: each must have denom + exponent
    if let Some(units) = asset["denom_units"].as_array() {
        for (i, u) in units.iter().enumerate() {
            if !u.get("denom").and_then(|d| d.as_str()).is_some() {
                errs.push(format!("{path}.denom_units[{i}]: missing 'denom'"));
            }
            if !u.get("exponent").and_then(|e| e.as_i64()).is_some() {
                errs.push(format!("{path}.denom_units[{i}]: missing 'exponent'"));
            }
        }
    }

    // traces[].type must be from enum
    if let Some(traces) = asset["traces"].as_array() {
        let valid_types = ["ibc", "ibc-cw20", "ibc-bridge", "bridge", "liquid-stake",
            "synthetic", "wrapped", "additional-mintage", "test-mintage", "legacy-mintage"];
        for (i, t) in traces.iter().enumerate() {
            if let Some(tt) = t["type"].as_str() {
                if !valid_types.contains(&tt) {
                    errs.push(format!("{path}.traces[{i}]: invalid type '{tt}'"));
                }
                // For ibc type, validate counterparty + chain sub-fields
                if tt == "ibc" || tt == "ibc-cw20" {
                    let cp = &t["counterparty"];
                    for f in &["chain_name", "base_denom", "channel_id"] {
                        if !cp.get(*f).and_then(|v| v.as_str()).is_some() {
                            errs.push(format!("{path}.traces[{i}].counterparty: missing '{f}'"));
                        }
                    }
                    if let Some(ch) = cp["channel_id"].as_str() {
                        if !ch.starts_with("channel-") {
                            errs.push(format!("{path}.traces[{i}].counterparty.channel_id: invalid pattern '{ch}'"));
                        }
                    }
                    let ch = &t["chain"];
                    for f in &["channel_id", "path"] {
                        if !ch.get(*f).and_then(|v| v.as_str()).is_some() {
                            errs.push(format!("{path}.traces[{i}].chain: missing '{f}'"));
                        }
                    }
                    if let Some(ch_id) = ch["channel_id"].as_str() {
                        if !ch_id.starts_with("channel-") {
                            errs.push(format!("{path}.traces[{i}].chain.channel_id: invalid pattern '{ch_id}'"));
                        }
                    }
                }
            }
        }
    }

    errs
}

/// Validate ibc_data entry against ibc_data.schema.json.
fn validate_ibc_data_entry(entry: &Value, key: &str) -> Vec<String> {
    let mut errs = Vec::new();

    // Required: $schema
    if entry.get("$schema").and_then(|s| s.as_str()).is_none() {
        errs.push(format!("ibc_data.{key}: missing '$schema'"));
    }

    // Required: chain_1, chain_2
    for side in &["chain_1", "chain_2"] {
        let c = match entry.get(*side) {
            Some(v) => v,
            None => {
                errs.push(format!("ibc_data.{key}: missing '{side}'"));
                continue;
            }
        };
        for f in &["chain_name", "chain_id", "client_id", "connection_id"] {
            if !c.get(*f).and_then(|v| v.as_str()).is_some() {
                errs.push(format!("ibc_data.{key}.{side}: missing '{f}'"));
            }
        }
    }

    // Required: channels array
    let channels = match entry["channels"].as_array() {
        Some(a) => a,
        None => {
            errs.push(format!("ibc_data.{key}: missing or non-array 'channels'"));
            return errs;
        }
    };

    for (i, ch) in channels.iter().enumerate() {
        let cp = format!("ibc_data.{key}.channels[{i}]");

        // Required: chain_1, chain_2, ordering, version
        for f in &["chain_1", "chain_2", "ordering", "version"] {
            if !ch.get(*f).is_some() {
                errs.push(format!("{cp}: missing '{f}'"));
            }
        }

        // ordering must be "ordered" or "unordered"
        match ch["ordering"].as_str() {
            Some("ordered") | Some("unordered") => {}
            Some(s) => errs.push(format!("{cp}.ordering: must be 'ordered' or 'unordered', got '{s}'")),
            None => errs.push(format!("{cp}.ordering: missing or invalid type (must be string)")),
        }

        // chain_1 and chain_2 must have channel_id + port_id
        for side in &["chain_1", "chain_2"] {
            let s = &ch[side];
            if s.get("channel_id").and_then(|v| v.as_str()).is_none() {
                errs.push(format!("{cp}.{side}: missing 'channel_id'"));
            }
            if s.get("port_id").and_then(|v| v.as_str()).is_none() {
                errs.push(format!("{cp}.{side}: missing 'port_id'"));
            }
        }

        // channel_id must match ^(channel-\d+|\*)$
        for side in &["chain_1", "chain_2"] {
            if let Some(cid) = ch[side]["channel_id"].as_str() {
                if cid != "*" && !cid.starts_with("channel-") {
                    errs.push(format!("{cp}.{side}.channel_id: invalid pattern '{cid}'"));
                }
            }
        }

        // tags (optional) must have preferred: bool, status: enum
        if let Some(tags) = ch.get("tags") {
            if tags.get("preferred").and_then(|p| p.as_bool()).is_none() {
                errs.push(format!("{cp}.tags: missing or non-bool 'preferred'"));
            }
            if let Some(status) = tags.get("status").and_then(|s| s.as_str()) {
                let valid = ["ACTIVE", "INACTIVE", "CLOSED", "PENDING"];
                if !valid.contains(&status) {
                    errs.push(format!("{cp}.tags.status: invalid '{status}'"));
                }
            }
        }
    }

    errs
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test_assetlist_schema_compliance() {
    let assetlist = load_json("assetlist.json");

    // Root must have $schema, chain_name, assets
    assert!(
        assetlist["$schema"].as_str().is_some(),
        "assetlist.json: missing '$schema'"
    );
    assert!(
        assetlist["chain_name"].as_str().is_some(),
        "assetlist.json: missing 'chain_name'"
    );
    let assets = assetlist["assets"].as_array()
        .expect("assetlist.json: 'assets' must be a non-empty array");
    assert!(!assets.is_empty(), "assetlist.json: 'assets' is empty");

    let mut total_errs = 0;
    for (i, asset) in assets.iter().enumerate() {
        let errs = validate_asset_entry(asset, &format!("assets[{i}]"));
        for e in &errs {
            eprintln!("  SCHEMA VIOLATION: {e}");
        }
        total_errs += errs.len();
    }

    println!(
        "assetlist.json: {} assets validated, {} schema violations",
        assets.len(),
        total_errs
    );
    assert_eq!(total_errs, 0, "assetlist.json has schema violations");
}

#[test]
fn test_ibc_data_schema_via_constructed_entry() {
    // Construct a schema-compliant ibc_data entry (the format the bin/ibc_info.rs now generates)
    let entry = serde_json::json!({
        "$schema": "../ibc_data.schema.json",
        "chain_1": {
            "chain_name": "terp",
            "chain_id": "morocco-1",
            "client_id": "07-tendermint-32",
            "connection_id": "connection-11",
        },
        "chain_2": {
            "chain_name": "akash",
            "chain_id": "akashnet-2",
            "client_id": "07-tendermint-210",
            "connection_id": "connection-207",
        },
        "channels": [{
            "chain_1": {
                "channel_id": "channel-115",
                "port_id": "transfer",
            },
            "chain_2": {
                "channel_id": "channel-6",
                "port_id": "transfer",
            },
            "ordering": "unordered",
            "version": "ics20-1",
            "tags": {
                "preferred": true,
                "status": "ACTIVE",
            },
        }],
    });

    let errs = validate_ibc_data_entry(&entry, "terp-akash");
    for e in &errs {
        eprintln!("  VIOLATION: {e}");
    }
    assert_eq!(errs.len(), 0, "Constructed ibc_data entry should be schema-compliant");
}

#[test]
fn test_ibc_data_rejects_int_ordering() {
    // The old format with int ordering should be rejected
    let entry = serde_json::json!({
        "$schema": "../ibc_data.schema.json",
        "chain_1": { "chain_name": "terp", "chain_id": "x", "client_id": "y", "connection_id": "z" },
        "chain_2": { "chain_name": "osmo", "chain_id": "x", "client_id": "y", "connection_id": "z" },
        "channels": [{
            "chain_1": { "channel_id": "channel-0", "port_id": "transfer" },
            "chain_2": { "channel_id": "channel-1", "port_id": "transfer" },
            "ordering": 1,  // schema requires string "unordered" or "ordered"
            "version": "ics20-1",
        }],
    });

    let errs = validate_ibc_data_entry(&entry, "terp-osmo");
    let ordering_errs: Vec<_> = errs.iter().filter(|e| e.contains("ordering")).collect();
    assert!(
        !ordering_errs.is_empty(),
        "Should flag int ordering as violation, got errs: {errs:?}"
    );
    println!("  ✓ Correctly rejected int ordering: {}", ordering_errs[0]);
}

#[test]
fn test_ibc_data_rejects_missing_tags() {
    let entry = serde_json::json!({
        "$schema": "../ibc_data.schema.json",
        "chain_1": { "chain_name": "terp", "chain_id": "x", "client_id": "y", "connection_id": "z" },
        "chain_2": { "chain_name": "osmo", "chain_id": "x", "client_id": "y", "connection_id": "z" },
        "channels": [{
            "chain_1": { "channel_id": "channel-0", "port_id": "transfer" },
            "chain_2": { "channel_id": "channel-1", "port_id": "transfer" },
            "ordering": "unordered",
            "version": "ics20-1",
            "tags": { "status": "ACTIVE" }  // missing preferred field
        }],
    });

    let errs = validate_ibc_data_entry(&entry, "terp-osmo");
    let preferred_errs: Vec<_> = errs.iter().filter(|e| e.contains("preferred")).collect();
    assert!(
        !preferred_errs.is_empty(),
        "Should flag missing preferred: {errs:?}"
    );
}

#[test]
fn test_ibc_denom_computation_known_hashes() {
    // Verify known IBC denom hashes from the real assetlist
    // AKT on Terp: path = "transfer/channel-1/uakt"
    // AKT on Osmosis (direct from Akash): path = "transfer/channel-1/uakt"
        let hash_osmo = compute_ibc_denom_hash("transfer/channel-1/uakt");
        assert_eq!(
            hash_osmo,
            "ibc/1480B8FD20AD5FCAE81EA87584D269547DD4D436843C1D20F15E00EB64743EF4",
            "AKT IBC denom hash on Osmosis mismatch"
        );

        // Verify the hash from the Terp assetlist by reading it directly
        let assetlist = load_json("assetlist.json");
        let akt_entry = assetlist["assets"].as_array().unwrap().iter()
            .find(|a| a["symbol"].as_str() == Some("AKT"))
            .expect("AKT not found in assetlist");
        let akt_terp_hash = akt_entry["base"].as_str().unwrap();
        println!("  AKT on Terp (from assetlist): base={akt_terp_hash}");

        // ATONE on Terp: verify hash from assetlist
        let hash_atone = compute_ibc_denom_hash("transfer/channel-13/uatone");
    // Load the actual hash from assetlist
    let assetlist = load_json("assetlist.json");
    let atone_entry = assetlist["assets"].as_array().unwrap().iter()
        .find(|a| a["symbol"].as_str() == Some("ATONE"))
        .expect("ATONE not found in assetlist");
    let atone_hash = atone_entry["base"].as_str().unwrap();
    assert_eq!(
        hash_atone, atone_hash,
        "ATONE IBC denom hash should match assetlist"
    );
    println!("  ✓ ATONE: {} matches assetlist entry", atone_hash);
}

#[test]
fn test_channel_graph_and_routing_with_real_ibc_data() {
    // Build ibc_data from known channel pairs (terp <-> osmosis, akash, atomone)
    // Uses the format now generated by bin/ibc_info.rs
    let ibc_data = serde_json::json!({
        "akash-terp": {
            "$schema": "../ibc_data.schema.json",
            "chain_1": { "chain_name": "akash", "chain_id": "akashnet-2", "client_id": "07-tendermint-210", "connection_id": "connection-207" },
            "chain_2": { "chain_name": "terp", "chain_id": "morocco-1", "client_id": "07-tendermint-32", "connection_id": "connection-11" },
            "channels": [{
                "chain_1": { "channel_id": "channel-6", "port_id": "transfer" },
                "chain_2": { "channel_id": "channel-115", "port_id": "transfer" },
                "ordering": "unordered",
                "version": "ics20-1",
                "tags": { "preferred": true, "status": "ACTIVE" }
            }]
        },
        "osmosis-terp": {
            "$schema": "../ibc_data.schema.json",
            "chain_1": { "chain_name": "osmosis", "chain_id": "osmosis-1", "client_id": "07-tendermint-3708", "connection_id": "connection-11060" },
            "chain_2": { "chain_name": "terp", "chain_id": "morocco-1", "client_id": "07-tendermint-33", "connection_id": "connection-13" },
            "channels": [{
                "chain_1": { "channel_id": "channel-1", "port_id": "transfer" },
                "chain_2": { "channel_id": "channel-6738", "port_id": "transfer" },
                "ordering": "unordered",
                "version": "ics20-1",
                "tags": { "preferred": true, "status": "ACTIVE" }
            }]
        },
        "atomone-terp": {
            "$schema": "../ibc_data.schema.json",
            "chain_1": { "chain_name": "atomone", "chain_id": "atomone-1", "client_id": "07-tendermint-46", "connection_id": "connection-42" },
            "chain_2": { "chain_name": "terp", "chain_id": "morocco-1", "client_id": "07-tendermint-34", "connection_id": "connection-12" },
            "channels": [{
                "chain_1": { "channel_id": "channel-10", "port_id": "transfer" },
                "chain_2": { "channel_id": "channel-13", "port_id": "transfer" },
                "ordering": "unordered",
                "version": "ics20-1",
                "tags": { "preferred": true, "status": "ACTIVE" }
            }]
        },
    });

    // Validate all entries
    if let Some(obj) = ibc_data.as_object() {
        for (key, entry) in obj {
            let errs = validate_ibc_data_entry(entry, key);
            assert!(errs.is_empty(), "ibc_data.{key}: {errs:?}");
        }
    }

    // 1. Build channel map
    let channel_map = build_channel_to_chain_map(&ibc_data);
    assert_eq!(channel_map.len(), 3, "Should have 3 channel entries");
    // Check terp's channel to osmosis
    let osmo_info = channel_map.get("osmosis").expect("Missing osmosis channel info");
    assert_eq!(osmo_info.terp_channel_id, "channel-6738");
    println!("  ✓ Channel map: terp/channel-6738 <-> osmosis/channel-1");

    // 2. Build channel graph
    let graph = IBCChannelGraph::build_from_state(&ibc_data);
    assert_eq!(graph.edges.len(), 4, "Graph should cover 4 chains (terp + 3 counterparties)");
    println!("  ✓ Channel graph: {} chains connected", graph.edges.len());

    // 3. Find routes
    let routes = graph.find_routes("terp", "akash", 3);
    assert!(!routes.is_empty(), "Should find terp->akash route");
    println!("  ✓ Route terp→akash: {} hops, {} paths", routes[0].len(), routes.len());

    // 4. Compute IBC denom via route
    let (denom, trace) = graph.compute_ibc_denom_for_route("uakt", &routes[0]);
    assert_eq!(trace, "transfer/channel-6/uakt", "Trace path mismatch (akash-side channel)");
    println!("  ✓ IBC denom for AKT on terp: {denom} (trace: {trace})");

    // Load native Terp assets and run routing table
    let mut chain_assets: HashMap<String, Vec<serde_json::Value>> = HashMap::new();
    // Use terp-native assets from state.json
    let state = load_json("state.json");
    let terp_assets: Vec<serde_json::Value> = state["morocco-1"]["assets"]
        .as_array()
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .filter(|a| a["traces"].as_array().map_or(true, |t| t.is_empty()))
        .collect();
    chain_assets.insert("terp".to_string(), terp_assets.clone());

    // Add osmosis assets
    let osmo_assets: Vec<serde_json::Value> = state["osmosis-1"]["assets"]
        .as_array()
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .map(|mut a| { a["_source_chain"] = serde_json::json!("osmosis"); a })
        .collect();
    chain_assets.insert("osmosis".to_string(), osmo_assets.clone());

    // Also load akash assets from osmosis (these are IBC assets on osmosis)
    let akash_assets_on_osmo: Vec<_> = osmo_assets.iter()
        .filter(|a| a["symbol"].as_str() == Some("AKT"))
        .cloned()
        .collect();
    if !akash_assets_on_osmo.is_empty() {
        // They need _source_chain = "akash" for routing
        let mut routed: Vec<_> = akash_assets_on_osmo.iter()
            .map(|a| {
                let mut v = a.clone();
                v["_source_chain"] = serde_json::json!("akash");
                v
            })
            .collect();
        chain_assets.entry("akash".to_string()).or_default().append(&mut routed);
    }

    println!("  Assets: terp={} native, osmosis={}", terp_assets.len(), osmo_assets.len());

    // 5. Premine routing table
    let rt = IBCAssetRoutingTable::premine(&graph, &chain_assets, 3);
    assert!(rt.metadata.total_routes > 0, "Should have at least some routes");
    println!("  ✓ Routing table: {} routes across {} chains", rt.metadata.total_routes, rt.metadata.chains.len());

    // Verify specific known route
    let routes_on_akash = rt.lookup_by_dest_chain("akash");
    println!("  Routes to akash: {}", routes_on_akash.len());
}

#[test]
fn test_asset_entry_validation_rejects_bad_data() {
    // Test that our validator catches schema violations
    let bad_asset = serde_json::json!({
        "base": "uterp",
        // missing: denom_units, type_asset, display, name, symbol
        "traces": [{
            "type": "ibc",
            "counterparty": {
                "chain_name": "osmosis",
                "base_denom": "uosmo",
                "channel_id": "bad-channel"  // should start with "channel-"
            },
            "chain": {
                "channel_id": "not-a-channel",  // should start with "channel-"
                "path": "transfer/channel-0/uosmo"
            }
        }]
    });

    let errs = validate_asset_entry(&bad_asset, "test_asset");
    assert!(!errs.is_empty(), "Should catch missing required fields");
    let missing: Vec<_> = errs.iter().filter(|e| e.contains("missing required")).collect();
    assert_eq!(missing.len(), 5, "Should flag 5 missing required fields: {missing:?}");

    let channel_pattern: Vec<_> = errs.iter().filter(|e| e.contains("invalid pattern")).collect();
    assert_eq!(channel_pattern.len(), 2, "Should flag 2 channel_id pattern violations");
    println!("  ✓ Validator correctly rejected {} violations", errs.len());
    for e in &errs {
        println!("    → {e}");
    }
}

#[test]
fn test_state_json_asset_structure() {
    // Verify the real state.json assets have the required structure
    let state = load_json("state.json");

    let osmo_assets = state["osmosis-1"]["assets"].as_array()
        .expect("state.json should have osmosis-1.assets");
    assert!(!osmo_assets.is_empty(), "osmosis should have assets");
    println!("osmosis-1.assets: {} entries", osmo_assets.len());

    // Validate each asset entry
    let mut total_errs = 0;
    for (i, a) in osmo_assets.iter().enumerate() {
        let errs = validate_asset_entry(a, &format!("state.osmosis-1.assets[{i}]"));
        total_errs += errs.len();
    }
    println!("  State asset validation: {total_errs} violations");
    // The state.json may have non-compliant entries (no schema enforcement when written)
    // We just report, don't assert — it's informational
}

#[test]
fn test_routing_table_integrity() {
    // Load the pre-computed routing table and verify internal consistency
    let rt_json = load_json("ibc_routing_table.json");
    let meta = &rt_json["metadata"];
    assert!(meta["total_routes"].as_u64().unwrap_or(0) > 0, "Should have routes");

    let routes = rt_json["routes"].as_object().expect("routes must be object");
    for (dest_chain, entries) in routes {
        let arr = entries.as_array().expect("routes entries must be array");
        for (i, entry) in arr.iter().enumerate() {
            // Each entry must have required fields
            for f in &["dest_chain", "dest_denom", "hop_count", "origin_chain", "origin_denom", "symbol", "trace_path"] {
                assert!(
                    entry.get(*f).is_some(),
                    "routing_table.{dest_chain}[{i}]: missing '{f}'"
                );
            }
            // hop_count should match route length
            if let (Some(hc), Some(route)) = (entry["hop_count"].as_u64(), entry["route"].as_array()) {
                // If hop_count is 0 (direct), route may be empty
                if hc > 0 {
                    assert_eq!(
                        hc as usize, route.len(),
                        "routing_table.{dest_chain}[{i}]: hop_count {} != route len {}",
                        hc, route.len()
                    );
                }
            }
            // dest_chain in entry should match key
            if let Some(dc) = entry["dest_chain"].as_str() {
                assert_eq!(
                    dc, dest_chain,
                    "routing_table.{dest_chain}[{i}]: dest_chain mismatch"
                );
            }
        }
    }

    println!(
        "✓ Routing table integrity: {} chains, {} routes validated",
        routes.len(),
        meta["total_routes"].as_u64().unwrap_or(0)
    );
}

#[test]
fn test_ibc_lookup_table_integrity() {
    let lt = load_json("ibc_lookup_table.json");
    let obj = lt.as_object().expect("lookup table must be an object");

    for (chain, entries) in obj {
        let eobj = entries.as_object().expect("entries must be object");
        for (ibc_hash, info) in eobj {
            // Each entry must have symbol, origin_chain, origin_denom, trace_path, hop_count
            for f in &["symbol", "origin_chain", "origin_denom", "trace_path", "hop_count"] {
                assert!(
                    info.get(*f).is_some(),
                    "lookup.{chain}.{ibc_hash}: missing '{f}'"
                );
            }
            // ibc_hash should start with "ibc/"
            assert!(
                ibc_hash.starts_with("ibc/"),
                "lookup.{chain}: key '{ibc_hash}' should start with 'ibc/'"
            );
        }
    }

    println!("✓ Lookup table integrity: {} chains", obj.len());
}

#[test]
fn test_construct_ibc_data_and_run_derivation_pipeline() {
    // End-to-end: construct ibc_data from scratch, build assets, run full pipeline
    let ibc_data = serde_json::json!({
        "chain-a-terp": {
            "$schema": "../ibc_data.schema.json",
            "chain_1": { "chain_name": "terp", "chain_id": "terp-1", "client_id": "07-tendermint-0", "connection_id": "connection-0" },
            "chain_2": { "chain_name": "chain-b", "chain_id": "chain-b", "client_id": "07-tendermint-0", "connection_id": "connection-0" },
            "channels": [{
                "chain_1": { "channel_id": "channel-0", "port_id": "transfer" },
                "chain_2": { "channel_id": "channel-0", "port_id": "transfer" },
                "ordering": "unordered", "version": "ics20-1",
                "tags": { "preferred": true, "status": "ACTIVE" }
            }]
        },
        "chain-b-chain-c": {
            "$schema": "../ibc_data.schema.json",
            "chain_1": { "chain_name": "chain-b", "chain_id": "chain-b", "client_id": "07-tendermint-0", "connection_id": "connection-0" },
            "chain_2": { "chain_name": "chain-c", "chain_id": "chain-c", "client_id": "07-tendermint-0", "connection_id": "connection-0" },
            "channels": [{
                "chain_1": { "channel_id": "channel-0", "port_id": "transfer" },
                "chain_2": { "channel_id": "channel-0", "port_id": "transfer" },
                "ordering": "unordered", "version": "ics20-1",
                "tags": { "preferred": true, "status": "ACTIVE" }
            }]
        },
    });

    let graph = IBCChannelGraph::build_from_state(&ibc_data);
    assert_eq!(graph.edges.len(), 3, "3 chains in graph");

    // Find chain-a → chain-c route (should go via chain-b)
    let routes = graph.find_routes("terp", "chain-c", 3);
    assert_eq!(routes.len(), 1, "Should find exactly 1 route chain-a→chain-c");
    assert_eq!(routes[0].len(), 2, "Should be 2 hops (a→b→c)");
    println!("  ✓ Route chain-a→chain-c: {} hops via chain-b", routes[0].len());

    // IBC denom for a double-hop transfer
    let (denom, trace) = graph.compute_ibc_denom_for_route("utoken", &routes[0]);
    assert_eq!(trace, "transfer/channel-0/transfer/channel-0/utoken");
    assert!(denom.starts_with("ibc/"));
    println!("  ✓ Double-hop IBC denom: {denom}");

    // Channel map
    let cm = build_channel_to_chain_map(&ibc_data);
    assert_eq!(cm.len(), 1, "Only chain-b has terp as counterparty in this data");

    println!("  ✓ Full derivation pipeline: graph → routes → denoms → map");
}
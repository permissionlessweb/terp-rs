//! Pure IBC authenticity unit tests (no Docker).

use scripts::ibc::{
    build_channel_to_chain_map, check_invariants, compute_ibc_denom_hash,
    finalize_channels_for_ibc_entry, hop_count_from_trace_path, ordering_to_str,
    validate_ibc_data_entry, IBCAssetRoutingTable, IBCChannelGraph, PredictedWorld,
};
use serde_json::json;
use std::collections::HashMap;

#[test]
fn hash_format_and_known_akt_osmosis() {
    let h = compute_ibc_denom_hash("transfer/channel-1/uakt");
    assert!(h.starts_with("ibc/"));
    assert_eq!(h.len(), 68);
    assert_eq!(
        h, "ibc/1480B8FD20AD5FCAE81EA87584D269547DD4D436843C1D20F15E00EB64743EF4"
    );
}

#[test]
fn schema_rejects_int_ordering() {
    let entry = json!({
        "$schema": "../ibc_data.schema.json",
        "chain_1": { "chain_name": "terp", "chain_id": "x", "client_id": "y", "connection_id": "z" },
        "chain_2": { "chain_name": "osmo", "chain_id": "x", "client_id": "y", "connection_id": "z" },
        "channels": [{
            "chain_1": { "channel_id": "channel-0", "port_id": "transfer" },
            "chain_2": { "channel_id": "channel-1", "port_id": "transfer" },
            "ordering": 1,
            "version": "ics20-1",
        }],
    });
    let errs = validate_ibc_data_entry(&entry, "terp-osmo");
    assert!(
        errs.iter().any(|e| e.contains("ordering")),
        "expected ordering violation: {errs:?}"
    );
}

#[test]
fn ordering_to_str_maps_grpc_ints() {
    assert_eq!(ordering_to_str(&json!(1)), "unordered");
    assert_eq!(ordering_to_str(&json!(2)), "ordered");
}

#[test]
fn hop_count_honesty_invariant() {
    let path = "transfer/channel-0/transfer/channel-1/uakt";
    assert_eq!(hop_count_from_trace_path(path), 2);
    // A world that sets hop_count=1 with this path is dishonest
    assert_ne!(1, hop_count_from_trace_path(path));
}

#[test]
fn prefer_direct_triangle() {
    let mut graph = IBCChannelGraph::new();
    graph.add_channel("a", "channel-ab", "b", "channel-ba", true, "ACTIVE".into());
    graph.add_channel("b", "channel-bc", "c", "channel-cb", true, "ACTIVE".into());
    graph.add_channel("a", "channel-ac", "c", "channel-ca", true, "ACTIVE".into());

    let mut assets = HashMap::new();
    assets.insert(
        "a".to_string(),
        vec![json!({"symbol": "TOKEN", "base": "utoken"})],
    );

    let table = IBCAssetRoutingTable::premine(&graph, &assets, 3);
    let preferred = table.lookup_ibc_denom("TOKEN", "c").expect("preferred");
    assert_eq!(preferred.hop_count, 1);
    assert!(preferred.preferred);
    assert!(!table
        .lookup_by_dest_chain("c")
        .iter()
        .filter(|r| r.hop_count > 1)
        .any(|r| r.preferred));
}

#[test]
fn finalize_akash_terp_channel_side_roundtrip() {
    let raw = vec![json!({
        "chain_1": { "channel_id": "channel-6", "port_id": "transfer" },
        "chain_2": { "channel_id": "channel-115", "port_id": "transfer" },
        "ordering": 1,
        "version": "ics20-1",
        "tags": { "status": "ACTIVE" }
    })];
    // akash < terp → chain_1_name = akash
    let final_chs = finalize_channels_for_ibc_entry(&raw, "akash", "akash", "Active");
    assert_eq!(final_chs[0]["chain_1"]["channel_id"], "channel-115");
    assert_eq!(final_chs[0]["chain_2"]["channel_id"], "channel-6");
    assert_eq!(final_chs[0]["ordering"], "unordered");
    assert_eq!(final_chs[0]["tags"]["preferred"], true);

    let ibc_entry = json!({
        "$schema": "../ibc_data.schema.json",
        "chain_1": {
            "chain_name": "akash",
            "chain_id": "akashnet-2",
            "client_id": "c1",
            "connection_id": "conn1"
        },
        "chain_2": {
            "chain_name": "terp",
            "chain_id": "morocco-1",
            "client_id": "c2",
            "connection_id": "conn2"
        },
        "channels": final_chs
    });
    let mut map_obj = serde_json::Map::new();
    map_obj.insert("akash".into(), ibc_entry);
    let map = build_channel_to_chain_map(&serde_json::Value::Object(map_obj));
    let info = map.get("akash").expect("akash map entry");
    assert_eq!(info.terp_channel_id, "channel-6");
    assert_eq!(info.counterparty_channel_id, "channel-115");
}

#[test]
fn check_invariants_flags_hop_count_lie() {
    let mut world = PredictedWorld::from_inputs(json!({}), &HashMap::new(), 1);
    world.routes.routes.insert(
        "c".into(),
        vec![scripts::ibc::IBCAssetRoute {
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
    assert!(report.has_errors());
    assert!(report.errors().any(|e| e.code == "hop_count"));
}

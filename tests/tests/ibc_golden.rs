//! Golden fixture tests for IBC authenticity (offline, no Docker).

use scripts::ibc::{
    check_invariants, compare_predict_observe, compute_ibc_denom_hash, golden_chain_assets,
    load_golden_ibc_data, load_known_hashes, FixtureBackend, ObserveBackend, PredictedWorld,
};

#[test]
fn known_hashes_match_compute() {
    let hashes = load_known_hashes().expect("load known_hashes");
    assert!(
        hashes.contains_key("transfer/channel-1/uakt"),
        "missing AKT osmosis vector"
    );
    for (path, expected) in &hashes {
        let got = compute_ibc_denom_hash(path);
        assert_eq!(&got, expected, "hash mismatch for path {path}");
        assert!(got.starts_with("ibc/"));
        assert_eq!(got.len(), 68);
    }
    // Explicit multi-hop synthetic pin
    let multi = hashes
        .get("transfer/channel-0/transfer/channel-1/uakt")
        .expect("multi-hop vector");
    assert_eq!(
        multi,
        &compute_ibc_denom_hash("transfer/channel-0/transfer/channel-1/uakt")
    );
}

#[test]
fn golden_ibc_data_predict_invariants_clean() {
    let ibc_data = load_golden_ibc_data().expect("load golden ibc-data");
    let assets = golden_chain_assets();
    let world = PredictedWorld::from_inputs(ibc_data, &assets, 3);
    let report = check_invariants(&world);
    for e in report.errors() {
        eprintln!(
            "ERROR [{}] {} ({:?})",
            e.code,
            e.message,
            e.path
        );
    }
    assert!(
        !report.has_errors(),
        "golden predicted world has hard invariant errors"
    );

    // Prefer-direct: AKT on osmosis is single-hop via direct akash-osmosis channel
    let akt = world
        .routes
        .lookup_ibc_denom("AKT", "osmosis")
        .expect("AKT preferred on osmosis");
    assert_eq!(akt.hop_count, 1);
    assert_eq!(akt.trace_path, "transfer/channel-1/uakt");
    assert_eq!(
        akt.dest_denom,
        "ibc/1480B8FD20AD5FCAE81EA87584D269547DD4D436843C1D20F15E00EB64743EF4"
    );
    assert!(akt.preferred);

    // Multi-hop alternates exist but are not preferred
    let all_akt: Vec<_> = world
        .routes
        .lookup_by_dest_chain("osmosis")
        .into_iter()
        .filter(|r| r.symbol == "AKT")
        .collect();
    assert!(all_akt.iter().any(|r| r.hop_count > 1));
    assert!(all_akt.iter().filter(|r| r.hop_count > 1).all(|r| !r.preferred));
}

#[test]
fn golden_fixture_backend_compare_on_known() {
    let hashes = load_known_hashes().expect("hashes");
    let backend = FixtureBackend::from_known_hashes(hashes);

    let ibc_data = load_golden_ibc_data().expect("ibc-data");
    let world = PredictedWorld::from_inputs(ibc_data, &golden_chain_assets(), 3);

    // Spot-check observe for AKT on osmosis preferred denom
    let akt = world
        .routes
        .lookup_ibc_denom("AKT", "osmosis")
        .expect("akt");
    let obs = backend
        .observe_denom_trace("osmosis", &akt.dest_denom)
        .unwrap()
        .expect("fixture observation for AKT");
    assert_eq!(obs.path, akt.trace_path);

    let report = compare_predict_observe(&world, &backend);
    // Unobserved preferred routes are Info only; no Errors for known hash match
    assert!(
        !report.has_errors(),
        "compare should not error on fixture-known paths"
    );
}

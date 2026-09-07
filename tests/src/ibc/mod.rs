//! IBC authenticity library: pure predict → observe → diff.
//!
//! Hard invariants (errors): channel side correctness, hop_count == transfer/
//! segments, prefer direct ACTIVE preferred routes, hash binding, schema.

pub mod diff;
pub mod fixtures;
pub mod graph;
pub mod hash;
pub mod normalize;
pub mod observe;
pub mod predict;
pub mod publish;
pub mod routes;
pub mod schema;

pub use diff::{DiffItem, DiffReport, DiffSeverity};
pub use publish::{
    default_public_dir, file_sha256, resolve_out_dir, AtomicPublisher,
};
pub use fixtures::{
    golden_chain_assets, golden_root, load_audited_known_hashes, load_expected_lookup,
    load_golden_ibc_data, load_json, load_known_hashes, load_synthetic_known_hashes,
    parse_known_hashes_value,
};
pub use graph::{ChannelEdge, ChannelHop, IBCChannelGraph};
pub use hash::{compute_ibc_denom_hash, hop_count_from_trace_path};
pub use normalize::{
    build_channel_to_chain_map, finalize_channels_for_ibc_entry, ordering_to_str,
};
pub use observe::{
    compare_balance, compare_predict_observe, compare_predict_observe_with, CompareOptions,
    FixtureBackend, ObserveBackend, ObservedBalance, ObservedChannel, ObservedDenomTrace,
    SnapshotBackend,
};
pub use predict::{check_invariants, PredictedWorld};
pub use routes::{
    derive_terp_ibc_denom, IBCAssetRoute, IBCAssetRoutingTable, RoutingTableMetadata,
    TerpChannelInfo,
};
pub use schema::{validate_asset_entry, validate_ibc_data_entry};

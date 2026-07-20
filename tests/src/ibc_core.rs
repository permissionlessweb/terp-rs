//! Back-compat re-export of `scripts::ibc` pure IBC derivation logic.
//!
//! Prefer `scripts::ibc` for new code. Existing `use scripts::ibc_core::...` keeps working.

pub use crate::ibc::{
    build_channel_to_chain_map, compute_ibc_denom_hash, derive_terp_ibc_denom, hop_count_from_trace_path,
    ChannelEdge, ChannelHop, IBCAssetRoute, IBCAssetRoutingTable, IBCChannelGraph,
    RoutingTableMetadata, TerpChannelInfo,
};

//! Crosslink light client verification primitives.
//!
//! This crate provides the core types and verification logic for the
//! Crosslink Trailing Finality Layer (TFL) IBC light client. It is
//! IBC-agnostic and can be used independently of the CosmWasm contract.
//!
//! All types are self-contained — no dependency on the full zebra-crosslink
//! node crate. This makes the library suitable for CosmWasm contracts.

pub mod client_state;
pub mod consensus_state;
pub mod error;
pub mod header;
pub mod membership;
pub mod misbehaviour;
pub mod types;
pub mod update;
pub mod verify;

pub use client_state::{ClientState, FinalizerEntry};
pub use consensus_state::ConsensusState;
pub use error::CrosslinkIBCError;
pub use header::CrosslinkHeader;
use zebra_crosslink::chain::ZcashCrosslinkParameters;
pub use zebra_crosslink::{
    FatPointerSignature2, FatPointerToBftBlock2, SerializationError, ZcashDeserialize,
    ZcashDeserializeInto, ZcashSerialize,
    chain::{BcBlockHeader, BftBlock, Blake3Hash, PROTOTYPE_PARAMETERS},
};

// #[derive(Clone, Debug,PartialEq)]
// pub struct ZcashCrosslinkParameters {
//     /// The best-chain confirmation depth, `σ`
//     ///
//     /// At least this many PoW blocks must be atop the PoW block used to obtain a finalized view.
//     pub bc_confirmation_depth_sigma: u64,

//     /// The depth of unfinalized PoW blocks past which "Stalled Mode" activates, `L`
//     ///
//     /// Quoting from [Zcash Trailing Finality Layer §3.3.3 Stalled Mode](https://electric-coin-company.github.io/tfl-book/design/crosslink/construction.html#stalled-mode):
//     ///
//     /// > In practice, L should be at least 2σ.
//     pub finalization_gap_bound: u64,
// }

// impl Default for ZcashCrosslinkParameters {
//     fn default() -> Self {
//         Self {
//             bc_confirmation_depth_sigma: PROTOTYPE_PARAMETERS.bc_confirmation_depth_sigma,
//             finalization_gap_bound: PROTOTYPE_PARAMETERS.finalization_gap_bound,
//         }
//     }
// }

// impl From<ZcashCrosslinkParameters> for ZcashCrosslinkParameters {
//     fn from(v: ZcashCrosslinkParameters) -> Self {
//         Self {
//             bc_confirmation_depth_sigma: v.bc_confirmation_depth_sigma,
//             finalization_gap_bound: v.bc_confirmation_depth_sigma,
//         }
//     }
// }
// impl Into<ZcashCrosslinkParameters> for ZcashCrosslinkParameters {
//     fn into(self) -> ZcashCrosslinkParameters {
//         ZcashCrosslinkParameters {
//             bc_confirmation_depth_sigma: self.bc_confirmation_depth_sigma,
//             finalization_gap_bound: self.finalization_gap_bound,
//         }
//     }
// }

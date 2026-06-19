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
pub use types::{
    BftBlock, Blake3Hash, FatPointerSignature2, FatPointerToBftBlock2, PowHeader,
    SerializationError, ZcashCrosslinkParameters, ZcashDeserialize, ZcashSerialize,
    PROTOTYPE_PARAMETERS,
};
//! hash-market library.
//!
//! Feature gates:
//! - `msg` — protobuf message types
//! - `custody` — key management and signing
//! - `transport` — provider transport modes (grpc, http_poll, websocket)
//! - `server` — validator sidecar HTTP endpoints
//! - `client` — hashmerchant client implementations (includes nostr)
//! - `nostr` — Nostr relay integration (NIP-77, NIP-87)
//! - `blossom` — BUD protocol server (requires server)
#[cfg(test)]
pub mod tests;

pub mod config;
pub mod fields;
pub mod hash;
pub mod msg;
pub mod server;
pub mod store;

#[cfg(feature = "custody")]
pub mod custody;

#[cfg(feature = "transport")]
pub mod transport;

#[cfg(feature = "server")]
pub mod middleware;

#[cfg(feature = "client")]
pub mod client;

#[cfg(feature = "ve")]
pub mod ve;

#[cfg(feature = "eth")]
pub use client::eth::{EthBlock, EthClient, EthProofResponse};

#[cfg(feature = "ve")]
pub use ve::{
    router, run_provider_feeder, ExtendVoteAllRequest, ExtendVoteRequest, ExtendVoteResponse,
    ProviderData, ProviderKey, ProviderStatus,
};

#[cfg(feature = "ve")]
pub use crate::client::ve::{SignedVoteExtension, VoteExtensionHandler};

#[cfg(feature = "nostr")]
pub use client::nostr::{
    HashMerchantKind, HashMerchantPayload, HashMerchantRelayInfo, NegentropyFilter,
};
 
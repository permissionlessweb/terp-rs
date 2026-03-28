//! # ict-rs — Interchain Test for Rust
//!
//! A Rust re-implementation of the [Interchain Test](https://github.com/strangelove-ventures/interchaintest)
//! framework for spinning up complex local multi-chain environments for testing and CI.
//!
//! ## Features
//!
//! - **Multi-chain**: Cosmos, Ethereum, Penumbra, and more
//! - **Multi-node**: Any number of validators and full nodes per chain
//! - **IBC**: Full relayer support (Hermes, CosmosRly)
//! - **Pluggable runtime**: Docker (default) or Kuasar lightweight sandboxes
//! - **Derive macros**: Auto-generate typed chain interaction functions from protobuf definitions
//! - **Custom authentication**: Pluggable signing backends (keyring, Ledger, KMS, etc.)
//!
//! ## Quick Start
//!
//! ```ignore
//! use ict_rs::prelude::*;
//!
//! #[tokio::test]
//! async fn test_ibc_transfer() {
//!     let ic = Interchain::new(IctRuntime::Docker(Default::default()))
//!         .unwrap()
//!         .add_chain(cosmos_chain)
//!         .add_relayer("hermes", hermes)
//!         .add_link(InterchainLink {
//!             chain1: "chain-a".into(),
//!             chain2: "chain-b".into(),
//!             relayer: "hermes".into(),
//!             path: "transfer".into(),
//!         });
//!
//!     ic.build(InterchainBuildOptions {
//!         test_name: "test_ibc_transfer".into(),
//!         skip_path_creation: false,
//!     }).await.unwrap();
//!
//!     // ... test logic ...
//!
//!     ic.close().await.unwrap();
//! }
//! ```

pub mod auth;
pub mod chain;
pub mod cli;
pub mod cosmwasm;
pub mod error;
pub mod genesis;
pub mod governance;
pub mod ibc;
pub mod interchain;
pub mod node;
pub mod relayer;
pub mod reporter;
pub mod runtime;
pub mod sidecar;
pub mod spec;
pub mod tx;
pub mod wallet;

pub mod modules;

#[cfg(feature = "testing")]
pub mod testing;

/// Re-export derive macros.
pub use ict_rs_derive::{ExecuteFns, QueryFns};

/// Convenience re-exports for common usage.
pub mod prelude {
    pub use crate::auth::Authenticator;
    pub use crate::chain::{Chain, ChainConfig, ChainType, SidecarConfig};
    pub use crate::cosmwasm::CosmWasmExt;
    pub use crate::error::{IctError, Result};
    pub use crate::governance::GovernanceExt;
    pub use crate::ibc::{ibc_denom, ibc_denom_multi_hop, ChannelOptions, ClientOptions};
    pub use crate::interchain::{wait_for_blocks, Interchain, InterchainBuildOptions, InterchainLink};
    pub use crate::relayer::{build_relayer, DockerRelayer, Relayer, RelayerType};
    pub use crate::runtime::{DockerImage, IctRuntime, RuntimeBackend};
    pub use crate::sidecar::SidecarProcess;
    pub use crate::spec::ChainSpec;
    pub use crate::tx::{ExecOutput, Tx, TransferOptions, WalletAmount};
    pub use crate::wallet::Wallet;

    pub use crate::{ExecuteFns, QueryFns};
}

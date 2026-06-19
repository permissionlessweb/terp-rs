#![doc = include_str!("../README.md")]
#![deny(clippy::nursery, clippy::pedantic, warnings)]
#![allow(missing_docs, unused_crate_dependencies)]

pub mod contract;
pub mod custom_query;
mod error;
pub mod instantiate;
pub mod msg;
pub mod query;
pub mod state;
pub mod sudo;

#[cfg(feature = "interface")]
pub mod interface;

pub use error::ContractError;
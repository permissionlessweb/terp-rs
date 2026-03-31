//! spec: `docs/zk-headstash/suite.md`
#[macro_use]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

use alloc::vec::Vec;

pub mod suite;
pub mod unit;
pub mod circuits;
pub use suite::HeadstashSuite;

/// One `ZkWasmVmSuite` implementation per test circuit.
/// Mirrors the cw-orch `Uploadable` pattern used by contracts.
#[cfg(feature = "interface")]
pub mod no_rick_suite;
#[cfg(feature = "interface")]
pub use no_rick_suite::NoRickSuite;

//! crate of various circuits

pub mod posiedon;
pub mod secp256k1_chip;
pub mod sinsemilla_commitdomain;
pub mod sinsemilla_hashdomain;

// Re-export the main test circuit for convenience
pub use sinsemilla_hashdomain::LeafHashTestCircuit;

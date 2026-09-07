//! Error types for the Crosslink light client.

use thiserror::Error;

/// Errors that can occur during light client operations.
#[derive(Error, Debug)]
pub enum CrosslinkIBCError {
    /// The fat pointer is invalid (wrong format, missing signatures, etc.)
    #[error("invalid fat pointer: {0}")]
    InvalidFatPointer(String),

    /// The BFT block is structurally invalid.
    #[error("invalid bft block: {0}")]
    InvalidBftBlock(String),

    #[error("{0}")]
    FromHexError(#[from] hex::FromHexError),

    #[error("{0}")]
    VerificationError(#[from] cosmwasm_std::VerificationError),

    /// Ed25519 signature verification failed.
    #[error("signature verification failed")]
    SignatureVerificationFailed,

    /// Header verification failed for a specific reason.
    #[error("header verification failed: {0}")]
    HeaderVerificationFailed(String),

    /// Membership verification failed.
    #[error("membership verification failed: {0}")]
    MembershipVerificationFailed(String),

    /// Misbehaviour (equivocation) detected.
    #[error("misbehaviour detected")]
    MisbehaviourDetected,
}

use cosmwasm_std::{StdError, VerificationError};
use cw_ownable::OwnershipError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Verification(#[from] VerificationError),

    #[error("{0}")]
    Ownership(#[from] OwnershipError),

    #[error("unauthorized")]
    Unauthorized {},

    #[error("invalid ed25519 public key: {reason}")]
    InvalidPubkey { reason: String },

    #[error("ed25519 signature verification failed: {reason}")]
    BadSignature { reason: String },
}

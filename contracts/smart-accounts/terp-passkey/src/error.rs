use cosmwasm_std::StdError;
use cw_ownable::OwnershipError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Ownership(#[from] OwnershipError),

    #[error("unauthorized")]
    Unauthorized {},

    #[error("missing authenticator params")]
    MissingParams {},

    #[error("invalid passkey credential: {reason}")]
    InvalidCredential { reason: String },
}

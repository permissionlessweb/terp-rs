use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("unauthorized")]
    Unauthorized {},

    #[error("issuer not registered: {issuer}")]
    UnknownIssuer { issuer: String },

    #[error("claim commitment not registered")]
    UnregisteredClaim {},

    #[error("invalid zk-jwt proof: {reason}")]
    InvalidProof { reason: String },

    #[error("nullifier already used")]
    NullifierReplay {},

    #[error("missing authenticator params")]
    MissingParams {},
}

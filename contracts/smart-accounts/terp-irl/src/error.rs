use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("unauthorized")]
    Unauthorized {},

    #[error("missing authenticator metadata")]
    MissingAuthenticatorMetadata {},

    #[error("invalid witness proof: {reason}")]
    InvalidWitness { reason: String },
}

use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DemoError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("invalid toy proof: {reason}")]
    InvalidProof { reason: String },
}

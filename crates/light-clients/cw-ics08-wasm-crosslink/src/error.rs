//! Defines the [`ContractError`] type.

use cosmwasm_std::StdError;
use crosslink_light_client::error::CrosslinkIBCError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    IoError(#[from] std::io::Error),

    #[error("{0}")]
    SerializationError(#[from] crosslink_light_client::SerializationError),

    #[error("client state latest height and slot are not equal")]
    ClientStateSlotMismatch,

    #[error("client and consensus state mismatch")]
    ClientAndConsensusStateMismatch,

    #[error("serializing client state failed: {0}")]
    SerializeClientStateFailed(#[source] serde_json::Error),

    #[error("serializing consensus state failed: {0}")]
    SerializeConsensusStateFailed(#[source] serde_json::Error),

    #[error("deserializing client state failed: {0}")]
    DeserializeClientStateFailed(#[source] serde_json::Error),

    #[error("deserializing consensus state failed: {0}")]
    DeserializeConsensusStateFailed(#[source] serde_json::Error),

    #[error("deserializing client message failed: {0}")]
    DeserializeClientMessageFailed(#[source] serde_json::Error),

    #[error("verify membership failed: {0}")]
    VerifyMembershipFailed(#[source] CrosslinkIBCError),

    #[error("verify non-membership failed: {0}")]
    VerifyNonMembershipFailed(#[source] CrosslinkIBCError),

    #[error("verify client message failed: {0}")]
    VerifyClientMessageFailed(#[source] CrosslinkIBCError),

    #[error("update client state failed: {0}")]
    UpdateClientStateFailed(#[source] CrosslinkIBCError),

    #[error("unsupported fork version")]
    UnsupportedForkVersion(String),

    #[error("client state not found")]
    ClientStateNotFound,

    #[error("consensus state not found")]
    ConsensusStateNotFound,

    #[error("prost encoding error: {0}")]
    ProstEncodeError(#[from] prost::EncodeError),

    #[error("prost decoding error: {0}")]
    ProstDecodeError(#[from] prost::DecodeError),

    #[error("serde json error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("invalid client message")]
    InvalidClientMessage,
}

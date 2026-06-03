use thiserror::Error;

#[derive(Error, Debug)]
pub enum NipError {
    #[error("Invalid public key length: expected 64 hex chars, got {0}")]
    InvalidPubKeyLength(usize),
    #[error("Invalid event ID length: expected 64 hex chars, got {0}")]
    InvalidEventIdLength(usize),
    #[error("Invalid signature length: expected 128 hex chars, got {0}")]
    InvalidSignatureLength(usize),
    #[error("Invalid hex format")]
    InvalidHexFormat,
    #[error("Hex decode error: {0}")]
    HexDecodeError(#[from] hex::FromHexError),
    #[error("Invalid relay URL: {0}")]
    InvalidRelayUrl(String),
    #[error("Invalid kind: {0}")]
    InvalidKind(u16),
    #[error("Missing required field: {0}")]
    MissingField(String),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Tag error: {0}")]
    Tag(String),
    #[error("NIP-01 error: {0}")]
    Nip01(String),
    #[error("NIP-15 error: {0}")]
    Nip15(String),
    #[error("NIP-52 error: {0}")]
    Nip52(String),
    #[error("NIP-77: {0}")]
    Nip77(String),
    #[error("NIP-87: {0}")]
    Nip87(String),
    #[error("Crypto error: {0}")]
    Crypto(String),
}

pub type NipResult<T> = Result<T, NipError>;

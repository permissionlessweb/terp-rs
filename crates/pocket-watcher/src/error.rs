use cosmwasm_std::StdError;

#[derive(thiserror::Error, Debug)]
pub enum QueryError {
    #[error("protobuf encode error: {0}")]
    Encode(#[from] prost::EncodeError),
    #[error("protobuf decode error: {0}")]
    Decode(#[from] prost::DecodeError),
    #[error("cosmwasm error: {0}")]
    Std(#[from] StdError),
}

pub type QueryResult<T> = Result<T, QueryError>;

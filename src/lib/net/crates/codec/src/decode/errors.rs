use crate::net_types::NetTypesError;

#[derive(Debug, thiserror::Error)]
pub enum NetDecodeError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid UTF-8: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),

    #[error("External error: {0}")]
    ExternalError(#[from] Box<dyn std::error::Error + Send + Sync>),

    #[error("Invalid Enum Variant")]
    InvalidEnumVariant,

    #[error("Net Type Error: {0}")]
    NetTypeError(#[from] NetTypesError),

    #[error("Async decoding not supported for this type")]
    AsyncNotSupported,
}

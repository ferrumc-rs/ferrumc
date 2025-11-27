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
}

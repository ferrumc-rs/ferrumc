use ferrumc_config::statics::get_global_config;

#[derive(Debug, thiserror::Error)]
pub enum NetDecodeError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid length: expected {expected}, got {actual}")]
    InvalidLength {
        expected: usize,
        actual: usize,
        field: String,
    },

    #[error("Invalid UTF-8: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),

    #[error("Compressed packet smaller than threshold. 'data_length' = {0}, but threshold is {threshold}", threshold = get_global_config().network_compression_threshold)]
    CompressedPacketTooSmall(usize),

    #[error("External error: {0}")]
    ExternalError(#[from] Box<dyn std::error::Error + Send + Sync>),

    #[error("Invalid Enum Variant")]
    InvalidEnumVariant,
}

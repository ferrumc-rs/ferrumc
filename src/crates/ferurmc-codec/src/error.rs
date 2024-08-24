pub type Result<T> = std::result::Result<T, CodecError>;


#[derive(Debug, thiserror::Error)]
pub enum CodecError {
    #[error("IO error: {0}")]
    Io(#[from] tokio::io::Error),
    #[error("Double conversion")]
    DoubleConversion,
    #[error("VarInt too big")]
    VarIntTooBig,
    #[error("VarLong too big")]
    VarLongTooBig,
}
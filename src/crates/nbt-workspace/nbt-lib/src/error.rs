#[derive(thiserror::Error, Debug)]
pub enum NBTError {
    #[error("NBT Serialization Error: {0}")]
    SerializeError(String),
    #[error("NBT, couldn't modify cursor")]
    ReadWriteError(#[from] std::io::Error),
    #[error("NBT, Deserialization Error: {0}")]
    DeserializeError(String),
    #[error("NBT, Unexpected EOF")]
    UnexpectedEOF,
    #[error("NBT, couldn't read string")]
    StringReadError(#[from] std::str::Utf8Error),
    /// (expected, actual)
    #[error("NBT, expected tag type {0}, got {1}")]
    InvalidType(&'static str, &'static str),
}

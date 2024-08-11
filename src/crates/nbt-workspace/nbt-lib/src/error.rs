#[derive(thiserror::Error, Debug)]
pub enum NBTError {
    #[error("NBT Serialization Error: {0}")]
    SerializeError(String),
    #[error("NBT, couldn't write to buffer")]
    WriteError(#[from] std::io::Error),
}
use crate::codec::encode::errors::NetEncodeError;
use crate::codec::net_types::NetTypesError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("Encoding Error: {0}")]
    Encode(#[from] NetEncodeError),

    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("NBT Error: {0}")]
    Nbt(String), // Or wrap the specific NBT error if possible

    #[error("Chunk Error: {0}")]
    Chunk(String),

    #[error(transparent)]
    NetTypes(#[from] NetTypesError),
}

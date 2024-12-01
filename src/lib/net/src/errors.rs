use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_encryption::errors::NetEncryptionError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NetError {
    #[error("Encryption Error: {0}")]
    EncryptionError(#[from] NetEncryptionError),

    #[error("Decoder Error: {0}")]
    DecoderError(#[from] NetDecodeError),

    #[error("Encoder Error: {0}")]
    EncoderError(#[from] NetEncodeError),

    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),
    
    #[error("Addr parse error: {0}")]
    AddrParseError(#[from] std::net::AddrParseError),

    #[error("Task Error: {0}")]
    TaskError(#[from] tokio::task::JoinError),


    #[error("UTF8 Error: {0}")]
    UTF8Error(#[from] std::string::FromUtf8Error),

    #[error("VarInt Error: {0}")]
    TypesError(#[from] ferrumc_net_codec::net_types::NetTypesError),

    #[error("ECS Error: {0}")]
    ECSError(#[from] ferrumc_ecs::errors::ECSError),

    #[error("Events Error: {0}")]
    EventsError(#[from] ferrumc_events::errors::EventsError),

    #[error("Invalid State: {0}")]
    InvalidState(u8),

    #[error("{0}")]
    Packet(#[from] PacketError),
    
    #[error("{0}")]
    Chunk(#[from] ChunkError),
}

#[derive(Debug, Error)]
pub enum PacketError {
    #[error("Invalid State: {0}")]
    InvalidState(u8),
}

#[derive(Debug, Error)]
pub enum ChunkError {
    #[error("Invalid Chunk: ({0}, {1})")]
    InvalidChunk(i32, i32),
}

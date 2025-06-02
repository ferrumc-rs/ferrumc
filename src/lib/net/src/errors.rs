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
    IOError(std::io::Error),

    #[error("Connection Dropped")]
    ConnectionDropped,

    #[error("Addr parse error: {0}")]
    AddrParseError(#[from] std::net::AddrParseError),

    #[error("UTF8 Error: {0}")]
    UTF8Error(#[from] std::string::FromUtf8Error),

    #[error("VarInt Error: {0}")]
    TypesError(ferrumc_net_codec::net_types::NetTypesError),

    #[error("ECS Error: {0}")]
    ECSError(bevy_ecs::error::BevyError),

    #[error("Invalid State: {0}")]
    InvalidState(u8),

    #[error("Mismatched Protocol Version: {0} != {1}")]
    MismatchedProtocolVersion(i32, i32),

    #[error("Handshake timeout")]
    HandshakeTimeout,

    #[error("Packet error: {0}")]
    Packet(PacketError),

    #[error("Chunk error: {0}")]
    Chunk(#[from] ChunkError),

    #[error("World error: {0}")]
    World(#[from] ferrumc_world::errors::WorldError),

    #[error("Misc error: {0}")]
    Misc(String),
}

#[derive(Debug, Error)]
pub enum PacketError {
    #[error("Invalid State: {0}")]
    InvalidState(u8),
    #[error("Invalid Packet: {0:02X}")]
    InvalidPacket(u8),
}

#[derive(Debug, Error)]
pub enum ChunkError {
    #[error("Invalid Chunk: ({0}, {1})")]
    InvalidChunk(i32, i32),
}

impl From<std::io::Error> for NetError {
    fn from(err: std::io::Error) -> Self {
        use std::io::ErrorKind::*;
        match err.kind() {
            ConnectionAborted | ConnectionReset | UnexpectedEof => NetError::ConnectionDropped,
            _ => NetError::IOError(err),
        }
    }
}
impl From<ferrumc_net_codec::net_types::NetTypesError> for NetError {
    fn from(err: ferrumc_net_codec::net_types::NetTypesError) -> Self {
        use ferrumc_net_codec::net_types::NetTypesError;
        use std::io::ErrorKind;

        if let NetTypesError::Io(io_err) = &err {
            if io_err.kind() == ErrorKind::UnexpectedEof {
                return NetError::ConnectionDropped;
            }
        }
        NetError::TypesError(err)
    }
}

impl From<PacketError> for NetError {
    fn from(err: PacketError) -> Self {
        NetError::Packet(err)
    }
}

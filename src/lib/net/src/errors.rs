use crate::ConnState;
use ferrumc_config::server_config::get_global_config;
use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_encryption::errors::NetEncryptionError;
use std::error::Error;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NetError {
    #[error("Encryption Error: {0}")]
    EncryptionError(#[from] NetEncryptionError),

    #[error("Authentication Error: {0}")]
    AuthenticationError(#[from] NetAuthenticationError),

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

    #[error("Compression error: {0}")]
    CompressionError(#[from] CompressionError),

    #[error("Misc error: {0}")]
    Misc(String),
}

#[derive(Debug, Error)]
pub enum CompressionError {
    #[error("Compressed packet smaller than threshold. 'data_length' = {0}, but threshold is {threshold}", threshold = get_global_config().network_compression_threshold
    )]
    CompressedPacketTooSmall(usize),

    #[error("Checksum mismatch: expected {expected:02X}, got {received:02X}")]
    ChecksumMismatch { expected: u32, received: u32 },

    #[error("Missing checksum in compressed packet")]
    MissingChecksum,

    #[error("Generic decompression error: {0}")]
    GenericDecompressionError(String),

    #[error("Generic compression error: {0}")]
    GenericCompressionError(String),

    #[error("Packet likely uncompressed, but compression is enabled")]
    PacketUncompressedWithCompressionEnabled,
}

#[derive(Debug, Error)]
pub enum PacketError {
    #[error("Invalid State: {0}")]
    InvalidState(u8),

    #[error("Invalid Packet: {0:02X}")]
    InvalidPacket(u8),

    #[error("Malformed Packet: {inp}", inp = if let Some(id) = .0 { format!("{id:02X}") } else { "None".to_string() }
    )]
    MalformedPacket(Option<u8>),

    #[error(
        "Unexpected Packet: expected 0X{expected:02X}, received 0X{received:02X} in state {state}"
    )]
    UnexpectedPacket {
        expected: u8,
        received: u8,
        state: ConnState,
    },
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

#[derive(Debug, Clone, Error)]
pub enum NetAuthenticationError {
    #[error("Failed to reach Mojang's authentication servers")]
    CouldNotReachMojang,

    #[error("Bad URL used to reach Mojang")]
    BadURL,

    #[error("The server has exceeded the rate limit allowed by Mojang")]
    RateLimitReached,

    #[error("The user could not be authenticated")]
    FailedToAuthenticate,

    #[error("Player's reported information does not match Mojang's information")]
    InformationDoesntMatch,

    #[error("Could not parse auth server response: {0}")]
    ParseError(#[from] Arc<dyn Error + Send + Sync>),

    #[error("Mojang returned a corrupted UUID")]
    CorruptUuid,

    #[error("Mojang responded with status code {0}")]
    UnknownStatusError(u16),
}

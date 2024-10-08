use ferrumc_net_encryption::errors::NetEncryptionError;
use ferrumc_net_packets::errors::NetPacketError;
use thiserror::Error;
use ferrumc_net_codec::r#mod::errors::NetDecodeError;

#[derive(Debug, Error)]
pub enum NetError {
    #[error("Something failed lol")]
    Something,

    #[error("Encryption Error: {0}")]
    EncryptionError(#[from] NetEncryptionError),

    #[error("Packet Error: {0}")]
    PacketError(#[from] NetPacketError),

    #[error("Decoder Error: {0}")]
    DecoderError(#[from] NetDecodeError),
}

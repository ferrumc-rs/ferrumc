use ferrumc_net_encryption::errors::NetEncryptionError;
use ferrumc_net_packets::errors::NetPacketError;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum NetError {
    #[error("Something failed lol")]
    Something,

    #[error("Encryption Error: {0}")]
    EncryptionError(#[from] NetEncryptionError),

    #[error("Packet Error: {0}")]
    PacketError(#[from] NetPacketError),
}

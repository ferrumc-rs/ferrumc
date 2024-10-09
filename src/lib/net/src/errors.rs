use ferrumc_net_encryption::errors::NetEncryptionError;
use thiserror::Error;
use ferrumc_net_codec::decode::errors::NetDecodeError;

#[derive(Debug, Error)]
pub enum NetError {
    #[error("Something failed lol")]
    Something(),

    #[error("Encryption Error: {0}")]
    EncryptionError(#[from] NetEncryptionError),

    #[error("Decoder Error: {0}")]
    DecoderError(#[from] NetDecodeError),

    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),
    
    #[error("VarInt Error: {0}")]
    TypesError(#[from] ferrumc_net_codec::net_types::NetTypesError),
    
    #[error("ECS Error: {0}")]
    ECSError(#[from] ferrumc_ecs::errors::ECSError),
    
    #[error("Events Error: {0}")]
    EventsError(#[from] ferrumc_events::errors::EventsError),
    
    
}


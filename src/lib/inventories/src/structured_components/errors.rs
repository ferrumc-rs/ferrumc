use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_codec::net_types::var_int::VarInt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StructuredComponentError {
    #[error("Protocol violation: {0}")]
    ProtocolViolation(&'static str),

    #[error("Invalid StructuredComponent Enum")]
    InvalidEnum,

    #[error("This StructuredComponent is not supported. Id : {0}")]
    NotSupported(VarInt),

    #[error("{limit_type} exceeds maximum allowed limit. Received: {actual}, Max: {max_limit}")]
    MaxLimitExceeded {
        limit_type: &'static str,
        actual: usize,
        max_limit: usize,
    },
}

impl From<StructuredComponentError> for NetEncodeError {
    fn from(value: StructuredComponentError) -> Self {
        NetEncodeError::ExternalError(Box::new(value))
    }
}

impl From<StructuredComponentError> for NetDecodeError {
    fn from(value: StructuredComponentError) -> Self {
        NetDecodeError::ExternalError(Box::new(value))
    }
}
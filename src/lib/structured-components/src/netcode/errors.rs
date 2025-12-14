use std::error::Error;
use std::fmt;
use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(Debug)]
pub struct ProtocolViolationError(pub &'static str);

impl fmt::Display for ProtocolViolationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Protocol violation: {}", self.0)
    }
}

impl Error for ProtocolViolationError {}

#[derive(Debug)]
pub struct InvalidStructuredComponentEnumError();

impl fmt::Display for InvalidStructuredComponentEnumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid StructuredComponent Enum")
    }
}

impl Error for InvalidStructuredComponentEnumError {}

impl From<InvalidStructuredComponentEnumError> for NetEncodeError {
    fn from(value: InvalidStructuredComponentEnumError) -> Self {
        NetEncodeError::ExternalError(Box::new(value))
    }
}

#[derive(Debug)]
pub struct NotSupportedStructuredComponentError(pub VarInt);

impl fmt::Display for NotSupportedStructuredComponentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "This StructuredComponent is not supported. Id : {}", self.0)
    }
}

impl Error for NotSupportedStructuredComponentError {}

impl From<NotSupportedStructuredComponentError> for NetEncodeError {
    fn from(value: NotSupportedStructuredComponentError) -> Self {
        NetEncodeError::ExternalError(Box::new(value))
    }
}

impl From<NotSupportedStructuredComponentError> for NetDecodeError {
    fn from(value: NotSupportedStructuredComponentError) -> Self {
        NetDecodeError::ExternalError(Box::new(value))
    }
}
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum NetEncryptionError {
    #[error("Verify token does not match token sent by server")]
    VerifyTokenMismatch {
        expected: Vec<u8>,
        returned: Vec<u8>,
    },
    #[error("Failed to convert public key to DER format")]
    DERConversionError,
    #[error("Something failed lol")]
    SomeError,
}

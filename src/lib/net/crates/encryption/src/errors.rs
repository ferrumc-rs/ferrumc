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
    #[error("Failed to decrypt bytes with RSA private key")]
    RSADecryptionError,
    #[error("Player shared secret key holder poisoned")]
    SharedKeyHolderPoisoned,
    #[error("Something failed lol")]
    SomeError,
}

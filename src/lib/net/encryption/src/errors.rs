use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum NetEncryptionError {
    #[error("Error in encrypting data with RSA")]
    RsaEncryptionError,
    #[error("Error in decrypting data with RSA")]
    RsaDecryptionError,
    #[error("Error in generating RSA keypair")]
    RsaKeyGenerationError,
    #[error("Error in encoding public key to ASN.1 DER format")]
    RsaKeyEncodingError,
}

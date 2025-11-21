use std::ops::Deref;
use std::sync::LazyLock;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use rsa::pkcs8::EncodePublicKey;
use crate::errors::NetEncryptionError;

pub mod errors;
pub mod read;
pub mod write;

/// The global EncryptionKeys instance to be used for encryption/decryption.
static ENCRYPTION_KEYS: LazyLock<EncryptionKeys> = LazyLock::new(|| EncryptionKeys::generate());

/// Struct to hold:
/// - A RSA private key
/// - The public key associated with said private key
/// - The public key encoded in DER format as specified by the Minecraft protocol
pub struct EncryptionKeys {
    pub public_key: RsaPublicKey,
    pub private_key: RsaPrivateKey,
    der_format: Vec<u8>,
}

impl EncryptionKeys {
    /// Generates a 1024-bit RSA key pair to be used with the network protocol.
    ///
    /// # Returns
    /// - `Self`: A new EncryptionKeys instance with a random RSA key pair.
    pub fn generate() -> Self {
        let private_key = RsaPrivateKey::new(&mut rand::rng(), 1024).expect("RsaPrivateKey failed to generate");
        let public_key = RsaPublicKey::from(&private_key);

        let der = public_key
            .to_public_key_der()
            .expect("der format should be supported")
            .to_vec();

        Self {
            public_key,
            private_key,
            der_format: der,
        }
    }

    /// Clones the DER formatted public key to be sent to the client when enabling encryption.
    ///
    /// # Returns
    /// - `Vec<u8>`: The cloned public key in DER format.
    pub fn clone_der(&self) -> Vec<u8> {
        self.der_format.clone()
    }

    /// Decrypts a byte array using this struct's private key.
    ///
    /// # Parameters
    /// - `data`: The data to be decrypted.
    ///
    /// # Returns
    /// - `Vec<u8>`: On success, the decrypted bytes.
    ///
    /// # Errors
    /// Returns `NetEncryptionError::RSADecryptionError` if decryption fails.
    pub fn decrypt_bytes(&self, data: &[u8]) -> Result<Vec<u8>, NetEncryptionError> {
        Ok(self.private_key.decrypt(Pkcs1v15Encrypt::default(), data)
            .map_err(|_| NetEncryptionError::RSADecryptionError)?
            .to_vec())
    }
}

/// Gets the current global EncryptionKeys instance.
///
/// # Returns
/// The current EncryptionKey instance.
pub fn get_encryption_keys() -> &'static EncryptionKeys {
    ENCRYPTION_KEYS.deref()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_key_generation() {

    }
}

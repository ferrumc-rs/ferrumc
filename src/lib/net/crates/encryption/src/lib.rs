use std::ops::Deref;
use std::sync::LazyLock;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use rsa::pkcs8::EncodePublicKey;
use crate::errors::NetEncryptionError;

pub mod errors;
pub mod cipher;

/// The generated key pair to use for encryption/decryption
static ENCRYPTION_KEYS: LazyLock<EncryptionKeys> = LazyLock::new(|| EncryptionKeys::generate());

/// Helper struct to store public and private keys for the server
pub struct EncryptionKeys {
    pub public_key: RsaPublicKey,
    pub private_key: RsaPrivateKey,
    der_format: Vec<u8>,
}

impl EncryptionKeys {
    /// Generates a 1024-bit RSA key pair to be used with the network protocol
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

    pub fn clone_der(&self) -> Vec<u8> {
        self.der_format.clone()
    }

    pub fn decrypt_bytes(&self, data: &[u8]) -> Result<Vec<u8>, NetEncryptionError> {
        Ok(self.private_key.decrypt(Pkcs1v15Encrypt::default(), data)
            .map_err(|_| NetEncryptionError::SomeError)? // TODO: more descriptive error
            .to_vec())
    }
}

pub fn get_encryption_keys() -> &'static EncryptionKeys {
    ENCRYPTION_KEYS.deref()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_key_generation() {

    }
}

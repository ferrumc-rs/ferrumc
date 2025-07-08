use std::sync::Arc;

use once_cell::sync::Lazy;
use rsa::{pkcs8::EncodePublicKey, rand_core::OsRng, RsaPrivateKey, RsaPublicKey};

pub mod digest;
pub mod errors;

#[cfg(test)]
mod tests;

pub static ENCRYPTION_KEYS: Lazy<EncryptionKeys> = Lazy::new(|| EncryptionKeys::new());

pub struct EncryptionKeys {
    pub private_key: Arc<RsaPrivateKey>,
    pub public_key: Arc<RsaPublicKey>,
}

impl EncryptionKeys {
    pub fn new() -> Self {
        let mut rng = OsRng;
        let private_key =
            RsaPrivateKey::new(&mut rng, 1024).expect("Failed to generate PEM key for encryption");
        let public_key = RsaPublicKey::from(private_key.clone());

        Self {
            private_key: Arc::new(private_key),
            public_key: Arc::new(public_key),
        }
    }

    pub fn get_public_der(&self) -> Vec<u8> {
        self.public_key
            .to_public_key_der()
            .unwrap()
            .as_bytes()
            .to_vec()
    }
}

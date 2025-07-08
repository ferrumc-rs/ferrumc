use std::sync::Arc;

use aes::{cipher::KeyInit, Aes128Dec, Aes128Enc};
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

#[derive(Clone)]
pub struct ConnectionEncryption {
    pub shared_secret: Vec<u8>,
    pub decrypt_cipher: Aes128Dec,
    pub encrypt_cipher: Aes128Enc,
}

impl ConnectionEncryption {
    pub fn new(shared_secret: Vec<u8>) -> Self {
        let decrypt_cipher = Aes128Dec::new_from_slice(&shared_secret).unwrap();
        let encrypt_cipher = Aes128Enc::new_from_slice(&shared_secret).unwrap();
        Self {
            shared_secret,
            decrypt_cipher,
            encrypt_cipher,
        }
    }

    pub fn encrypt(&mut self, data: &mut [u8]) {
        self.encrypt_cipher.encrypt(data);
    }

    pub fn decrypt(&mut self, data: &mut [u8]) {
        self.decrypt_cipher.decrypt(data);
    }
}

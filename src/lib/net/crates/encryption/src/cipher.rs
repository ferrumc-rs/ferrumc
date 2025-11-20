use aes::Aes128;
use aes::cipher::{AsyncStreamCipher, KeyIvInit};

type AesCfb8Encryptor = cfb8::Encryptor<Aes128>;
type AesCfb8Decryptor = cfb8::Decryptor<Aes128>;

pub struct EncryptionCipher {
    encryptor: AesCfb8Encryptor,
    decryptor: AesCfb8Decryptor,
}

impl EncryptionCipher {
    pub fn new(shared_key: &[u8]) -> EncryptionCipher {
        Self {
            encryptor: AesCfb8Encryptor::new_from_slices(shared_key, shared_key).unwrap(), // TODO: better error handling
            decryptor: AesCfb8Decryptor::new_from_slices(shared_key, shared_key).unwrap(), // TODO: better error handling
        }
    }

    pub fn encrypt(&self, data: &mut [u8]) {
        self.encryptor.encrypt(data)
    }

    pub fn decrypt(&self, data: &mut [u8]) {
        self.decryptor.decrypt(data)
    }
}
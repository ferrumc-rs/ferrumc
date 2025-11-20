use aes::cipher::generic_array::GenericArray;
use aes::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use aes::Aes128;
use log::debug;

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

    pub fn encrypt(&mut self, data: &mut [u8]) {
        for b in data.iter_mut() {
            let mut arr = [*b];
            let block = GenericArray::from_mut_slice(&mut arr);
            self.encryptor.encrypt_block_mut(block);
            *b = block[0];
        }
    }

    pub fn decrypt(&mut self, data: &mut [u8]) {
        for b in data.iter_mut() {
            let mut arr = [*b];
            let block = GenericArray::from_mut_slice(&mut arr);
            self.decryptor.decrypt_block_mut(block);
            *b = block[0];
        }
    }
}
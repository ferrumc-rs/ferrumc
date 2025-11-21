use std::sync::atomic::{AtomicBool, Ordering};
use aes::cipher::generic_array::GenericArray;
use aes::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use aes::Aes128;
use tokio::sync::Mutex as TokioMutex;

type AesCfb8Encryptor = cfb8::Encryptor<Aes128>;
type AesCfb8Decryptor = cfb8::Decryptor<Aes128>;

pub struct EncryptionCipher {
    encryptor: TokioMutex<AesCfb8Encryptor>,
    decryptor: TokioMutex<AesCfb8Decryptor>,
    should_encrypt: AtomicBool,
}

impl EncryptionCipher {
    pub fn new() -> EncryptionCipher {
        let key = [0; 16];

        Self {
            encryptor: TokioMutex::new(AesCfb8Encryptor::new_from_slices(&key, &key).unwrap()), // TODO: better error handling
            decryptor: TokioMutex::new(AesCfb8Decryptor::new_from_slices(&key, &key).unwrap()), // TODO: better error handling
            should_encrypt: AtomicBool::new(false),
        }
    }

    pub async fn update_keys(&self, shared_key: &[u8]) {
        {
            let mut lock = self.encryptor.lock().await;
            *lock = AesCfb8Encryptor::new_from_slices(&shared_key, &shared_key).unwrap(); // TODO: better error handling
            let mut lock = self.decryptor.lock().await;
            *lock = AesCfb8Decryptor::new_from_slices(&shared_key, &shared_key).unwrap(); // TODO: better error handling
        }

        self.should_encrypt.store(true, Ordering::Release);
    }

    pub async fn encrypt(&self, data: &mut [u8]) {
        if !self.should_encrypt.load(Ordering::Relaxed) { return; }

        {
            let mut lock = self.encryptor.lock().await;
            for b in data.chunks_mut(1) {
                let block = GenericArray::from_mut_slice(b);
                lock.encrypt_block_mut(block);
            }
        }
    }

    pub async fn decrypt(&self, data: &mut [u8]) {
        if !self.should_encrypt.load(Ordering::Relaxed) { return; }

        {
            let mut lock = self.decryptor.lock().await;
            for b in data.chunks_mut(1) {
                let block = GenericArray::from_mut_slice(b);
                lock.decrypt_block_mut(block);
            }
        }
    }
}
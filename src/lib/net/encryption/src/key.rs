use rsa::{RsaPublicKey, RsaPrivateKey, pkcs1::EncodeRsaPublicKey, Pkcs1v15Encrypt};
use rand::rngs::OsRng;
use crate::errors;

pub struct KeyPair {
    pub public_key: RsaPublicKey, //public key side of the keypair (for encryption)
    pub encoded_public_key: der::Document, //the public key encoded in ASN.1 DER format for publishing to the client
    pub private_key: RsaPrivateKey, //private key side of the keypair (for decryption)
}

impl KeyPair {
    pub fn new() -> Result<KeyPair, errors::NetEncryptionError> {
        let mut rng = OsRng;

        let private_key = RsaPrivateKey::new(&mut rng, 1024)
            .map_err(|_e| errors::NetEncryptionError::RsaKeyGenerationError)?;

        let public_key = RsaPublicKey::from(&private_key);

        let encoded_public_key = public_key.to_pkcs1_der()
            .map_err(|_e| errors::NetEncryptionError::RsaKeyEncodingError)?;

        Ok(Self {
            public_key,
            encoded_public_key,
            private_key,
        })
    }

    pub fn encrypt (&self, data: &[u8]) -> Result<Vec<u8>, errors::NetEncryptionError> {
        let mut rng = OsRng;

        self.public_key.encrypt(&mut rng, Pkcs1v15Encrypt, data)
            .map_err(|_e| errors::NetEncryptionError::RsaEncryptionError)
    }

    pub fn decrypt (&self, data: &[u8]) -> Result<Vec<u8>, errors::NetEncryptionError> {
        self.private_key.decrypt(Pkcs1v15Encrypt, data)
            .map_err(|_e| errors::NetEncryptionError::RsaDecryptionError)
    }
}

impl Default for KeyPair {
    fn default() -> KeyPair {
        Self::new().expect("Keypair generation failed")
    }
}

#[test]
fn test_encrypt_decrypt() {
    let keypair = KeyPair::new().expect("Failed keypair generation");
    let msg = "Hello World!";
    let msg_in_bytes: Vec<u8> = msg.to_string().into_bytes();
    let encrypted_msg = keypair.encrypt(&msg_in_bytes).expect("Failed to encrypt");
    let decrypted_msg = keypair.decrypt(&encrypted_msg).expect("Failed to decrypt");
    assert_eq!(decrypted_msg, msg_in_bytes);
}

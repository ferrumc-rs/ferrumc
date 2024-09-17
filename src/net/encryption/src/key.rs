use rsa::{RsaPublicKey, RsaPrivateKey, pkcs1::EncodeRsaPublicKey, Pkcs1v15Encrypt};
use rand::rngs::OsRng;

pub struct KeyPair {
    pub public_key: RsaPublicKey,
    pub encoded_public_key: der::Document,
    pub private_key: RsaPrivateKey,
}

impl KeyPair {
    pub fn new() -> Self {
        let mut rng = OsRng;
        let private_key = RsaPrivateKey::new(&mut rng, 1024).expect("Failed to generate a key");
        let public_key = RsaPublicKey::from(&private_key);
        let encoded_public_key = public_key.to_pkcs1_der().expect("Failed to encode public key");

        Self {
            public_key,
            encoded_public_key,
            private_key,
        }
    }

    pub fn encrypt (&self, data: &Vec<u8>) -> Vec<u8> {
        let mut rng = OsRng;
        self.public_key.encrypt(&mut rng, Pkcs1v15Encrypt, &data).expect("Failed to encrypt data")
    }

    pub fn decrypt (&self, data: &Vec<u8>) -> Vec<u8> {
        self.private_key.decrypt(Pkcs1v15Encrypt, &data).expect("Failed to decrypt data")
    }
}

#[test]
fn test_encrypt_decrypt() {
    let keypair = KeyPair::new();
    let msg = "Hello World!";
    let msg_in_bytes: Vec<u8> = msg.to_string().into_bytes();
    let encrypted_msg = keypair.encrypt(&msg_in_bytes);
    let decrypted_msg = keypair.decrypt(&encrypted_msg);
    assert_eq!(decrypted_msg, msg_in_bytes);
}

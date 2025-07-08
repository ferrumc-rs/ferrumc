use rsa::{
    pkcs1::EncodeRsaPublicKey, pkcs8::DecodePublicKey, rand_core::OsRng, Pkcs1v15Encrypt,
    RsaPrivateKey, RsaPublicKey,
};

use crate::{digest::get_player_digest, EncryptionKeys, ENCRYPTION_KEYS};

#[test]
fn test_pem_generation() {
    let keys = EncryptionKeys::new();
    let public_pem = keys
        .public_key
        .to_pkcs1_pem(Default::default())
        .expect("failed...");

    println!("Public Pem Encryption Key: {public_pem}");
    println!(
        "Public Pem Encryption Key (Bytes): {:?}",
        public_pem.as_bytes()
    );
}

#[test]
fn test_minecraft_hashes() {
    assert_eq!(
        get_player_digest("Notch"),
        "4ed1f46bbe04bc756bcb17c0c7ce3e4632f06a48"
    );
    assert_eq!(
        get_player_digest("jeb_"),
        "-7c9d5b0044c130109a5d7b5fb5c317c02b4e28c1"
    );
    assert_eq!(
        get_player_digest("simon"),
        "88e16a1019277b15d58faf0541e11910eb756f6"
    );
}

#[test]
fn test_decrypt() {
    let mut rng = OsRng;
    let priv_key = RsaPrivateKey::new(&mut rng, 1024).expect("failed to generate key");
    let pub_key = RsaPublicKey::from(&priv_key);

    let message = b"test secret";

    // Encrypt using the public key
    let enc = pub_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, message)
        .expect("encryption failed");

    // Decrypt using the private key
    let dec = priv_key
        .decrypt(Pkcs1v15Encrypt, &enc)
        .expect("decryption failed");

    assert_eq!(dec, message);
}

#[test]
fn test_encrypt_decrypt() {
    let mut rng = OsRng;

    let priv_key = RsaPrivateKey::new(&mut rng, 1024).unwrap();
    let pub_key = RsaPublicKey::from(&priv_key);

    let message = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; // under 117 bytes
    let encrypted = pub_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, &message)
        .unwrap();

    let decrypted = priv_key.decrypt(Pkcs1v15Encrypt, &encrypted).unwrap();

    assert_eq!(message, decrypted);
}

#[test]
fn test_encrypt_decrypt_shared_secret() {
    use rsa::pkcs8::EncodePublicKey;
    use rsa::{rand_core::OsRng, Pkcs1v15Encrypt, RsaPrivateKey};

    let mut rng = OsRng;

    let private_key = RsaPrivateKey::new(&mut rng, 1024).unwrap();
    let public_key = private_key.to_public_key();

    // Get DER to simulate Minecraft client loading the key
    let der = public_key.to_public_key_der().unwrap().as_bytes().to_vec();

    // Simulate Minecraft client encrypting with DER key
    let client_public_key = RsaPublicKey::from_public_key_der(&der).unwrap();

    let secret = b"1234567890abcdef"; // 16 bytes
    let encrypted = client_public_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, secret)
        .unwrap();

    // Simulate server decrypting
    let decrypted = private_key.decrypt(Pkcs1v15Encrypt, &encrypted).unwrap();

    assert_eq!(&decrypted, secret);
}

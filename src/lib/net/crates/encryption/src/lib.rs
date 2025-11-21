use crate::errors::NetEncryptionError;
use num_bigint::BigInt;
use rsa::pkcs8::EncodePublicKey;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use sha1::{Digest, Sha1};
use std::ops::Deref;
use std::sync::LazyLock;

pub mod errors;
pub mod read;
pub mod write;

/// The global EncryptionKeys instance to be used for encryption/decryption.
static ENCRYPTION_KEYS: LazyLock<EncryptionKeys> = LazyLock::new(EncryptionKeys::generate);

/// Struct to hold encryption keys.
///
/// Holds:
/// - A RSA private key
/// - The public key associated with said private key
/// - The public key encoded in DER format as specified by the Minecraft protocol
pub struct EncryptionKeys {
    #[allow(unused)] // Public key currently is only used to create DER format
    public_key: RsaPublicKey,
    private_key: RsaPrivateKey,
    der_format: Vec<u8>,
}

impl EncryptionKeys {
    /// Generates a 1024-bit RSA key pair to be used with the network protocol.
    ///
    /// # Returns
    /// - `Self`: A new EncryptionKeys instance with a random RSA key pair.
    pub fn generate() -> Self {
        let private_key =
            RsaPrivateKey::new(&mut rand::rng(), 1024).expect("RsaPrivateKey failed to generate");
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
        Ok(self
            .private_key
            .decrypt(Pkcs1v15Encrypt, data)
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

/// Minecraft's custom implementation of a SHA1 hex digest.
///
/// # Parameters
/// - `server_id`: The server id sent to the player in an EncryptionRequest packet.
/// - `shared_secret`: The shared secret returned by the player in an EncryptionResponse packet.
///
/// # Returns
/// - `String`: The resulting hex representation of the SHA1 digest
pub fn minecraft_hex_digest(server_id: &str, shared_secret: &[u8]) -> String {
    let mut hasher = Sha1::new();
    sha1::digest::Update::update(&mut hasher, server_id.as_bytes());
    sha1::digest::Update::update(&mut hasher, shared_secret);
    sha1::digest::Update::update(&mut hasher, &get_encryption_keys().clone_der());
    let digest = hasher.finalize();

    // Minecraft requires this as part of their special hexdigest function
    let bigint = BigInt::from_signed_bytes_be(&digest);

    bigint.to_str_radix(16)
}

#[cfg(test)]
mod tests {
    use num_bigint::BigInt;
    use sha1::{Digest, Sha1};

    /// Minecraft's custom implementation of a SHA1 hex digest.
    ///
    /// # Note
    /// This function is only to be used in tests. The only different between this function and `minecraft_hex_digest`
    /// is that this function does **NOT** hash the encryption key.
    ///
    /// # Parameters
    /// - `server_id`: The server id sent to the player in an EncryptionRequest packet.
    /// - `shared_secret`: The shared secret returned by the player in an EncryptionResponse packet.
    ///
    /// # Returns
    /// - `String`: The resulting hex representation of the SHA1 digest
    #[allow(unused)]
    fn minecraft_hex_digest_test(server_id: &str, shared_secret: &[u8]) -> String {
        let mut hasher = Sha1::new();
        sha1::digest::Update::update(&mut hasher, server_id.as_bytes());
        sha1::digest::Update::update(&mut hasher, shared_secret);
        let digest = hasher.finalize();

        // Minecraft requires this as part of their special hexdigest function
        let bigint = BigInt::from_signed_bytes_be(&digest);

        bigint.to_str_radix(16)
    }

    #[test]
    fn test_hex_digest() {
        assert_eq!(
            "-7c9d5b0044c130109a5d7b5fb5c317c02b4e28c1",
            minecraft_hex_digest_test("", b"jeb_")
        );
        assert_eq!(
            "4ed1f46bbe04bc756bcb17c0c7ce3e4632f06a48",
            minecraft_hex_digest_test("", b"Notch")
        );
        assert_eq!(
            "88e16a1019277b15d58faf0541e11910eb756f6",
            minecraft_hex_digest_test("", b"simon")
        );
    }
}

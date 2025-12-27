use aes::cipher::generic_array::GenericArray;
use aes::cipher::{BlockDecryptMut, KeyIvInit};
use aes::Aes128;
use cfb8::Decryptor;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, ReadBuf};

/// A wrapper around a reader that decrypts incoming bytes using AES/CFB8, if configured.
pub struct EncryptedReader<Reader> {
    reader: Reader,
    cipher: Option<Decryptor<Aes128>>,
}

impl<Reader> EncryptedReader<Reader> {
    /// Sets the internal AES cipher to use the specified key.
    ///
    /// # Parameters
    /// - `key`: The key to use for the AES cipher.
    pub fn update_cipher(&mut self, key: &[u8]) {
        self.cipher = Some(Decryptor::new_from_slices(key, key).unwrap());
    }
}

impl<Reader> From<Reader> for EncryptedReader<Reader> {
    fn from(reader: Reader) -> EncryptedReader<Reader> {
        Self {
            reader,
            cipher: None,
        }
    }
}

impl<Reader: AsyncRead + Unpin> AsyncRead for EncryptedReader<Reader> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let before = buf.filled().len();
        let poll = Pin::new(&mut self.reader).poll_read(cx, buf);

        if let Poll::Ready(Ok(())) = poll {
            // If cipher is not None, use it to decrypt incoming bytes
            if let Some(cipher) = self.cipher.as_mut() {
                for b in buf.filled_mut()[before..].chunks_mut(1) {
                    let block = GenericArray::from_mut_slice(b);
                    cipher.decrypt_block_mut(block);
                }
            }
        }

        poll
    }
}

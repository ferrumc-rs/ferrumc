use std::pin::Pin;
use std::task::{Context, Poll};
use aes::Aes128;
use aes::cipher::BlockDecryptMut;
use aes::cipher::generic_array::GenericArray;
use cfb8::Decryptor;
use tokio::io::{AsyncRead, ReadBuf};

pub struct EncryptedReader<Reader> {
    reader: Reader,
    cipher: Option<Decryptor<Aes128>>,
}

impl<Reader> EncryptedReader<Reader> {
    pub fn update_cipher(&mut self, cipher: Decryptor<Aes128>) {
        self.cipher = Some(cipher);
    }
}

impl<Reader> From<Reader> for EncryptedReader<Reader> {
    fn from(reader: Reader) -> EncryptedReader<Reader> {
        Self { reader, cipher: None }
    }
}

impl<Reader: AsyncRead + Unpin> AsyncRead for EncryptedReader<Reader> {
    fn poll_read(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<std::io::Result<()>> {
        let before = buf.filled().len();
        let poll = Pin::new(&mut self.reader).poll_read(cx, buf);

        if let Poll::Ready(Ok(())) = poll {
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
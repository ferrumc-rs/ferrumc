use aes::cipher::generic_array::GenericArray;
use aes::cipher::BlockEncryptMut;
use aes::Aes128;
use cfb8::Encryptor;
use std::io::Error;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::AsyncWrite;

pub struct EncryptedWriter<Writer> {
    writer: Writer,
    cipher: Option<Encryptor<Aes128>>,
}

impl<Writer> EncryptedWriter<Writer> {
    pub fn update_cipher(&mut self, cipher: Encryptor<Aes128>) {
        self.cipher = Some(cipher);
    }
}

impl<Writer> From<Writer> for EncryptedWriter<Writer> {
    fn from(writer: Writer) -> Self {
        Self { writer, cipher: None }
    }
}

impl<Writer: AsyncWrite + Unpin> AsyncWrite for EncryptedWriter<Writer> {
    fn poll_write(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<Result<usize, Error>> {
        let mut buf = buf.to_vec();

        if let Some(enc) = self.cipher.as_mut() {
            for b in buf.chunks_mut(1) {
                let block = GenericArray::from_mut_slice(b);
                enc.encrypt_block_mut(block);
            }
        }

        Pin::new(&mut self.writer).poll_write(cx, &buf)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        Pin::new(&mut self.writer).poll_flush(cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        Pin::new(&mut self.writer).poll_shutdown(cx)
    }
}
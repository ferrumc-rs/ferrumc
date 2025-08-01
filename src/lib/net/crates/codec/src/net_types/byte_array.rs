use crate::encode::errors::NetEncodeError;
use crate::encode::{NetEncode, NetEncodeOpts};
use crate::net_types::var_int::VarInt;
use std::io::Write;
use tokio::io::AsyncWrite;
use tokio::io::AsyncWriteExt;

/// A wrapper around a byte array that can be encoded with a length prefix.
/// This is faster than a LengthPrefixedVec for raw byte data, as it avoids encoding each byte individually.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ByteArray(pub Vec<u8>);

impl ByteArray {
    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }
}

impl NetEncode for ByteArray {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        VarInt::new(self.0.len() as i32).encode(writer, opts)?;
        // Since it's just a load of plain bytes that don't need any special encoding, we can just
        // hurl all of them at the writer and call it a day.
        writer.write_all(&self.0)?;
        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        VarInt::new(self.0.len() as i32)
            .encode_async(writer, opts)
            .await?;
        writer.write_all(&self.0).await?;
        Ok(())
    }
}

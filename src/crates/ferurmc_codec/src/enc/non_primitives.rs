use tokio::io::{AsyncWrite, AsyncWriteExt};

use crate::enc::NetEncode;
use crate::error::Result;
use crate::network_types::varint::VarInt;

impl NetEncode for String {
    async fn net_encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: AsyncWrite + Unpin,
    {
        self.as_str().net_encode(writer).await?;
        Ok(())
    }
}

impl<'a> NetEncode for &'a str {
    async fn net_encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: AsyncWrite + Unpin,
    {
        let len = VarInt::new(self.len() as i32);
        len.net_encode(writer).await?;
        writer.write_all(self.as_bytes()).await?;

        Ok(())
    }
}
// TODO: Use simd for primitive types to swap endianness way faster
impl<E: NetEncode> NetEncode for Vec<E> {
    /// Caution: This function does not encode the size of the vector.
    /// The size of the vector should be encoded before calling this function.
    async fn net_encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: AsyncWrite + Unpin,
    {
        // Length is handled by the macro or the person calling this function
        for v in self {
            v.net_encode(writer).await?;
        }

        Ok(())
    }
}
impl<O: NetEncode> NetEncode for Option<O> {
    async fn net_encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: AsyncWrite + Unpin,
    {
        match self {
            Some(v) => v.net_encode(writer).await,
            None => Ok(()),
        }
    }
}
impl<'a, E: NetEncode> NetEncode for &'a [E] {
    async fn net_encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: AsyncWrite + Unpin,
    {
        //! <WARNING> This function does not encode the size of the slice.
        for v in self.iter() {
            v.net_encode(writer).await?;
        }

        Ok(())
    }
}
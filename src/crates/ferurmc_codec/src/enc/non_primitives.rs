use crate::enc::Encode;
use crate::network_types::varint::VarInt;
use tokio::io::{AsyncWrite, AsyncWriteExt};
use crate::error::Result;

impl Encode for String {

    async fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: AsyncWrite + Unpin,
    {
        self.as_str().encode(writer).await?;
        Ok(())
    }
}

impl<'a> Encode for &'a str {

    async fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: AsyncWrite + Unpin,
    {
        let len = VarInt::new(self.len() as i32);
        len.encode(writer).await?;
        writer
            .write_all(self.as_bytes())
            .await?;

        Ok(())
    }
}
// TODO: Use simd for primitive types to swap endianness way faster
impl<E: Encode> Encode for Vec<E> {


    /// Caution: This function does not encode the size of the vector.
    /// The size of the vector should be encoded before calling this function.
    async fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: AsyncWrite + Unpin,
    {
        // Length is handled by the macro or the person calling this function
        for v in self {
            v.encode(writer).await?;
        }

        Ok(())
    }
}
impl<O: Encode> Encode for Option<O> {

    async fn encode<W>(&self, writer: &mut W) -> Result<()>
    where
        W: AsyncWrite + Unpin,
    {
        match self {
            Some(v) => v.encode(writer).await,
            None => Ok(()),
        }
    }
}
use crate::decode::{NetDecode, NetDecodeOpts, NetDecodeResult};
use crate::encode::{NetEncode, NetEncodeOpts, NetEncodeResult};
use crate::net_types::var_int::VarInt;
use std::io::{Read, Write};
use std::ops::{Deref, DerefMut};
use tokio::io::AsyncWrite;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LengthPrefixedVec<T> {
    pub data: Vec<T>,
}

impl<T> Default for LengthPrefixedVec<T> {
    fn default() -> Self {
        Self { data: Vec::new() }
    }
}

impl<T> LengthPrefixedVec<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self { data }
    }
}

impl<T> Deref for LengthPrefixedVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for LengthPrefixedVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T> NetEncode for LengthPrefixedVec<T>
where
    T: NetEncode,
{
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> NetEncodeResult<()> {
        VarInt::from(self.len()).encode(writer, opts)?;

        for item in &self.data {
            item.encode(writer, opts)?;
        }

        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> NetEncodeResult<()> {
        VarInt::from(self.len()).encode_async(writer, opts).await?;

        for item in &self.data {
            item.encode_async(writer, opts).await?;
        }

        Ok(())
    }
}

impl<T> NetDecode for LengthPrefixedVec<T>
where
    T: NetDecode,
{
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> NetDecodeResult<Self> {
        let length = VarInt::decode(reader, opts)?;

        let mut data = Vec::new();
        for _ in 0..length.val {
            data.push(T::decode(reader, opts)?);
        }

        Ok(Self { data })
    }
}

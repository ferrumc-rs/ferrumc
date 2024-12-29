use crate::decode::{NetDecode, NetDecodeOpts, NetDecodeResult};
use crate::encode::{NetEncode, NetEncodeOpts, NetEncodeResult};
use crate::net_types::var_int::VarInt;
use std::io::{Read, Write};
use tokio::io::AsyncWrite;

#[derive(Debug, Clone)]
pub struct LengthPrefixedVec<T> {
    pub length: VarInt,
    pub data: Vec<T>,
}

impl<T> LengthPrefixedVec<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self {
            length: VarInt::new(data.len() as i32),
            data,
        }
    }

    pub fn push(&mut self, data: T) {
        self.data.push(data);
        self.length = VarInt::new(self.length.val + 1);
    }
}

impl<T> NetEncode for LengthPrefixedVec<T>
where
    T: NetEncode,
{
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> NetEncodeResult<()> {
        self.length.encode(writer, opts)?;

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
        self.length.encode_async(writer, opts).await?;

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

        Ok(Self { length, data })
    }
}

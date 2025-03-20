use crate::decode::{NetDecode, NetDecodeOpts, NetDecodeResult};
use crate::encode::{NetEncode, NetEncodeOpts, NetEncodeResult};
use crate::net_types::var_int::VarInt;
use std::io::{Read, Write};

#[derive(Debug)]
pub struct LengthPrefixedVec<T> {
    pub length: VarInt,
    pub data: Vec<T>,
}

impl<T> Default for LengthPrefixedVec<T> {
    fn default() -> Self {
        Self {
            length: VarInt::new(0),
            data: Vec::new(),
        }
    }
}

impl<T> LengthPrefixedVec<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self {
            length: VarInt::new(data.len() as i32),
            data,
        }
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

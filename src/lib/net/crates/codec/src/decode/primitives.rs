use std::io::Read;
use crate::decode::{NetDecode, NetDecodeOpts, NetDecodeResult};
use crate::net_types::var_int::VarInt;

macro_rules! impl_for_primitives {
    ($($primitive_type:ty | $alt:ty),*) => {
        $(
            impl NetDecode for $primitive_type {
                fn decode<R: Read>(reader: &mut R, _: &NetDecodeOpts) -> NetDecodeResult<Self> {
                    let mut buf = [0; std::mem::size_of::<Self>()];
                    reader.read_exact(&mut buf)?;
                    Ok(Self::from_be_bytes(buf))
                }
            }

            impl NetDecode for $alt {
                fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> NetDecodeResult<Self> {
                    // Basically use the decode method of the primitive type,
                    // and then convert it to the alternative type.
                    <$primitive_type as NetDecode>::decode(reader, opts)
                    .map(|x| x as Self)
                }
            }
        )*
    };
}

impl_for_primitives!(
    u8 | i8,
    u16 | i16,
    u32 | i32,
    u64 | i64,
    u128 | i128,
    f32 | f64
);

impl NetDecode for bool {
    fn decode<R: Read>(reader: &mut R, _: &NetDecodeOpts) -> NetDecodeResult<Self> {
        Ok(<u8 as NetDecode>::decode(reader, &NetDecodeOpts::None)? != 0)
    }
}

impl NetDecode for String {
    fn decode<R: Read>(reader: &mut R, _: &NetDecodeOpts) -> NetDecodeResult<Self> {
        let len = <VarInt as NetDecode>::decode(reader, &NetDecodeOpts::None)?.val as usize;
        let mut buf = vec![0; len];
        reader.read_exact(&mut buf)?;
        Ok(String::from_utf8(buf)?)
    }
}
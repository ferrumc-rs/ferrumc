use crate::decode::{NetDecode, NetDecodeOpts, NetDecodeResult};
use crate::net_types::var_int::VarInt;
use std::io::Read;

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

impl<T> NetDecode for Vec<T>
where
    T: NetDecode,
{
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> NetDecodeResult<Self> {
        if matches!(opts, NetDecodeOpts::IsSizePrefixed) {
            let len = <VarInt as NetDecode>::decode(reader, opts)?.val as usize;
            let mut vec = Vec::with_capacity(len);
            for _ in 0..len {
                vec.push(T::decode(reader, opts)?);
            }
            return Ok(vec);
        }

        // read to end
        let mut data = Vec::new();
        R::read_to_end(reader, &mut data)?;

        let mut cursor = std::io::Cursor::new(data);

        let mut vec = Vec::new();
        while cursor.position() < cursor.get_ref().len() as u64 {
            vec.push(T::decode(&mut cursor, opts)?);
        }

        Ok(vec)
    }
}

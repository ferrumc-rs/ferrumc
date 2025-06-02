use crate::decode::{NetDecode, NetDecodeOpts, NetDecodeResult};
use crate::net_types::var_int::VarInt;
use std::collections::HashMap;
use std::hash::Hash;
use std::io::Read;
use tokio::io::AsyncRead;
use tokio::io::AsyncReadExt;

macro_rules! impl_for_primitives {
    ($($primitive_type:ty $(| $alt:ty)?),*) => {
        $(
            impl NetDecode for $primitive_type {
                fn decode<R: Read>(reader: &mut R, _: &NetDecodeOpts) -> NetDecodeResult<Self> {
                    let mut buf = [0; std::mem::size_of::<Self>()];
                    reader.read_exact(&mut buf)?;
                    Ok(Self::from_be_bytes(buf))
                }
                async fn decode_async<R: AsyncRead + Unpin>(reader: &mut R, _: &NetDecodeOpts) -> NetDecodeResult<Self> {
                    let mut buf = [0; std::mem::size_of::<Self>()];
                    reader.read_exact(&mut buf).await?;
                    Ok(Self::from_be_bytes(buf))
                }
            }

            $(
                impl NetDecode for $alt {
                    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> NetDecodeResult<Self> {
                        // Basically use the decode method of the primitive type,
                        // and then convert it to the alternative type.
                        <$primitive_type as NetDecode>::decode(reader, opts)
                        .map(|x| x as Self)
                    }
                    async fn decode_async<R: AsyncRead + Unpin>(reader: &mut R, opts: &NetDecodeOpts) -> NetDecodeResult<Self> {
                        // Basically use the decode method of the primitive type,
                        // and then convert it to the alternative type.
                        <$primitive_type as NetDecode>::decode_async(reader, opts)
                        .await
                        .map(|x| x as Self)
                    }
                }
            )?
        )*
    };
}

impl_for_primitives!(
    u8 | i8,
    u16 | i16,
    u32 | i32,
    u64 | i64,
    u128 | i128,
    usize | isize,
    f32,
    f64
);

impl NetDecode for bool {
    fn decode<R: Read>(reader: &mut R, _: &NetDecodeOpts) -> NetDecodeResult<Self> {
        Ok(<u8 as NetDecode>::decode(reader, &NetDecodeOpts::None)? != 0)
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        _: &NetDecodeOpts,
    ) -> NetDecodeResult<Self> {
        Ok(<u8 as NetDecode>::decode_async(reader, &NetDecodeOpts::None).await? != 0)
    }
}

impl NetDecode for String {
    fn decode<R: Read>(reader: &mut R, _: &NetDecodeOpts) -> NetDecodeResult<Self> {
        let len = <VarInt as NetDecode>::decode(reader, &NetDecodeOpts::None)?.0 as usize;
        let mut buf = vec![0; len];
        reader.read_exact(&mut buf)?;
        Ok(String::from_utf8(buf)?)
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        _: &NetDecodeOpts,
    ) -> NetDecodeResult<Self> {
        let len = <VarInt as NetDecode>::decode_async(reader, &NetDecodeOpts::None)
            .await?
            .0 as usize;
        let mut buf = vec![0; len];
        reader.read_exact(&mut buf).await?;
        Ok(String::from_utf8(buf)?)
    }
}

impl<T> NetDecode for Vec<T>
where
    T: NetDecode,
{
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> NetDecodeResult<Self> {
        if matches!(opts, NetDecodeOpts::IsSizePrefixed) {
            let len = <VarInt as NetDecode>::decode(reader, opts)?.0 as usize;
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

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> NetDecodeResult<Self> {
        if matches!(opts, NetDecodeOpts::IsSizePrefixed) {
            let len = <VarInt as NetDecode>::decode_async(reader, opts).await?.0 as usize;
            let mut vec = Vec::with_capacity(len);
            for _ in 0..len {
                vec.push(T::decode_async(reader, opts).await?);
            }
            return Ok(vec);
        }

        // read to end
        let mut data = Vec::new();
        R::read_to_end(reader, &mut data).await?;

        let mut cursor = std::io::Cursor::new(data);

        let mut vec = Vec::new();
        while cursor.position() < cursor.get_ref().len() as u64 {
            vec.push(T::decode_async(&mut cursor, opts).await?);
        }

        Ok(vec)
    }
}

/// This isn't actually a type in the Minecraft Protocol. This is just for saving data/ or for general use.
/// It was created for saving/reading chunks!
impl<K, V> NetDecode for HashMap<K, V>
where
    K: NetDecode + Eq + Hash,
    V: NetDecode,
{
    fn decode<R: Read>(reader: &mut R, opts: &NetDecodeOpts) -> NetDecodeResult<Self> {
        let len = <VarInt as NetDecode>::decode(reader, opts)?.0 as usize;
        let mut map = HashMap::with_capacity(len);
        for _ in 0..len {
            let key = K::decode(reader, opts)?;
            let value = V::decode(reader, opts)?;
            map.insert(key, value);
        }
        Ok(map)
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        opts: &NetDecodeOpts,
    ) -> NetDecodeResult<Self> {
        let len = <VarInt as NetDecode>::decode_async(reader, opts).await?.0 as usize;
        let mut map = HashMap::with_capacity(len);
        for _ in 0..len {
            let key = K::decode_async(reader, opts).await?;
            let value = V::decode_async(reader, opts).await?;
            map.insert(key, value);
        }
        Ok(map)
    }
}

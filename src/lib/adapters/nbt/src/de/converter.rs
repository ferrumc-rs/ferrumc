use crate::de::borrow::{NbtTape, NbtTapeElement};
use crate::{NBTError, Result};

pub trait FromNbt<'a>: Sized {
    fn from_nbt(tapes: &NbtTape<'a>, element: &NbtTapeElement<'a>) -> Result<Self>;
}

mod primitives {
    use super::*;
    use crate::de::borrow::NbtDeserializable;

    macro_rules! impl_for_primitives {
        ($($ty:ty) | *, $variant:ident) => {
            $(
            impl FromNbt<'_> for $ty {
                fn from_nbt(_tapes: &NbtTape, element: &NbtTapeElement) -> Result<Self> {
                    match element {
                        NbtTapeElement::$variant(val) => Ok(*val as $ty),
                        _ => Err(NBTError::TypeMismatch { expected: stringify!($variant), found: element.nbt_type() }),
                    }
                }
            })*
        };
    }

    impl_for_primitives!(i8 | u8, Byte);
    impl_for_primitives!(i16 | u16, Short);
    impl_for_primitives!(i32 | u32, Int);
    impl_for_primitives!(i64 | u64, Long);
    impl_for_primitives!(f32, Float);
    impl_for_primitives!(f64, Double);

    impl FromNbt<'_> for bool {
        fn from_nbt(_tapes: &NbtTape, element: &NbtTapeElement) -> Result<Self> {
            match element {
                NbtTapeElement::Byte(val) => Ok(*val != 0),
                _ => Err(NBTError::TypeMismatch {
                    expected: "Byte",
                    found: element.nbt_type(),
                }),
            }
        }
    }

    impl FromNbt<'_> for String {
        fn from_nbt(_tapes: &NbtTape, element: &NbtTapeElement) -> Result<Self> {
            match element {
                NbtTapeElement::String(val) => Ok(val.to_string()),
                _ => Err(NBTError::TypeMismatch {
                    expected: "String",
                    found: element.nbt_type(),
                }),
            }
        }
    }

    impl<'a> FromNbt<'a> for &'a str {
        fn from_nbt(_tapes: &NbtTape<'a>, element: &NbtTapeElement<'a>) -> Result<Self> {
            match element {
                NbtTapeElement::String(val) => Ok(val),
                _ => Err(NBTError::TypeMismatch {
                    expected: "String",
                    found: element.nbt_type(),
                }),
            }
        }
    }

    impl<'a, T: FromNbt<'a>> FromNbt<'a> for Vec<T> {
        fn from_nbt(tapes: &NbtTape<'a>, element: &NbtTapeElement<'a>) -> Result<Self> {
            match tapes.unpack_list::<T>(element) {
                Some(vec) => Ok(vec),
                None => Err(NBTError::TypeMismatch {
                    expected: "List",
                    found: element.nbt_type(),
                }),
            }
        }
    }

    impl<'a, T: NbtDeserializable<'a>> FromNbt<'a> for &'a [T] {
        fn from_nbt(tapes: &NbtTape<'a>, element: &NbtTapeElement<'a>) -> Result<Self> {
            match tapes.unpack_list_sliced::<T>(element) {
                Some(slice) => Ok(slice),
                None => Err(NBTError::TypeMismatch {
                    expected: "List Slice (T != array type)",
                    found: element.nbt_type(),
                }),
            }
        }
    }

    // optional
    impl<'a, T: FromNbt<'a>> FromNbt<'a> for Option<T> {
        fn from_nbt(tapes: &NbtTape<'a>, element: &NbtTapeElement<'a>) -> Result<Self> {
            // handle optionals yourself lol (jk they're handled by the derive macro :p)
            Ok(Some(T::from_nbt(tapes, element)?))
        }
    }
}

pub mod readers {
    pub mod async_reader {
        use std::io;
        use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
        use crate::de::borrow::NbtTag;

        enum Frame {
            Compound,
            List {
                tag_type: NbtTag,
                remaining: i32,
            },
        }

        pub async fn read_to_buf<R, W>(reader: &mut R, writer: &mut W) -> Result<(), io::Error>
        where
            R: AsyncRead + Unpin,
            W: AsyncWrite + Unpin,
        {
            let tag = read_tag(reader, writer).await?;

            if tag == NbtTag::End {
                return Ok(());
            }

            read_nbt_string(reader, writer).await?;
            read_nbt_content_async(&tag, reader, writer).await
        }

        pub async fn read_nbt_content_async<R, W>(tag: &NbtTag, reader: &mut R, writer: &mut W) -> Result<(), io::Error>
        where
            R: AsyncRead + Unpin,
            W: AsyncWrite + Unpin,
        {
            let mut current_tag = tag.clone();
            let mut stack: Vec<Frame> = Vec::with_capacity(8);

            loop {
                match current_tag {
                    NbtTag::End => {}
                    NbtTag::Compound => {
                        stack.push(Frame::Compound);
                    }
                    NbtTag::List => {
                        let tag_type = read_tag(reader, writer).await?;
                        let length = read_i32(reader, writer).await?;
                        if length > 0 {
                            stack.push(Frame::List {
                                tag_type,
                                remaining: length,
                            });
                        }
                    }
                    _ => {
                        read_tag_content_simple(current_tag, reader, writer).await?;
                    }
                }

                loop {
                    match stack.last_mut() {
                        None => {
                            return Ok(());
                        }
                        Some(Frame::Compound) => {
                            current_tag = read_tag(reader, writer).await?;

                            if current_tag != NbtTag::End {
                                read_nbt_string(reader, writer).await?;
                            } else {
                                stack.pop();
                                continue;
                            }
                            break;
                        }
                        Some(Frame::List { tag_type, remaining }) => {
                            if *remaining > 0 {
                                current_tag = tag_type.clone();
                                *remaining -= 1;
                                break;
                            } else {
                                stack.pop();
                                continue;
                            }
                        }
                    }
                }
            }
        }

        async fn read_tag_content_simple<R, W>(tag: NbtTag, reader: &mut R, writer: &mut W) -> Result<(), io::Error>
        where
            R: AsyncRead + Unpin,
            W: AsyncWrite + Unpin,
        {
            match tag {
                NbtTag::Byte => copy_bytes(reader, writer, 1).await,
                NbtTag::Short => copy_bytes(reader, writer, 2).await,
                NbtTag::Int => copy_bytes(reader, writer, 4).await,
                NbtTag::Long => copy_bytes(reader, writer, 8).await,
                NbtTag::Float => copy_bytes(reader, writer, 4).await,
                NbtTag::Double => copy_bytes(reader, writer, 8).await,
                NbtTag::ByteArray => {
                    let len = read_i32(reader, writer).await?;
                    copy_bytes(reader, writer, len as usize).await
                }
                NbtTag::IntArray => {
                    let len = read_i32(reader, writer).await?;
                    copy_bytes(reader, writer, len as usize * 4).await
                }
                NbtTag::LongArray => {
                    let len = read_i32(reader, writer).await?;
                    copy_bytes(reader, writer, len as usize * 8).await
                }
                NbtTag::String => read_nbt_string(reader, writer).await,
                _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Unexpected complex tag")),
            }
        }

        async fn read_nbt_string<R, W>(reader: &mut R, writer: &mut W) -> Result<(), io::Error>
        where
            R: AsyncRead + Unpin,
            W: AsyncWrite + Unpin,
        {
            let len = read_u16(reader, writer).await?;
            copy_bytes(reader, writer, len as usize).await
        }

        async fn read_tag<R: AsyncRead + Unpin, W: AsyncWrite + Unpin>(reader: &mut R, writer: &mut W)
                                                                       -> Result<NbtTag, io::Error> {
            let tag = reader.read_u8().await?;
            writer.write_u8(tag).await?;

            Ok(NbtTag::from(tag)) //todo: use try from
        }

        async fn read_i32<R, W>(reader: &mut R, writer: &mut W) -> Result<i32, io::Error>
        where
            R: AsyncRead + Unpin,
            W: AsyncWrite + Unpin,
        {
            let val = reader.read_i32().await?;
            writer.write_i32(val).await?;

            Ok(val)
        }

        async fn read_u16<R, W>(reader: &mut R, writer: &mut W) -> Result<u16, io::Error>
        where
            R: AsyncRead + Unpin,
            W: AsyncWrite + Unpin,
        {
            let val = reader.read_u16().await?;
            writer.write_u16(val).await?;

            Ok(val)
        }

        async fn copy_bytes<R, W>(
            reader: &mut R,
            writer: &mut W,
            count: usize,
        ) -> Result<(), io::Error>
        where
            R: AsyncRead + Unpin,
            W: AsyncWrite + Unpin,
        {
            if count <= 1024 {
                let mut buf = [0u8; 1024];
                let slice = &mut buf[..count];
                reader.read_exact(slice).await?;
                writer.write_all(slice).await?;
            } else {
                let mut limited = reader.take(count as u64);
                let copied = tokio::io::copy(&mut limited, writer).await?;

                if copied < count as u64 {
                    return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Incomplete NBT data"));
                }
            }

            Ok(())
        }
    }
    pub mod sync_reader {
        use std::io::{self, Read, Write};
        use crate::de::borrow::NbtTag;

        pub fn read_to_buf<R, W>(reader: &mut R, writer: &mut W) -> Result<(), io::Error>
        where
            R: Read,
            W: Write,
        {
            let tag = read_tag(reader, writer)?;

            if tag == NbtTag::End {
                return Ok(());
            }

            read_nbt_string(reader, writer)?;
            read_nbt_content_recursive(&tag, reader, writer)
        }

        pub fn read_nbt_content_recursive<R, W>(tag: &NbtTag, reader: &mut R, writer: &mut W) -> Result<(), io::Error>
        where
            R: Read,
            W: Write,
        {
            match tag {
                NbtTag::End => Ok(()),
                NbtTag::Compound => {
                    loop {
                        let inner_tag = read_tag(reader, writer)?;

                        if inner_tag == NbtTag::End {
                            break;
                        }

                        read_nbt_string(reader, writer)?;
                        read_nbt_content_recursive(&inner_tag, reader, writer)?;
                    }
                    Ok(())
                }
                NbtTag::List => {
                    let tag_type = read_tag(reader, writer)?;
                    let length = read_i32(reader, writer)?;

                    for _ in 0..length {
                        read_nbt_content_recursive(&tag_type, reader, writer)?;
                    }

                    Ok(())
                }
                _ => read_tag_content_simple(tag, reader, writer),
            }
        }

        fn read_tag_content_simple<R, W>(tag: &NbtTag, reader: &mut R, writer: &mut W) -> Result<(), io::Error>
        where
            R: Read,
            W: Write,
        {
            match tag {
                NbtTag::Byte => copy_bytes(reader, writer, 1),
                NbtTag::Short => copy_bytes(reader, writer, 2),
                NbtTag::Int => copy_bytes(reader, writer, 4),
                NbtTag::Long => copy_bytes(reader, writer, 8),
                NbtTag::Float => copy_bytes(reader, writer, 4),
                NbtTag::Double => copy_bytes(reader, writer, 8),
                NbtTag::ByteArray => {
                    let len = read_i32(reader, writer)?;
                    copy_bytes(reader, writer, len as usize)?;
                    Ok(())
                }
                NbtTag::IntArray => {
                    let len = read_i32(reader, writer)?;
                    copy_bytes(reader, writer, len as usize * 4)?;
                    Ok(())
                }
                NbtTag::LongArray => {
                    let len = read_i32(reader, writer)?;
                    copy_bytes(reader, writer, len as usize * 8)?;
                    Ok(())
                }
                NbtTag::String => read_nbt_string(reader, writer),
                _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Unexpected complex tag")),
            }
        }

        fn read_nbt_string<R, W>(reader: &mut R, writer: &mut W) -> Result<(), io::Error>
        where
            R: Read,
            W: Write,
        {
            let len = read_u16(reader, writer)?;
            copy_bytes(reader, writer, len as usize)
        }

        fn read_tag<R: Read, W: Write>(reader: &mut R, writer: &mut W) -> Result<NbtTag, io::Error> {
            let mut buf = [0u8; 1];
            reader.read_exact(&mut buf)?;
            writer.write_all(&buf)?;
            Ok(NbtTag::from(buf[0])) //todo: use try from
        }

        fn read_i32<R: Read, W: Write>(reader: &mut R, writer: &mut W) -> Result<i32, io::Error> {
            let mut buf = [0u8; 4];
            reader.read_exact(&mut buf)?;
            writer.write_all(&buf)?;
            Ok(i32::from_be_bytes(buf))
        }

        fn read_u16<R: Read, W: Write>(reader: &mut R, writer: &mut W) -> Result<u16, io::Error> {
            let mut buf = [0u8; 2];
            reader.read_exact(&mut buf)?;
            writer.write_all(&buf)?;
            Ok(u16::from_be_bytes(buf))
        }

        fn copy_bytes<R: Read, W: Write>(reader: &mut R, writer: &mut W, count: usize) -> Result<(), io::Error> {
            if (count == 0) {
                return Ok(());
            }

            if count <= 1024 {
                let mut buf = [0u8; 1024];
                let slice = &mut buf[..count];
                reader.read_exact(slice)?;
                writer.write_all(slice)?;
            } else {
                let mut limited = reader.take(count as u64);
                let copied = io::copy(&mut limited, writer)?;

                if copied < count as u64 {
                    return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Incomplete NBT data"));
                }
            }

            Ok(())
        }
    }
}

mod maps {
    use crate::{FromNbt, NBTError, NbtTape, NbtTapeElement, Result};
    use std::collections::{BTreeMap, HashMap};

    impl<'a, V: FromNbt<'a>> FromNbt<'a> for HashMap<String, V> {
        fn from_nbt(tapes: &NbtTape<'a>, element: &NbtTapeElement<'a>) -> Result<Self> {
            let compound = element.as_compound().ok_or(NBTError::TypeMismatch {
                expected: "Compound (from HashMap<String, V>)",
                found: element.nbt_type(),
            })?;
            // Compound: &Vec<(&str, NbtTapeElement)>, therefore we can just iterate over it and turn it into a hashmap.
            compound
                .iter()
                .map(|(key, val)| Ok((key.to_string(), V::from_nbt(tapes, val)?)))
                .collect()
        }
    }

    impl<'a, V: FromNbt<'a>> FromNbt<'a> for HashMap<&'a str, V> {
        fn from_nbt(tapes: &NbtTape<'a>, element: &NbtTapeElement<'a>) -> Result<Self> {
            let compound = element.as_compound().ok_or(NBTError::TypeMismatch {
                expected: "Compound (from HashMap<&str, V>)",
                found: element.nbt_type(),
            })?;
            // Compound: &Vec<(&str, NbtTapeElement)>, therefore we can just iterate over it and turn it into a hashmap.
            compound
                .iter()
                .map(|(key, val)| Ok((*key, V::from_nbt(tapes, val)?)))
                .collect()
        }
    }

    impl<'a, V: FromNbt<'a>> FromNbt<'a> for BTreeMap<&'a str, V> {
        fn from_nbt(tapes: &NbtTape<'a>, element: &NbtTapeElement<'a>) -> Result<Self> {
            let compound = element.as_compound().ok_or(NBTError::TypeMismatch {
                expected: "Compound (from BTreeMap<&str, V>)",
                found: element.nbt_type(),
            })?;
            // Compound: &Vec<(&str, NbtTapeElement)>, therefore we can just iterate over it and turn it into a hashmap.

            compound
                .iter()
                .map(|(key, val)| Ok((*key, V::from_nbt(tapes, val)?)))
                .collect()
        }
    }

    impl<'a, V: FromNbt<'a>> FromNbt<'a> for BTreeMap<String, V> {
        fn from_nbt(tapes: &NbtTape<'a>, element: &NbtTapeElement<'a>) -> Result<Self> {
            let compound = element.as_compound().ok_or(NBTError::TypeMismatch {
                expected: "Compound (from BTreeMap<String, V>)",
                found: element.nbt_type(),
            })?;
            // Compound: &Vec<(&str, NbtTapeElement)>, therefore we can just iterate over it and turn it into a hashmap.

            compound
                .iter()
                .map(|(key, val)| Ok((key.to_string(), V::from_nbt(tapes, val)?)))
                .collect()
        }
    }
}

#[cfg(test)]
mod test_map {
    use crate::{FromNbt, NBTSerializable, NBTSerializeOptions};
    use std::collections::HashMap;

    #[test]
    fn test_hashmap_both_ways() {
        let some_hashmap = maplit::hashmap! {
            "key1" => 1,
            "key2" => 2,
            "key3" => 3,
        };

        let data = {
            let mut buf = Vec::new();
            some_hashmap.serialize(&mut buf, &NBTSerializeOptions::WithHeader("root"));
            buf
        };

        let mut tapes = crate::de::borrow::NbtTape::new(&data);
        tapes.parse();
        let root = tapes
            .root
            .as_ref()
            .map(|(_, b)| b)
            .expect("failed to get root");
        let hashmap =
            HashMap::<&str, i32>::from_nbt(&tapes, root).expect("failed to deserialize root");

        assert_eq!(some_hashmap, hashmap);
    }

    #[test]
    fn test_btreemap_both_ways() {
        let some_btreemap = maplit::btreemap! {
            "key1" => 1,
            "key2" => 2,
            "key3" => 3,
        };

        let data = {
            let mut buf = Vec::new();
            some_btreemap.serialize(&mut buf, &NBTSerializeOptions::WithHeader("root"));
            buf
        };

        let mut tapes = crate::de::borrow::NbtTape::new(&data);
        tapes.parse();
        let root = tapes
            .root
            .as_ref()
            .map(|(_, b)| b)
            .expect("failed to get root");
        let btreemap = std::collections::BTreeMap::<&str, i32>::from_nbt(&tapes, root)
            .expect("failed to deserialize root");

        assert_eq!(some_btreemap, btreemap);
    }
}

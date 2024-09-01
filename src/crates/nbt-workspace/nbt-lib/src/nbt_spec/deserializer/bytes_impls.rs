use std::collections::HashMap;
use crate::NBTError;
use super::*;

impl NBTDeserializeBytes for i8 {
    #[inline]
    fn read_from_bytes(cursor: &mut Cursor<Vec<u8>>) -> NBTResult<Self> {
        cursor.read_i8()
    }
}

impl NBTDeserializeBytes for i16 {
    #[inline]
    fn read_from_bytes(cursor: &mut Cursor<Vec<u8>>) -> NBTResult<Self> {
        cursor.read_i16()
    }
}

impl NBTDeserializeBytes for i32 {
    #[inline]
    fn read_from_bytes(cursor: &mut Cursor<Vec<u8>>) -> NBTResult<Self> {
        cursor.read_i32()
    }
}

impl NBTDeserializeBytes for i64 {
    #[inline]
    fn read_from_bytes(cursor: &mut Cursor<Vec<u8>>) -> NBTResult<Self> {
        cursor.read_i64()
    }
}

impl NBTDeserializeBytes for f32 {
    #[inline]
    fn read_from_bytes(cursor: &mut Cursor<Vec<u8>>) -> NBTResult<Self> {
        cursor.read_f32()
    }
}

impl NBTDeserializeBytes for f64 {
    #[inline]
    fn read_from_bytes(cursor: &mut Cursor<Vec<u8>>) -> NBTResult<Self> {
        cursor.read_f64()
    }
}

impl NBTDeserializeBytes for String {
    #[inline]
    fn read_from_bytes(cursor: &mut Cursor<Vec<u8>>) -> NBTResult<Self> {
        cursor.read_nbt_string()
    }
}

impl<T: NBTDeserializeBytes> NBTDeserializeBytes for Vec<T> {
    #[inline]
    fn read_from_bytes(cursor: &mut Cursor<Vec<u8>>) -> NBTResult<Self> {
        let len = cursor.read_i32()?;
        let mut vec = Vec::with_capacity(len as usize);
        for _ in 0..len {
            vec.push(T::read_from_bytes(cursor)?);
        }
        Ok(vec)
    }
}

impl<V: NBTDeserializeBytes + NBTDeserialize> NBTDeserializeBytes for HashMap<String, V> {
    #[inline]
    fn read_from_bytes(cursor: &mut Cursor<Vec<u8>>) -> NBTResult<Self> {
        let Ok(nbt_tag) = nbt_tag_reader::read_tag(cursor) else {
            return Err(NBTError::DeserializeError("Failed to read compound tag".to_string()))
        };

        match nbt_tag {
            NBTTag::Compound(hashmap) => {
                Ok(hashmap
                    .into_iter()
                    .filter_map(|(key, tag)| {
                        let value = V::read_from(tag).ok()?;
                        Some((key, value))
                    })
                    .collect())
            }
            _ => Err(NBTError::InvalidType("Compound", nbt_tag.my_type())),
        }
    }
}
use std::collections::HashMap;
use std::io::Write;

pub use crate::nbt_spec::serializer::{NBTAnonymousType, NBTSerialize};
use crate::nbt_spec::serializer::tag_types::*;
use crate::NBTResult;

pub trait NBTFieldType {
    fn tag_type(&self) -> u8;
}

macro_rules! impl_nbt_serialize {
    ($type:ty, $tag_type:expr) => {
        impl NBTSerialize for $type {
            fn nbt_serialize<W: Write>(&self, writer: &mut W) -> NBTResult<()> {
                Ok(writer.write_all(&self.to_be_bytes())?)
            }
        }

        impl NBTFieldType for $type {
            fn tag_type(&self) -> u8 {
                $tag_type
            }
        }
        impl NBTAnonymousType for $type {
            fn tag_type() -> u8 {
                $tag_type
            }
        }
    };
}
macro_rules! impl_nbt_tag_type {
    ($type:ty, $tag_type:expr) => {
        impl NBTFieldType for $type {
            fn tag_type(&self) -> u8 {
                $tag_type
            }
        }
        impl NBTAnonymousType for $type {
            fn tag_type() -> u8 {
                $tag_type
            }
        }
    };
}

impl_nbt_serialize!(u8, TAG_BYTE);
impl_nbt_serialize!(i8, TAG_BYTE);
impl_nbt_serialize!(u16, TAG_SHORT);
impl_nbt_serialize!(i16, TAG_SHORT);
impl_nbt_serialize!(i32, TAG_INT);
impl_nbt_serialize!(i64, TAG_LONG);
impl_nbt_serialize!(f32, TAG_FLOAT);
impl_nbt_serialize!(f64, TAG_DOUBLE);

// impl NBTFieldType for bool { fn tag_type(&self) -> u8 { TAG_BYTE } }
impl_nbt_tag_type!(bool, TAG_BYTE);
impl NBTSerialize for bool {
    fn nbt_serialize<W: Write>(&self, writer: &mut W) -> NBTResult<()> {
        Ok(writer.write_all(&(*self as u8).to_be_bytes())?)
    }
}

// impl NBTFieldType for String { fn tag_type(&self) -> u8 { TAG_STRING } }
impl_nbt_tag_type!(String, TAG_STRING);
impl NBTSerialize for String {
    fn nbt_serialize<W: Write>(&self, writer: &mut W) -> NBTResult<()> {
        writer.write_all(&(self.len() as u16).to_be_bytes())?;
        Ok(writer.write_all(self.as_bytes())?)
    }
}

// impl<'a> NBTFieldType for &'a str { fn tag_type(&self) -> u8 { TAG_STRING } }
impl_nbt_tag_type!(&str, TAG_STRING);
impl<'a> NBTSerialize for &'a str {
    fn nbt_serialize<W: Write>(&self, writer: &mut W) -> NBTResult<()> {
        writer.write_all(&((*self).len() as u16).to_be_bytes())?;
        Ok(writer.write_all((*self).as_bytes())?)
    }
}

impl<T: NBTFieldType> NBTFieldType for Vec<T>
where
    T: NBTAnonymousType,
{
    fn tag_type(&self) -> u8 {
        // TAG_LIST
        match <T as NBTAnonymousType>::tag_type() {
            TAG_BYTE => TAG_BYTE_ARRAY,
            TAG_INT => TAG_INT_ARRAY,
            TAG_LONG => TAG_LONG_ARRAY,
            _ => TAG_LIST,
        }
    }
}

impl<T: NBTAnonymousType> NBTAnonymousType for Vec<T> {
    fn tag_type() -> u8 {
        // T::tag_type()
        match T::tag_type() {
            TAG_BYTE => TAG_BYTE_ARRAY,
            TAG_INT => TAG_INT_ARRAY,
            TAG_LONG => TAG_LONG_ARRAY,
            _ => TAG_LIST,
        }
    }
}

impl<T> NBTSerialize for Vec<T>
where
    T: NBTSerialize + NBTAnonymousType,
{
    fn nbt_serialize<W: Write>(&self, writer: &mut W) -> NBTResult<()> {
        let tag_type = <T as NBTAnonymousType>::tag_type();
        if tag_type == TAG_LIST {
            writer.write_all(&tag_type.to_be_bytes())?;
        }
        writer.write_all(&(self.len() as i32).to_be_bytes())?;
        for v in self {
            v.nbt_serialize(writer)?;
        }
        if self.len() == 0 {
            TAG_END.nbt_serialize(writer)?;
        }
        Ok(())
    }
}

impl<T> NBTFieldType for Option<T>
where
    T: NBTSerialize + NBTAnonymousType,
{
    fn tag_type(&self) -> u8 {
        <T as NBTAnonymousType>::tag_type()
    }
}

impl<T> NBTSerialize for Option<T>
where
    T: NBTSerialize + NBTAnonymousType,
{
    fn nbt_serialize<W: Write>(&self, writer: &mut W) -> NBTResult<()> {
        match self {
            Some(v) => v.nbt_serialize(writer),
            None => Ok(()),
        }
    }
}

impl<K, V> NBTFieldType for HashMap<K, V> {
    fn tag_type(&self) -> u8 {
        TAG_COMPOUND
    }
}

impl<K, V> NBTSerialize for HashMap<K, V>
where
    K: NBTSerialize,
    V: NBTSerialize,
{
    fn nbt_serialize<W: Write>(&self, writer: &mut W) -> NBTResult<()> {
        writer.write_all(&(self.len() as i32).to_be_bytes())?;
        for (k, v) in self {
            k.nbt_serialize(writer)?;
            v.nbt_serialize(writer)?;
        }
        Ok(())
    }
}

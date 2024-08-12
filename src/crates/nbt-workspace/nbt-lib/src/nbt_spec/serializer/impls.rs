use std::io::Write;

use crate::nbt_spec::serializer::NBTSerialize;
use crate::nbt_spec::serializer::tag_types::*;
use crate::NBTResult;

pub trait NBTTagIdentity {
    fn tag_type() -> u8;
}

macro_rules! impl_nbt_serialize {
    ($type:ty, $tag_type:expr) => {
        impl NBTSerialize for $type {
            fn serialize<W: Write>(&self, writer: &mut W) -> NBTResult<()> {
                Ok(writer.write_all(&self.to_be_bytes())?)
            }
        }

        impl NBTTagIdentity for $type {
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

impl NBTTagIdentity for bool { fn tag_type() -> u8 { TAG_BYTE } }
impl NBTSerialize for bool {
    fn serialize<W: Write>(&self, writer: &mut W) -> NBTResult<()> {
        Ok(writer.write_all(&(*self as u8).to_be_bytes())?)
    }
}

impl NBTTagIdentity for String { fn tag_type() -> u8 { TAG_STRING } }
impl NBTSerialize for String {
    fn serialize<W: Write>(&self, writer: &mut W) -> NBTResult<()> {
        writer.write_all(&(self.len() as u16).to_be_bytes())?;
        Ok(writer.write_all(self.as_bytes())?)
    }
}

impl<'a> NBTTagIdentity for &'a str { fn tag_type() -> u8 { TAG_STRING } }
impl<'a> NBTSerialize for &'a str {
    fn serialize<W: Write>(&self, writer: &mut W) -> NBTResult<()> {
        writer.write_all(&((*self).len() as u16).to_be_bytes())?;
        Ok(writer.write_all((*self).as_bytes())?)
    }
}


impl<T: NBTTagIdentity> NBTTagIdentity for Vec<T> {
    fn tag_type() -> u8 {
        TAG_LIST
    }
}

impl<T> NBTSerialize for Vec<T>
where
    T: NBTSerialize,
{
    fn serialize<W: Write>(&self, writer: &mut W) -> NBTResult<()> {
        let tag_type = T::tag_type();
        writer.write_all(&tag_type.to_be_bytes())?;
        writer.write_all(&(self.len() as i32).to_be_bytes())?;
        for v in self {
            v.serialize(writer)?;
        }
        if self.len() == 0 {
            TAG_END.serialize(writer)?;
        }
        Ok(())
    }
}

impl<T> NBTTagIdentity for Option<T>
where
    T: NBTSerialize,
{
    fn tag_type() -> u8 {
        T::tag_type()
    }
}

impl<T> NBTSerialize for Option<T>
where
    T: NBTSerialize,
{
    fn serialize<W: Write>(&self, writer: &mut W) -> NBTResult<()> {
        match self {
            Some(v) => v.serialize(writer),
            None => Ok(()),
        }
    }
}

impl<K, V> NBTTagIdentity for std::collections::HashMap<K, V> {
    fn tag_type() -> u8 {
        TAG_COMPOUND
    }
}
impl<K, V> NBTSerialize for std::collections::HashMap<K, V>
where
    K: NBTSerialize,
    V: NBTSerialize,
{
    fn serialize<W: Write>(&self, writer: &mut W) -> NBTResult<()> {
        writer.write_all(&(self.len() as i32).to_be_bytes())?;
        for (k, v) in self {
            k.serialize(writer)?;
            v.serialize(writer)?;
        }
        Ok(())
    }
}
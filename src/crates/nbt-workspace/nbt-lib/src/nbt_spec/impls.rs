use std::io::Write;
use crate::nbt_spec::serializer::NBTSerialize;

macro_rules! impl_nbt_serialize {
    ($type:ty) => {
        impl NBTSerialize for $type {
            fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
                writer.write_all(&self.to_be_bytes())
            }
        }
    };
}

impl_nbt_serialize!(u8);
impl_nbt_serialize!(i8);
impl_nbt_serialize!(i16);
impl_nbt_serialize!(i32);
impl_nbt_serialize!(i64);
impl_nbt_serialize!(f32);
impl_nbt_serialize!(f64);


impl NBTSerialize for bool {
    fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&(*self as u8).to_be_bytes())
    }
}

impl NBTSerialize for String {
    fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&(self.len() as u16).to_be_bytes())?;
        writer.write_all(self.as_bytes())
    }
}

impl<T> NBTSerialize for Vec<T>
where
    T: NBTSerialize,
{
    fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&(self.len() as i32).to_be_bytes())?;
        for v in self {
            v.serialize(writer)?;
        }
        Ok(())
    }
}

impl<T> NBTSerialize for Option<T>
where
    T: NBTSerialize,
{
    fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        match self {
            Some(v) => v.serialize(writer),
            None => Ok(()),
        }
    }
}

impl<K, V> NBTSerialize for std::collections::HashMap<K, V>
where
    K: NBTSerialize,
    V: NBTSerialize,
{
    fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&(self.len() as i32).to_be_bytes())?;
        for (k, v) in self {
            k.serialize(writer)?;
            v.serialize(writer)?;
        }
        Ok(())
    }
}
use std::io::{Write};
use std::collections::HashMap;
use crate::nbt_spec::serializer::NBTSerialize;
use crate::nbt_spec::tag_types;
use crate::nbt_spec::named_tag::NamedTag;
use crate::NBTResult;

#[derive(Debug)]
pub enum Tag {
    End,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String),
    List(Vec<Tag>),
    Compound(HashMap<String, NamedTag>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl Tag {
    pub fn get_type_id(&self) -> u8 {
        match self {
            Tag::End => tag_types::TAG_END,
            Tag::Byte(_) => tag_types::TAG_BYTE,
            Tag::Short(_) => tag_types::TAG_SHORT,
            Tag::Int(_) => tag_types::TAG_INT,
            Tag::Long(_) => tag_types::TAG_LONG,
            Tag::Float(_) => tag_types::TAG_FLOAT,
            Tag::Double(_) => tag_types::TAG_DOUBLE,
            Tag::ByteArray(_) => tag_types::TAG_BYTE_ARRAY,
            Tag::String(_) => tag_types::TAG_STRING,
            Tag::List(_) => tag_types::TAG_LIST,
            Tag::Compound(_) => tag_types::TAG_COMPOUND,
            Tag::IntArray(_) => tag_types::TAG_INT_ARRAY,
            Tag::LongArray(_) => tag_types::TAG_LONG_ARRAY,
        }
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> NBTResult<()> {
        match self {
            Tag::End => {},
            Tag::Byte(v) => v.serialize(writer)?,
            Tag::Short(v) => v.serialize(writer)?,
            Tag::Int(v) => v.serialize(writer)?,
            Tag::Long(v) => v.serialize(writer)?,
            Tag::Float(v) => v.serialize(writer)?,
            Tag::Double(v) => v.serialize(writer)?,
            Tag::ByteArray(arr) => arr.serialize(writer)?,
            Tag::String(s) => s.serialize(writer)?,
            Tag::List(list) => {
                let tag_id = list.first().map_or(tag_types::TAG_END, |tag| tag.get_type_id());
                writer.write_all(&[tag_id])?;
                writer.write_all(&(list.len() as i32).to_be_bytes())?;
                for tag in list {
                    tag.write(writer)?;
                }
            },
            Tag::Compound(map) => {
                for (_, named_tag) in map {
                    named_tag.serialize(writer)?;
                }
                writer.write_all(&[tag_types::TAG_END])?;
            },
            Tag::IntArray(arr) => arr.serialize(writer)?,
            Tag::LongArray(arr) => arr.serialize(writer)?,
        }
        Ok(())
    }
}

impl NBTSerialize for Tag {
    fn serialize<W: Write>(&self, writer: &mut W) -> NBTResult<()> {
        writer.write_all(&[self.get_type_id()])?;
        self.write(writer)?;
        Ok(())
    }
}
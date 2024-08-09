use std::collections::HashMap;
use std::io;
use tokio::io::{AsyncWrite, AsyncWriteExt};
use crate::nbt_spec::serializer::NBTSerialize;
use crate::nbt_spec::tag_types;

pub struct NamedTag<T> {
    pub name: String,
    pub value: T,
}

impl<T> NamedTag<T> {
    pub fn new(name: impl Into<String>, value: T) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }
}


#[derive(Debug, PartialEq)]
pub enum Tag {
    End,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
    List(Vec<Tag>),
    Compound(HashMap<String, Tag>),
    // We'll implement array types later
}

impl Tag {
    pub fn tag_id(&self) -> u8 {
        match self {
            Tag::End => tag_types::TAG_END,
            Tag::Byte(_) => tag_types::TAG_BYTE,
            Tag::Short(_) => tag_types::TAG_SHORT,
            Tag::Int(_) => tag_types::TAG_INT,
            Tag::Long(_) => tag_types::TAG_LONG,
            Tag::Float(_) => tag_types::TAG_FLOAT,
            Tag::Double(_) => tag_types::TAG_DOUBLE,
            Tag::String(_) => tag_types::TAG_STRING,
            Tag::List(_) => tag_types::TAG_LIST,
            Tag::Compound(_) => tag_types::TAG_COMPOUND,
        }
    }

    pub async fn serialize_without_type<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> io::Result<()> {
        match self {
            Tag::End => {},
            Tag::Byte(v) => writer.write_i8(*v).await?,
            Tag::Short(v) => writer.write_i16(*v).await?,
            Tag::Int(v) => writer.write_i32(*v).await?,
            Tag::Long(v) => writer.write_i64(*v).await?,
            Tag::Float(v) => writer.write_f32(*v).await?,
            Tag::Double(v) => writer.write_f64(*v).await?,
            Tag::String(v) => {
                writer.write_u16(v.len() as u16).await?;
                writer.write_all(v.as_bytes()).await?;
            },
            Tag::List(v) => v.nbt_serialize("", writer).await?,
            Tag::Compound(v) => v.nbt_serialize("", writer).await?,
        }
        Ok(())
    }
}
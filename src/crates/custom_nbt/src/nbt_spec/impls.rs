use std::collections::HashMap;
use std::io;
use tokio::io::{AsyncWrite, AsyncWriteExt};
use crate::nbt_spec::buffer::NBTBuffer;
use crate::nbt_spec::serializer::NBTSerialize;
use crate::nbt_spec::tag_types;
use crate::nbt_spec::tags::Tag;

impl NBTSerialize for String {
    async fn nbt_serialize<W: AsyncWrite + Unpin + Send>(&self, name: &str, writer: &mut W) -> io::Result<()> {
        let mut buffer = NBTBuffer::new();
        buffer.write_u8(tag_types::TAG_STRING);
        buffer.write_string(name);
        buffer.write_string(self);
        buffer.flush(writer).await
    }
}

impl NBTSerialize for Vec<Tag> {
    async fn nbt_serialize<W: AsyncWrite + Unpin + Send>(&self, name: &str, writer: &mut W) -> io::Result<()> {
        let mut buffer = NBTBuffer::new();
        buffer.write_u8(tag_types::TAG_LIST);
        buffer.write_string(name);

        // Write the type of the list elements
        if let Some(first) = self.first() {
            buffer.write_u8(first.tag_id());
        } else {
            buffer.write_u8(tag_types::TAG_END);
        }

        // Write the length of the list
        buffer.write_i32(self.len() as i32);
        buffer.flush(writer).await?;

        // Serialize each element
        for item in self {
            item.serialize_without_type(writer).await?;
        }

        Ok(())
    }
}

impl NBTSerialize for HashMap<String, Tag> {
    async fn nbt_serialize<W: AsyncWrite + Unpin + Send>(&self, name: &str, writer: &mut W) -> io::Result<()> {
        let mut buffer = NBTBuffer::new();
        buffer.write_u8(tag_types::TAG_COMPOUND);
        buffer.write_string(name);
        buffer.flush(writer).await?;

        // Serialize each key-value pair
        for (key, value) in self {
            value.nbt_serialize(key, writer).await?;
        }

        // Write the end tag
        writer.write_u8(tag_types::TAG_END).await?;

        Ok(())
    }
}

impl NBTSerialize for Tag {
    async fn nbt_serialize<W: AsyncWrite + Unpin + Send>(&self, name: &str, writer: &mut W) -> io::Result<()> {
        let mut buffer = NBTBuffer::new();
        buffer.write_u8(self.tag_id());
        buffer.write_string(name);
        buffer.flush(writer).await?;

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
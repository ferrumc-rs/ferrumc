use std::io::{Read, Write};

#[derive(Debug, PartialEq)]
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
    Compound(Vec<(String, Tag)>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

pub struct NBTSerializer;

impl NBTSerializer {
    pub fn serialize<W: Write>(tag: &Tag, writer: &mut W) -> std::io::Result<()> {
        // TODO: Implement serialization
        Ok(())
    }

    pub fn deserialize<R: Read>(reader: &mut R) -> std::io::Result<Tag> {
        // TODO: Implement deserialization
        Ok(Tag::End)
    }
}

#[derive(Debug)]
pub enum NBTError {
    IoError(std::io::Error),
    InvalidTagType(u8),
    // Add more error types as needed
}

impl From<std::io::Error> for NBTError {
    fn from(err: std::io::Error) -> Self {
        NBTError::IoError(err)
    }
}

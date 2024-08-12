use std::collections::HashMap;
use std::io::Cursor;
use crate::error::NBTError;
use crate::nbt_spec::deserializer::cursor_ext::CursorExt;
use crate::nbt_spec::deserializer::NBTDeserializeBytes;
use crate::NBTResult;

#[derive(Debug)]
pub enum NBTTag {
    End,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String),
    List(Vec<NBTTag>),
    Compound(HashMap<String, NBTTag>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl NBTTag {
    pub fn get(&mut self, key: &str) -> Option<NBTTag> {
        match self {
            NBTTag::Compound(map) => map.remove(key),
            _ => None,
        }
    }
}

pub fn read_tag(cursor: &mut Cursor<Vec<u8>>) -> NBTResult<NBTTag> {
    let mut compound_data: HashMap<String, NBTTag> = HashMap::new();

    while cursor.position() < cursor.get_ref().len() as u64 {
        let tag_type: u8 = cursor.read_i8()? as u8;
        if tag_type == 0 {
            break;
        }
        let name: String = cursor.read_nbt_string()?;

        let tag = read_tag_based_on_type(cursor, tag_type)
            .map_err(|e| NBTError::DeserializeError(format!("Error reading tag '{}': {}. Possibly wrong type?", name, e)))?;

        if let NBTTag::End = tag { break; }

        compound_data.insert(name, tag);
    }

    Ok(NBTTag::Compound(compound_data))
}

fn read_tag_based_on_type(cursor: &mut Cursor<Vec<u8>>, tag_type: u8) -> NBTResult<NBTTag> {
    match tag_type {
        0 => Ok(NBTTag::End),
        1 => Ok(NBTTag::Byte(cursor.read_i8()?)),
        2 => Ok(NBTTag::Short(cursor.read_i16()?)),
        3 => Ok(NBTTag::Int(cursor.read_i32()?)),
        4 => Ok(NBTTag::Long(cursor.read_i64()?)),
        5 => Ok(NBTTag::Float(cursor.read_f32()?)),
        6 => Ok(NBTTag::Double(cursor.read_f64()?)),
        7 => Ok(NBTTag::ByteArray(Vec::read_from_bytes(cursor)?)),
        8 => Ok(NBTTag::String(cursor.read_nbt_string()?)),
        9 => {
            let list_type = cursor.read_i8()? as u8;
            let len = cursor.read_i32()?;
            let mut list = Vec::with_capacity(len as usize);
            for _ in 0..len {
                list.push(read_tag_based_on_type(cursor, list_type)?);
            }
            Ok(NBTTag::List(list))
        }
        10 => read_tag(cursor),
        11 => Ok(NBTTag::IntArray(Vec::read_from_bytes(cursor)?)),
        12 => Ok(NBTTag::LongArray(Vec::read_from_bytes(cursor)?)),
        _ => Err(NBTError::DeserializeError(format!("Unknown tag type: {}", tag_type))),
    }
}

impl NBTTag {
    pub fn my_type<'a>(&self) -> &'a str {
        match self {
            NBTTag::End => "TAG_END",
            NBTTag::Byte(_) => "TAG_BYTE",
            NBTTag::Short(_) => "TAG_SHORT",
            NBTTag::Int(_) => "TAG_INT",
            NBTTag::Long(_) => "TAG_LONG",
            NBTTag::Float(_) => "TAG_FLOAT",
            NBTTag::Double(_) => "TAG_DOUBLE",
            NBTTag::ByteArray(_) => "TAG_BYTE_ARRAY",
            NBTTag::String(_) => "TAG_STRING",
            NBTTag::List(_) => "TAG_LIST",
            NBTTag::Compound(_) => "TAG_COMPOUND",
            NBTTag::IntArray(_) => "TAG_INT_ARRAY",
            NBTTag::LongArray(_) => "TAG_LONG_ARRAY",
        }
    }
}
use std::collections::HashMap;
use std::io::{Cursor, Read};
use std::simd::*;
use std::simd::num::SimdInt;
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

#[inline]
pub fn read_tag(cursor: &mut Cursor<Vec<u8>>) -> NBTResult<NBTTag> {
    if cursor.get_ref().len() >= cursor.position() as usize {
        Ok(unsafe { read_tag_unchecked(cursor) })
    }else {
        Err(NBTError::UnexpectedEOF)
    }



 /*   let mut compound_data: HashMap<String, NBTTag> = HashMap::new();

    while cursor.position() < cursor.get_ref().len() as u64 {
        let tag_type: u8 = cursor.read_i8()? as u8;
        if tag_type == 0 {
            break;
        }
        let name: String = cursor.read_nbt_string()?;

        let tag = read_tag_based_on_type(cursor, tag_type)
            .map_err(|e| NBTError::DeserializeError(format!("Error reading tag '{}': {}", name, e)))?;

        if let NBTTag::End = tag { break; }

        compound_data.insert(name, tag);
    }

    Ok(NBTTag::Compound(compound_data))*/
}

#[inline]
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





#[inline(always)]
unsafe fn read_tag_based_on_type_unchecked(cursor: &mut Cursor<Vec<u8>>, tag_type: u8) -> NBTTag {
    match tag_type {
        0 => NBTTag::End,
        1 => NBTTag::Byte(cursor.read_i8_unchecked()),
        2 => NBTTag::Short(cursor.read_i16_unchecked()),
        3 => NBTTag::Int(cursor.read_i32_unchecked()),
        4 => NBTTag::Long(cursor.read_i64_unchecked()),
        5 => NBTTag::Float(cursor.read_f32_unchecked()),
        6 => NBTTag::Double(cursor.read_f64_unchecked()),
        7 => {
            let len = cursor.read_i32_unchecked() as usize;
            let mut vec = Vec::<u8>::with_capacity(len);
            unsafe {
                vec.set_len(len);
                cursor.read_exact(vec.as_mut_slice()).expect("Failed to read byte array");
            }
            // Convert Vec<u8> to Vec<i8>
            NBTTag::ByteArray(vec.into_iter().map(|b| b as i8).collect())
        },
        8 => NBTTag::String(cursor.read_nbt_string_unchecked()),
        9 => {
            let list_type = cursor.read_i8_unchecked() as u8;
            let len = cursor.read_i32_unchecked();
            let mut list = Vec::with_capacity(len as usize);
            for _ in 0..len {
                list.push(read_tag_based_on_type_unchecked(cursor, list_type));
            }
            NBTTag::List(list)
        }
        10 => read_tag_unchecked(cursor),
        11 => {
            let len = cursor.read_i32_unchecked() as usize;
            NBTTag::IntArray(read_int_array_simd(cursor, len))
        },
        12 => {
            let len = cursor.read_i32_unchecked() as usize;
            NBTTag::LongArray(read_long_array_simd(cursor, len))
        },
        _ => std::hint::unreachable_unchecked(),
    }
}

#[inline(always)]
unsafe fn read_tag_unchecked(cursor: &mut Cursor<Vec<u8>>) -> NBTTag {
    let mut compound_data = HashMap::new();

    loop {
        if cursor.position() >= cursor.get_ref().len() as u64 { break; }
        
        let tag_type: u8 = cursor.read_i8_unchecked() as u8;
        if tag_type == 0 {
            break;
        }
        let name: String = cursor.read_nbt_string_unchecked();
        let tag = read_tag_based_on_type_unchecked(cursor, tag_type);
        compound_data.insert(name, tag);
    }

    NBTTag::Compound(compound_data)
}


// SIMD implementations
#[inline(always)]
unsafe fn read_int_array_simd(cursor: &mut Cursor<Vec<u8>>, len: usize) -> Vec<i32> {
    let mut result = Vec::with_capacity(len);
    let mut remaining = len;
    let mut pos = cursor.position() as usize;

    while remaining >= 4 {
        unsafe {
            let simd_val: Simd<i32, 4> = Simd::from_slice(
                std::mem::transmute(&cursor.get_ref()[pos..pos + 16])
            ).swap_bytes();
            result.extend_from_slice(simd_val.as_array());
            pos+=16;
            remaining-=4;
        }
    }

    for _ in 0..remaining {
        result.push(i32::from_be_bytes(*(cursor.get_ref().as_ptr().add(pos) as *const [u8; 4])));
        pos += 4;
    }

    cursor.set_position(pos as u64);
    result
}

#[inline(always)]
unsafe fn read_long_array_simd(cursor: &mut Cursor<Vec<u8>>, len: usize) -> Vec<i64> {
    let mut result = Vec::with_capacity(len);
    let mut remaining = len;
    let mut pos = cursor.position() as usize;

    while remaining >= 2 {
        unsafe {
            let simd_val: Simd<i64, 2> = Simd::from_slice(
                std::mem::transmute(&cursor.get_ref()[pos..pos + 16])
            ).swap_bytes();
            result.extend_from_slice(simd_val.as_array());
            result.extend_from_slice(simd_val.as_array());
            pos+=16;
            remaining-=2;
        }
    }

    if remaining > 0 {
        result.push(i64::from_be_bytes(*(cursor.get_ref().as_ptr().add(pos) as *const [u8; 8])));
        pos += 8;
    }

    cursor.set_position(pos as u64);
    result
}
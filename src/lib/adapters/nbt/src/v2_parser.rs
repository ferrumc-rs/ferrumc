#![allow(unsafe_code)]

use std::collections::HashMap;
use std::fmt::Display;
use std::io::Cursor;
use std::simd::Simd;
use crate::v2_parser::cursor::CursorExt;

mod cursor;
mod read_from_bytes;
mod bytes_impls;

pub trait NBTDeserializeBytes {
    fn read_from_bytes(cursor: &mut Cursor<Vec<u8>>) -> NBTResult<Self>
    where Self: Sized;
}
pub trait NBTDeserialize {
    fn read_from(nbt: NBTTag) -> NBTResult<Self>
    where Self: Sized;
}


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


pub(crate) type NBTResult<T> = std::result::Result<T, NBTError>;

#[derive(Debug, thiserror::Error)]
pub enum NBTError {
    DeserializeError(String),
    UnexpectedEOF,
    IoError(std::io::Error),
    InvalidType(&'static str, &'static str),
    ReadWriteError,
}
impl Display for NBTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[inline]
pub fn read_tag(cursor: &mut Cursor<Vec<u8>>) -> NBTResult<NBTTag> {
    if cursor.get_ref().len() >= cursor.position() as usize {
        Ok(read_tag_checked(cursor)?)
    } else {
        Err(NBTError::UnexpectedEOF)
    }
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
        11 => {
            let len = cursor.read_i32()? as usize;
            Ok(NBTTag::IntArray(read_int_array_simd(cursor, len)))
        }
        12 => {
            let len = cursor.read_i32()? as usize;
            Ok(NBTTag::LongArray(read_long_array_simd(cursor, len)))
        }
        _ => Err(NBTError::DeserializeError(format!(
            "Unknown tag type: {}",
            tag_type
        ))),
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

#[inline]
fn read_tag_checked(cursor: &mut Cursor<Vec<u8>>) -> NBTResult<NBTTag> {
    let mut compound_data = HashMap::new();

    loop {
        if cursor.position() >= cursor.get_ref().len() as u64 {
            break;
        }

        let tag_type: u8 = cursor.read_i8()? as u8;
        if tag_type == 0 {
            break;
        }
        let name: String = cursor.read_nbt_string()?;
        let tag = read_tag_based_on_type(cursor, tag_type)?;
        compound_data.insert(name, tag);
    }

    Ok(NBTTag::Compound(compound_data))
}

/*#[inline(always)]
unsafe fn read_tag_unchecked(cursor: &mut Cursor<Vec<u8>>) -> NBTTag {
    let mut compound_data = HashMap::new();

    loop {
        if cursor.position() >= cursor.get_ref().len() as u64 {
            break;
        }

        let tag_type: u8 = cursor.read_i8_unchecked() as u8;
        if tag_type == 0 {
            break;
        }
        let name: String = cursor.read_nbt_string_unchecked();
        let tag = read_tag_based_on_type_unchecked(cursor, tag_type);
        compound_data.insert(name, tag);
    }

    NBTTag::Compound(compound_data)
}*/
/*
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
                cursor
                    .read_exact(vec.as_mut_slice())
                    .expect("Failed to read byte array");
            }
            // Convert Vec<u8> to Vec<i8>
            NBTTag::ByteArray(vec.into_iter().map(|b| b as i8).collect())
        }
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
        }
        12 => {
            let len = cursor.read_i32_unchecked() as usize;
            NBTTag::LongArray(read_long_array_simd(cursor, len))
        }
        _ => std::hint::unreachable_unchecked(),
    }
}
*/
// SIMD implementations
/*#[inline(always)]
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
}*/
#[inline(always)]
fn read_int_array_simd(cursor: &mut Cursor<Vec<u8>>, len: usize) -> Vec<i32> {
    let mut result = Vec::with_capacity(len);
    let mut remaining = len;
    let mut pos = cursor.position() as usize;
    let data = cursor.get_ref();

    while remaining >= 4 && pos + 16 <= data.len() {
        let chunk = unsafe { std::slice::from_raw_parts(data[pos..].as_ptr(), 16) };
        let bytes: Simd<u8, 16> = Simd::from_slice(chunk);
        let ints = Simd::from_array([
            i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            i32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
            i32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]),
            i32::from_be_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]),
        ]);
        result.extend_from_slice(ints.as_array());
        pos += 16;
        remaining -= 4;
    }

    while remaining > 0 && pos + 4 <= data.len() {
        let val = i32::from_be_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
        result.push(val);
        pos += 4;
        remaining -= 1;
    }

    cursor.set_position(pos as u64);
    result
}

#[inline(always)]
fn read_long_array_simd(cursor: &mut Cursor<Vec<u8>>, len: usize) -> Vec<i64> {
    let mut result = Vec::with_capacity(len);
    let mut remaining = len;
    let mut pos = cursor.position() as usize;
    let data = cursor.get_ref();

    while remaining >= 2 && pos + 16 <= data.len() {
        let chunk = unsafe { std::slice::from_raw_parts(data[pos..].as_ptr(), 16) };
        let bytes: Simd<u8, 16> = Simd::from_slice(chunk);
        let longs = Simd::from_array([
            i64::from_be_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ]),
            i64::from_be_bytes([
                bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14],
                bytes[15],
            ]),
        ]);
        result.extend_from_slice(longs.as_array());
        pos += 16;
        remaining -= 2;
    }

    while remaining > 0 && pos + 8 <= data.len() {
        let val = i64::from_be_bytes([
            data[pos],
            data[pos + 1],
            data[pos + 2],
            data[pos + 3],
            data[pos + 4],
            data[pos + 5],
            data[pos + 6],
            data[pos + 7],
        ]);
        result.push(val);
        pos += 8;
        remaining -= 1;
    }

    cursor.set_position(pos as u64);
    result
}

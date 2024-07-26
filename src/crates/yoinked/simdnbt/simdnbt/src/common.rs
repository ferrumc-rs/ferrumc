use std::{mem, slice};

use crate::{
    error::UnexpectedEofError,
    raw_list::RawList,
    reader::Reader,
    swap_endianness::{swap_endianness_as_u8, SwappableNumber},
    Mutf8Str,
};

pub const END_ID: u8 = 0;
pub const BYTE_ID: u8 = 1;
pub const SHORT_ID: u8 = 2;
pub const INT_ID: u8 = 3;
pub const LONG_ID: u8 = 4;
pub const FLOAT_ID: u8 = 5;
pub const DOUBLE_ID: u8 = 6;
pub const BYTE_ARRAY_ID: u8 = 7;
pub const STRING_ID: u8 = 8;
pub const LIST_ID: u8 = 9;
pub const COMPOUND_ID: u8 = 10;
pub const INT_ARRAY_ID: u8 = 11;
pub const LONG_ARRAY_ID: u8 = 12;

pub const MAX_DEPTH: usize = 512;

#[inline(always)]
pub fn read_with_u16_length<'a>(
    data: &mut Reader<'a>,
    width: usize,
) -> Result<&'a [u8], UnexpectedEofError> {
    let length = data.read_u16()?;
    let length_in_bytes = length as usize * width;
    data.read_slice(length_in_bytes)
}

#[inline(never)]
pub fn read_with_u32_length<'a>(
    data: &mut Reader<'a>,
    width: usize,
) -> Result<&'a [u8], UnexpectedEofError> {
    let length = data.read_u32()?;
    let length_in_bytes = length as usize * width;
    data.read_slice(length_in_bytes)
}

pub fn read_string<'a>(data: &mut Reader<'a>) -> Result<&'a Mutf8Str, UnexpectedEofError> {
    let data = read_with_u16_length(data, 1)?;
    Ok(Mutf8Str::from_slice(data))
}

pub fn read_u8_array<'a>(data: &mut Reader<'a>) -> Result<&'a [u8], UnexpectedEofError> {
    read_with_u32_length(data, 1)
}
pub fn read_i8_array<'a>(data: &mut Reader<'a>) -> Result<&'a [i8], UnexpectedEofError> {
    Ok(slice_u8_into_i8(read_u8_array(data)?))
}

pub fn read_int_array<'a>(data: &mut Reader<'a>) -> Result<RawList<'a, i32>, UnexpectedEofError> {
    let array_bytes = read_with_u32_length(data, 4)?;
    Ok(RawList::new(array_bytes))
}

pub fn read_long_array<'a>(data: &mut Reader<'a>) -> Result<RawList<'a, i64>, UnexpectedEofError> {
    let array_bytes = read_with_u32_length(data, 8)?;
    Ok(RawList::new(array_bytes))
}

fn slice_u8_into_i8(s: &[u8]) -> &[i8] {
    unsafe { slice::from_raw_parts(s.as_ptr() as *const i8, s.len()) }
}

pub fn slice_i8_into_u8(s: &[i8]) -> &[u8] {
    unsafe { slice::from_raw_parts(s.as_ptr() as *const u8, s.len()) }
}

#[inline(always)]
pub fn write_with_u32_length(data: &mut Vec<u8>, width: usize, value: &[u8]) {
    let length = value.len() / width;
    data.reserve(4 + value.len());
    unsafe {
        unchecked_extend(data, &(length as u32).to_be_bytes());
        unchecked_extend(data, value);
    }
}

pub fn write_u32(data: &mut Vec<u8>, value: u32) {
    data.extend_from_slice(&value.to_be_bytes());
}
pub fn write_string(data: &mut Vec<u8>, value: &Mutf8Str) {
    data.reserve(2 + value.len());
    // SAFETY: We reserved enough capacity
    unsafe {
        unchecked_write_string(data, value);
    }
}
/// Write a string to a Vec<u8> without checking if the Vec has enough capacity.
/// This is unsafe because it can cause a buffer overflow if the Vec doesn't have enough capacity.
///
/// # Safety
///
/// You must reserve enough capacity (2 + value.len()) in the Vec before calling this function.
#[inline]
pub unsafe fn unchecked_write_string(data: &mut Vec<u8>, value: &Mutf8Str) {
    unchecked_extend(data, &(value.len() as u16).to_be_bytes());
    unchecked_extend(data, value.as_bytes());
}

/// Extend a Vec<u8> with a slice of u8 without checking if the Vec has enough capacity.
///
/// This optimization is barely measurable, but it does make it slightly faster!
///
/// # Safety
///
/// You must reserve enough capacity in the Vec before calling this function.
#[inline]
pub unsafe fn unchecked_extend(data: &mut Vec<u8>, value: &[u8]) {
    let ptr = data.as_mut_ptr();
    let len = data.len();
    std::ptr::copy_nonoverlapping(value.as_ptr(), ptr.add(len), value.len());
    data.set_len(len + value.len());
}

#[inline]
pub unsafe fn unchecked_push(data: &mut Vec<u8>, value: u8) {
    let ptr = data.as_mut_ptr();
    let len = data.len();
    std::ptr::write(ptr.add(len), value);
    data.set_len(len + 1);
}

/// Convert a slice of any type into a slice of u8. This will probably return the data as little
/// endian! Use [`slice_into_u8_big_endian`] to get big endian (the endianness that's used in NBT).
#[inline]
pub fn slice_into_u8_native_endian<T>(s: &[T]) -> &[u8] {
    unsafe { slice::from_raw_parts(s.as_ptr() as *const u8, mem::size_of_val(s)) }
}

/// Convert a slice of any type into a Vec<u8>. This will return the data as big endian (the
/// endianness that's used in NBT).
#[inline]
pub fn slice_into_u8_big_endian<T: SwappableNumber>(s: &[T]) -> Vec<u8> {
    swap_endianness_as_u8::<T>(slice_into_u8_native_endian(s))
}

#[cfg(test)]
mod tests {
    use super::*;

    // this test specifically checks with little-endian
    #[cfg(target_endian = "little")]
    #[test]
    fn test_slice_into_u8_native_endian() {
        assert_eq!(slice_into_u8_native_endian(&[1u16, 2u16]), [1, 0, 2, 0]);
        assert_eq!(
            slice_into_u8_native_endian(&[1u32, 2u32]),
            [1, 0, 0, 0, 2, 0, 0, 0]
        );
    }

    #[test]
    fn test_slice_into_u8_big_endian() {
        assert_eq!(slice_into_u8_big_endian(&[1u16, 2u16]), [0, 1, 0, 2]);
        assert_eq!(
            slice_into_u8_big_endian(&[1u32, 2u32]),
            [0, 0, 0, 1, 0, 0, 0, 2]
        );
    }
}

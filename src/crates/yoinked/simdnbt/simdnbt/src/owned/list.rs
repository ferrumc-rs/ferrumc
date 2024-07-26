use crate::{
    common::{
        read_i8_array, read_int_array, read_long_array, read_string, read_u8_array,
        read_with_u32_length, slice_i8_into_u8, slice_into_u8_big_endian, unchecked_extend,
        unchecked_push, write_string, write_u32, write_with_u32_length, BYTE_ARRAY_ID, BYTE_ID,
        COMPOUND_ID, DOUBLE_ID, END_ID, FLOAT_ID, INT_ARRAY_ID, INT_ID, LIST_ID, LONG_ARRAY_ID,
        LONG_ID, SHORT_ID, STRING_ID,
    },
    error::NonRootError,
    mutf8::Mutf8String,
    reader::Reader,
    swap_endianness::swap_endianness,
};

use super::{compound::NbtCompound, MAX_DEPTH};

/// A list of NBT tags of a single type.
#[repr(u8)]
#[derive(Debug, Default, Clone, PartialEq)]
pub enum NbtList {
    #[default]
    Empty = END_ID,
    Byte(Vec<i8>) = BYTE_ID,
    Short(Vec<i16>) = SHORT_ID,
    Int(Vec<i32>) = INT_ID,
    Long(Vec<i64>) = LONG_ID,
    Float(Vec<f32>) = FLOAT_ID,
    Double(Vec<f64>) = DOUBLE_ID,
    ByteArray(Vec<Vec<u8>>) = BYTE_ARRAY_ID,
    String(Vec<Mutf8String>) = STRING_ID,
    List(Vec<NbtList>) = LIST_ID,
    Compound(Vec<NbtCompound>) = COMPOUND_ID,
    IntArray(Vec<Vec<i32>>) = INT_ARRAY_ID,
    LongArray(Vec<Vec<i64>>) = LONG_ARRAY_ID,
}
impl NbtList {
    pub(crate) fn read(data: &mut Reader<'_>, depth: usize) -> Result<Self, NonRootError> {
        if depth > MAX_DEPTH {
            return Err(NonRootError::max_depth_exceeded());
        }
        let tag_type = data.read_u8().map_err(|_| NonRootError::unexpected_eof())?;
        Ok(match tag_type {
            END_ID => {
                data.skip(4)?;
                NbtList::Empty
            }
            BYTE_ID => NbtList::Byte(read_i8_array(data)?.to_owned()),
            SHORT_ID => NbtList::Short(swap_endianness(read_with_u32_length(data, 2)?)),
            INT_ID => NbtList::Int(swap_endianness(read_with_u32_length(data, 4)?)),
            LONG_ID => NbtList::Long(swap_endianness(read_with_u32_length(data, 8)?)),
            FLOAT_ID => NbtList::Float(swap_endianness(read_with_u32_length(data, 4)?)),
            DOUBLE_ID => NbtList::Double(swap_endianness(read_with_u32_length(data, 8)?)),
            BYTE_ARRAY_ID => NbtList::ByteArray({
                let length = data.read_u32()?;
                // arbitrary number to prevent big allocations
                let mut arrays = Vec::with_capacity(length.min(128) as usize);
                for _ in 0..length {
                    arrays.push(read_u8_array(data)?.to_vec())
                }
                arrays
            }),
            STRING_ID => NbtList::String({
                let length = data.read_u32()?;
                // arbitrary number to prevent big allocations
                let mut strings = Vec::with_capacity(length.min(128) as usize);
                for _ in 0..length {
                    strings.push(read_string(data)?.to_owned())
                }
                strings
            }),
            LIST_ID => NbtList::List({
                let length = data.read_u32()?;
                // arbitrary number to prevent big allocations
                let mut lists = Vec::with_capacity(length.min(128) as usize);
                for _ in 0..length {
                    lists.push(NbtList::read(data, depth + 1)?)
                }
                lists
            }),
            COMPOUND_ID => NbtList::Compound({
                let length = data.read_u32()?;
                // arbitrary number to prevent big allocations
                let mut compounds = Vec::with_capacity(length.min(128) as usize);
                let mut capacity: usize = 8;
                for _ in 0..length {
                    let tag = NbtCompound::read_with_depth_and_capacity(data, depth + 1, capacity)?;
                    capacity = tag.len();
                    compounds.push(tag);
                }
                compounds
            }),
            INT_ARRAY_ID => NbtList::IntArray({
                let length = data.read_u32()?;
                // arbitrary number to prevent big allocations
                let mut arrays = Vec::with_capacity(length.min(128) as usize);
                for _ in 0..length {
                    arrays.push(read_int_array(data)?.to_vec())
                }
                arrays
            }),
            LONG_ARRAY_ID => NbtList::LongArray({
                let length = data.read_u32()?;
                // arbitrary number to prevent big allocations
                let mut arrays = Vec::with_capacity(length.min(128) as usize);
                for _ in 0..length {
                    arrays.push(read_long_array(data)?.to_vec())
                }
                arrays
            }),
            _ => return Err(NonRootError::unknown_tag_id(tag_type)),
        })
    }

    pub fn write(&self, data: &mut Vec<u8>) {
        // fast path for compound since it's very common to have lists of compounds
        if let NbtList::Compound(compounds) = self {
            data.reserve(5);
            // SAFETY: we just reserved 5 bytes
            unsafe {
                unchecked_push(data, COMPOUND_ID);
                unchecked_extend(data, &(compounds.len() as u32).to_be_bytes());
            }
            for compound in compounds {
                compound.write(data);
            }
            return;
        }

        data.push(self.id());
        match self {
            NbtList::Empty => {
                data.extend(&0u32.to_be_bytes());
            }
            NbtList::Byte(bytes) => {
                write_with_u32_length(data, 1, slice_i8_into_u8(bytes));
            }
            NbtList::Short(shorts) => {
                write_with_u32_length(data, 2, &slice_into_u8_big_endian(shorts));
            }
            NbtList::Int(ints) => {
                write_with_u32_length(data, 4, &slice_into_u8_big_endian(ints));
            }
            NbtList::Long(longs) => {
                write_with_u32_length(data, 8, &slice_into_u8_big_endian(longs));
            }
            NbtList::Float(floats) => {
                write_with_u32_length(data, 4, &slice_into_u8_big_endian(floats));
            }
            NbtList::Double(doubles) => {
                write_with_u32_length(data, 8, &slice_into_u8_big_endian(doubles));
            }
            NbtList::ByteArray(byte_arrays) => {
                write_u32(data, byte_arrays.len() as u32);
                for array in byte_arrays {
                    write_with_u32_length(data, 1, array);
                }
            }
            NbtList::String(strings) => {
                write_u32(data, strings.len() as u32);
                for string in strings {
                    write_string(data, string);
                }
            }
            NbtList::List(lists) => {
                write_u32(data, lists.len() as u32);
                for list in lists {
                    list.write(data);
                }
            }
            NbtList::Compound(_) => {
                unreachable!("fast path for compound should have been taken")
            }
            NbtList::IntArray(int_arrays) => {
                write_u32(data, int_arrays.len() as u32);
                for array in int_arrays {
                    write_with_u32_length(data, 4, &slice_into_u8_big_endian(array));
                }
            }
            NbtList::LongArray(long_arrays) => {
                write_u32(data, long_arrays.len() as u32);
                for array in long_arrays {
                    write_with_u32_length(data, 8, &slice_into_u8_big_endian(array));
                }
            }
        }
    }

    /// Get the numerical ID of the tag type.
    #[inline]
    pub fn id(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)`
        // `union` between `repr(C)` structs, each of which has the `u8`
        // discriminant as its first field, so we can read the discriminant
        // without offsetting the pointer.
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }

    pub fn bytes(&self) -> Option<&[i8]> {
        match self {
            NbtList::Byte(bytes) => Some(bytes),
            _ => None,
        }
    }
    pub fn into_bytes(self) -> Option<Vec<i8>> {
        match self {
            NbtList::Byte(bytes) => Some(bytes),
            _ => None,
        }
    }

    pub fn shorts(&self) -> Option<Vec<i16>> {
        match self {
            NbtList::Short(shorts) => Some(shorts.to_vec()),
            _ => None,
        }
    }
    pub fn into_shorts(self) -> Option<Vec<i16>> {
        match self {
            NbtList::Short(shorts) => Some(shorts),
            _ => None,
        }
    }

    pub fn ints(&self) -> Option<Vec<i32>> {
        match self {
            NbtList::Int(ints) => Some(ints.to_vec()),
            _ => None,
        }
    }
    pub fn into_ints(self) -> Option<Vec<i32>> {
        match self {
            NbtList::Int(ints) => Some(ints),
            _ => None,
        }
    }

    pub fn longs(&self) -> Option<Vec<i64>> {
        match self {
            NbtList::Long(longs) => Some(longs.to_vec()),
            _ => None,
        }
    }
    pub fn into_longs(self) -> Option<Vec<i64>> {
        match self {
            NbtList::Long(longs) => Some(longs),
            _ => None,
        }
    }

    pub fn floats(&self) -> Option<Vec<f32>> {
        match self {
            NbtList::Float(floats) => Some(floats.to_vec()),
            _ => None,
        }
    }
    pub fn into_floats(self) -> Option<Vec<f32>> {
        match self {
            NbtList::Float(floats) => Some(floats),
            _ => None,
        }
    }

    pub fn doubles(&self) -> Option<Vec<f64>> {
        match self {
            NbtList::Double(doubles) => Some(doubles.to_vec()),
            _ => None,
        }
    }
    pub fn into_doubles(self) -> Option<Vec<f64>> {
        match self {
            NbtList::Double(doubles) => Some(doubles),
            _ => None,
        }
    }

    pub fn byte_arrays(&self) -> Option<&[Vec<u8>]> {
        match self {
            NbtList::ByteArray(byte_arrays) => Some(byte_arrays),
            _ => None,
        }
    }
    pub fn into_byte_arrays(self) -> Option<Vec<Vec<u8>>> {
        match self {
            NbtList::ByteArray(byte_arrays) => Some(byte_arrays),
            _ => None,
        }
    }

    pub fn strings(&self) -> Option<&[Mutf8String]> {
        match self {
            NbtList::String(strings) => Some(strings),
            _ => None,
        }
    }
    pub fn into_strings(self) -> Option<Vec<Mutf8String>> {
        match self {
            NbtList::String(strings) => Some(strings),
            _ => None,
        }
    }

    pub fn lists(&self) -> Option<&[NbtList]> {
        match self {
            NbtList::List(lists) => Some(lists),
            _ => None,
        }
    }
    pub fn into_lists(self) -> Option<Vec<NbtList>> {
        match self {
            NbtList::List(lists) => Some(lists),
            _ => None,
        }
    }

    pub fn compounds(&self) -> Option<&[NbtCompound]> {
        match self {
            NbtList::Compound(compounds) => Some(compounds),
            _ => None,
        }
    }
    pub fn into_compounds(self) -> Option<Vec<NbtCompound>> {
        match self {
            NbtList::Compound(compounds) => Some(compounds),
            _ => None,
        }
    }

    pub fn int_arrays(&self) -> Option<&[Vec<i32>]> {
        match self {
            NbtList::IntArray(int_arrays) => Some(int_arrays),
            _ => None,
        }
    }
    pub fn into_int_arrays(self) -> Option<Vec<Vec<i32>>> {
        match self {
            NbtList::IntArray(int_arrays) => Some(int_arrays),
            _ => None,
        }
    }

    pub fn long_arrays(&self) -> Option<&[Vec<i64>]> {
        match self {
            NbtList::LongArray(long_arrays) => Some(long_arrays),
            _ => None,
        }
    }
    pub fn into_long_arrays(self) -> Option<Vec<Vec<i64>>> {
        match self {
            NbtList::LongArray(long_arrays) => Some(long_arrays),
            _ => None,
        }
    }

    pub fn as_nbt_tags(&self) -> Vec<super::NbtTag> {
        match self {
            NbtList::Empty => vec![],
            NbtList::Byte(bytes) => bytes.iter().copied().map(super::NbtTag::Byte).collect(),
            NbtList::Short(shorts) => shorts.iter().copied().map(super::NbtTag::Short).collect(),
            NbtList::Int(ints) => ints.iter().copied().map(super::NbtTag::Int).collect(),
            NbtList::Long(longs) => longs.iter().copied().map(super::NbtTag::Long).collect(),
            NbtList::Float(floats) => floats.iter().copied().map(super::NbtTag::Float).collect(),
            NbtList::Double(doubles) => {
                doubles.iter().copied().map(super::NbtTag::Double).collect()
            }
            NbtList::ByteArray(byte_arrays) => byte_arrays
                .iter()
                .cloned()
                .map(super::NbtTag::ByteArray)
                .collect(),
            NbtList::String(strings) => {
                strings.iter().cloned().map(super::NbtTag::String).collect()
            }
            NbtList::List(lists) => lists.iter().cloned().map(super::NbtTag::List).collect(),
            NbtList::Compound(compounds) => compounds
                .iter()
                .cloned()
                .map(super::NbtTag::Compound)
                .collect(),
            NbtList::IntArray(int_arrays) => int_arrays
                .iter()
                .cloned()
                .map(super::NbtTag::IntArray)
                .collect(),
            NbtList::LongArray(long_arrays) => long_arrays
                .iter()
                .cloned()
                .map(super::NbtTag::LongArray)
                .collect(),
        }
    }
}

impl From<Vec<i8>> for NbtList {
    fn from(bytes: Vec<i8>) -> Self {
        NbtList::Byte(bytes)
    }
}
impl From<Vec<i16>> for NbtList {
    fn from(shorts: Vec<i16>) -> Self {
        NbtList::Short(shorts)
    }
}
impl From<Vec<i32>> for NbtList {
    fn from(ints: Vec<i32>) -> Self {
        NbtList::Int(ints)
    }
}
impl From<Vec<i64>> for NbtList {
    fn from(longs: Vec<i64>) -> Self {
        NbtList::Long(longs)
    }
}
impl From<Vec<f32>> for NbtList {
    fn from(floats: Vec<f32>) -> Self {
        NbtList::Float(floats)
    }
}
impl From<Vec<f64>> for NbtList {
    fn from(doubles: Vec<f64>) -> Self {
        NbtList::Double(doubles)
    }
}
impl From<Vec<Vec<u8>>> for NbtList {
    fn from(byte_arrays: Vec<Vec<u8>>) -> Self {
        NbtList::ByteArray(byte_arrays)
    }
}
impl From<Vec<Mutf8String>> for NbtList {
    fn from(strings: Vec<Mutf8String>) -> Self {
        NbtList::String(strings)
    }
}
impl From<Vec<String>> for NbtList {
    fn from(strings: Vec<String>) -> Self {
        NbtList::String(strings.into_iter().map(Mutf8String::from).collect())
    }
}
impl From<Vec<NbtList>> for NbtList {
    fn from(lists: Vec<NbtList>) -> Self {
        NbtList::List(lists)
    }
}
impl From<Vec<NbtCompound>> for NbtList {
    fn from(compounds: Vec<NbtCompound>) -> Self {
        NbtList::Compound(compounds)
    }
}
impl From<Vec<Vec<i32>>> for NbtList {
    fn from(int_arrays: Vec<Vec<i32>>) -> Self {
        NbtList::IntArray(int_arrays)
    }
}
impl From<Vec<Vec<i64>>> for NbtList {
    fn from(long_arrays: Vec<Vec<i64>>) -> Self {
        NbtList::LongArray(long_arrays)
    }
}

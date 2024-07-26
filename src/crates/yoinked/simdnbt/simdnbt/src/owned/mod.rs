//! The owned variant of NBT. This is useful if you're writing NBT or if you can't keep a reference
//! to the original data.

mod compound;
mod list;

use std::{io::Cursor, ops::Deref};

use crate::{
    common::{
        read_int_array, read_long_array, read_string, read_with_u32_length,
        slice_into_u8_big_endian, unchecked_extend, unchecked_push, write_string, BYTE_ARRAY_ID,
        BYTE_ID, COMPOUND_ID, DOUBLE_ID, END_ID, FLOAT_ID, INT_ARRAY_ID, INT_ID, LIST_ID,
        LONG_ARRAY_ID, LONG_ID, MAX_DEPTH, SHORT_ID, STRING_ID,
    },
    error::NonRootError,
    mutf8::Mutf8String,
    reader::{Reader, ReaderFromCursor},
    Error, Mutf8Str,
};

pub use self::{compound::NbtCompound, list::NbtList};

/// Read a normal root NBT compound. This is either empty or has a name and compound tag.
///
/// Returns `Ok(Nbt::None)` if there is no data.
pub fn read(data: &mut Cursor<&[u8]>) -> Result<Nbt, Error> {
    let mut reader = ReaderFromCursor::new(data);
    Nbt::read(&mut reader)
}
/// Read a root NBT compound, but without reading the name. This is used in Minecraft when reading
/// NBT over the network.
///
/// This is similar to [`read_tag`], but returns an [`Nbt`] instead (guaranteeing it'll be either
/// empty or a compound).
pub fn read_unnamed(data: &mut Cursor<&[u8]>) -> Result<Nbt, Error> {
    let mut reader = ReaderFromCursor::new(data);
    Nbt::read_unnamed(&mut reader)
}
/// Read a compound tag. This may have any number of items.
pub fn read_compound(data: &mut Cursor<&[u8]>) -> Result<NbtCompound, NonRootError> {
    let mut reader = ReaderFromCursor::new(data);
    NbtCompound::read(&mut reader)
}
/// Read an NBT tag, without reading its name. This may be any type of tag except for an end tag. If you need to be able to
/// handle end tags, use [`read_optional_tag`].
pub fn read_tag(data: &mut Cursor<&[u8]>) -> Result<NbtTag, NonRootError> {
    let mut reader = ReaderFromCursor::new(data);
    NbtTag::read(&mut reader)
}
/// Read any NBT tag, without reading its name. This may be any type of tag, including an end tag.
///
/// Returns `Ok(None)` if there is no data.
pub fn read_optional_tag(data: &mut Cursor<&[u8]>) -> Result<Option<NbtTag>, NonRootError> {
    let mut reader = ReaderFromCursor::new(data);
    NbtTag::read_optional(&mut reader)
}

/// A complete NBT container. This contains a name and a compound tag.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BaseNbt {
    name: Mutf8String,
    tag: NbtCompound,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Nbt {
    Some(BaseNbt),
    #[default]
    None,
}

impl Nbt {
    pub fn new(name: Mutf8String, tag: NbtCompound) -> Self {
        Self::Some(BaseNbt { name, tag })
    }

    /// Reads NBT from the given data. Returns `Ok(Nbt::None)` if there is no data.
    fn read(data: &mut Reader<'_>) -> Result<Nbt, Error> {
        let root_type = data.read_u8().map_err(|_| Error::UnexpectedEof)?;
        if root_type == END_ID {
            return Ok(Nbt::None);
        }
        if root_type != COMPOUND_ID {
            return Err(Error::InvalidRootType(root_type));
        }
        let name = read_string(data)?.to_owned();
        let tag = NbtCompound::read(data)?;

        Ok(Nbt::Some(BaseNbt { name, tag }))
    }

    fn read_unnamed(data: &mut Reader<'_>) -> Result<Nbt, Error> {
        let root_type = data.read_u8().map_err(|_| Error::UnexpectedEof)?;
        if root_type == END_ID {
            return Ok(Nbt::None);
        }
        if root_type != COMPOUND_ID {
            return Err(Error::InvalidRootType(root_type));
        }
        let tag = NbtCompound::read(data)?;

        Ok(Nbt::Some(BaseNbt {
            name: Mutf8String::from(""),
            tag,
        }))
    }

    pub fn write(&self, data: &mut Vec<u8>) {
        match self {
            Nbt::Some(nbt) => nbt.write(data),
            Nbt::None => {
                data.push(END_ID);
            }
        }
    }

    pub fn write_unnamed(&self, data: &mut Vec<u8>) {
        match self {
            Nbt::Some(nbt) => nbt.write_unnamed(data),
            Nbt::None => {
                data.push(END_ID);
            }
        }
    }

    pub fn unwrap(self) -> BaseNbt {
        match self {
            Nbt::Some(nbt) => nbt,
            Nbt::None => panic!("called `OptionalNbt::unwrap()` on a `None` value"),
        }
    }

    pub fn unwrap_or<'a>(&'a self, default: &'a BaseNbt) -> &'a BaseNbt {
        match self {
            Nbt::Some(nbt) => nbt,
            Nbt::None => default,
        }
    }

    pub fn is_some(&self) -> bool {
        match self {
            Nbt::Some(_) => true,
            Nbt::None => false,
        }
    }

    pub fn is_none(&self) -> bool {
        !self.is_some()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Mutf8Str, &NbtTag)> {
        const EMPTY: &NbtCompound = &NbtCompound { values: Vec::new() };

        if let Nbt::Some(nbt) = self {
            nbt.iter()
        } else {
            EMPTY.iter()
        }
    }
}
impl Deref for Nbt {
    type Target = BaseNbt;

    fn deref(&self) -> &Self::Target {
        const EMPTY: &BaseNbt = &BaseNbt {
            name: Mutf8String { vec: Vec::new() },
            tag: NbtCompound { values: Vec::new() },
        };

        match self {
            Nbt::Some(nbt) => nbt,
            Nbt::None => EMPTY,
        }
    }
}

impl IntoIterator for Nbt {
    type Item = (Mutf8String, NbtTag);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        const EMPTY: NbtCompound = NbtCompound { values: Vec::new() };

        match self {
            Nbt::Some(nbt) => nbt.tag.into_iter(),
            Nbt::None => EMPTY.into_iter(),
        }
    }
}

impl BaseNbt {
    pub fn new(name: impl Into<Mutf8String>, tag: NbtCompound) -> Self {
        let name = name.into();
        Self { name, tag }
    }

    /// Get the name of the NBT compound. This is often an empty string.
    pub fn name(&self) -> &Mutf8Str {
        &self.name
    }

    /// Writes the NBT to the given buffer.
    pub fn write(&self, data: &mut Vec<u8>) {
        data.push(COMPOUND_ID);
        write_string(data, &self.name);
        self.tag.write(data);
    }

    pub fn write_unnamed(&self, data: &mut Vec<u8>) {
        data.push(COMPOUND_ID);
        self.tag.write(data);
    }

    pub fn into_inner(self) -> NbtCompound {
        self.tag
    }
}

impl IntoIterator for BaseNbt {
    type Item = (Mutf8String, NbtTag);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.tag.into_iter()
    }
}

impl Deref for BaseNbt {
    type Target = NbtCompound;

    fn deref(&self) -> &Self::Target {
        &self.tag
    }
}

/// A single NBT tag.
#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
pub enum NbtTag {
    Byte(i8) = BYTE_ID,
    Short(i16) = SHORT_ID,
    Int(i32) = INT_ID,
    Long(i64) = LONG_ID,
    Float(f32) = FLOAT_ID,
    Double(f64) = DOUBLE_ID,
    ByteArray(Vec<u8>) = BYTE_ARRAY_ID,
    String(Mutf8String) = STRING_ID,
    List(NbtList) = LIST_ID,
    Compound(NbtCompound) = COMPOUND_ID,
    IntArray(Vec<i32>) = INT_ARRAY_ID,
    LongArray(Vec<i64>) = LONG_ARRAY_ID,
}
impl NbtTag {
    /// Get the numerical ID of the tag type.
    #[inline]
    pub fn id(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)`
        // `union` between `repr(C)` structs, each of which has the `u8`
        // discriminant as its first field, so we can read the discriminant
        // without offsetting the pointer.
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }

    #[inline(always)]
    fn read_with_type(
        data: &mut Reader<'_>,
        tag_type: u8,
        depth: usize,
    ) -> Result<Self, NonRootError> {
        match tag_type {
            BYTE_ID => Ok(NbtTag::Byte(
                data.read_i8().map_err(|_| NonRootError::unexpected_eof())?,
            )),
            SHORT_ID => Ok(NbtTag::Short(
                data.read_i16()
                    .map_err(|_| NonRootError::unexpected_eof())?,
            )),
            INT_ID => Ok(NbtTag::Int(
                data.read_i32()
                    .map_err(|_| NonRootError::unexpected_eof())?,
            )),
            LONG_ID => Ok(NbtTag::Long(
                data.read_i64()
                    .map_err(|_| NonRootError::unexpected_eof())?,
            )),
            FLOAT_ID => Ok(NbtTag::Float(
                data.read_f32()
                    .map_err(|_| NonRootError::unexpected_eof())?,
            )),
            DOUBLE_ID => Ok(NbtTag::Double(
                data.read_f64()
                    .map_err(|_| NonRootError::unexpected_eof())?,
            )),
            BYTE_ARRAY_ID => Ok(NbtTag::ByteArray(read_with_u32_length(data, 1)?.to_owned())),
            STRING_ID => Ok(NbtTag::String(read_string(data)?.to_owned())),
            LIST_ID => Ok(NbtTag::List(NbtList::read(data, depth + 1)?)),
            COMPOUND_ID => Ok(NbtTag::Compound(NbtCompound::read_with_depth(
                data,
                depth + 1,
            )?)),
            INT_ARRAY_ID => Ok(NbtTag::IntArray(read_int_array(data)?.to_vec())),
            LONG_ARRAY_ID => Ok(NbtTag::LongArray(read_long_array(data)?.to_vec())),
            _ => Err(NonRootError::unknown_tag_id(tag_type)),
        }
    }

    fn read(data: &mut Reader<'_>) -> Result<Self, NonRootError> {
        let tag_type = data.read_u8().map_err(|_| NonRootError::unexpected_eof())?;
        Self::read_with_type(data, tag_type, 0)
    }

    fn read_optional(data: &mut Reader<'_>) -> Result<Option<Self>, NonRootError> {
        let tag_type = data.read_u8().map_err(|_| NonRootError::unexpected_eof())?;
        if tag_type == END_ID {
            return Ok(None);
        }
        Ok(Some(Self::read_with_type(data, tag_type, 0)?))
    }

    /// Write to the data without checking that there's enough space in it.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it doesn't check that there's enough space in the data.
    /// 4 bytes MUST be reserved before calling this function.
    #[inline]
    unsafe fn unchecked_write_without_tag_type(&self, data: &mut Vec<u8>) {
        match self {
            NbtTag::Byte(byte) => unsafe {
                unchecked_push(data, *byte as u8);
            },
            NbtTag::Short(short) => unsafe {
                unchecked_extend(data, &short.to_be_bytes());
            },
            NbtTag::Int(int) => unsafe {
                unchecked_extend(data, &int.to_be_bytes());
            },
            NbtTag::Long(long) => {
                data.extend_from_slice(&long.to_be_bytes());
            }
            NbtTag::Float(float) => unsafe {
                unchecked_extend(data, &float.to_be_bytes());
            },
            NbtTag::Double(double) => {
                data.extend_from_slice(&double.to_be_bytes());
            }
            NbtTag::ByteArray(byte_array) => {
                unsafe {
                    unchecked_extend(data, &byte_array.len().to_be_bytes());
                }
                data.extend_from_slice(byte_array);
            }
            NbtTag::String(string) => {
                write_string(data, string);
            }
            NbtTag::List(list) => {
                list.write(data);
            }
            NbtTag::Compound(compound) => {
                compound.write(data);
            }
            NbtTag::IntArray(int_array) => {
                unsafe {
                    unchecked_extend(data, &int_array.len().to_be_bytes());
                }
                data.extend_from_slice(&slice_into_u8_big_endian(int_array));
            }
            NbtTag::LongArray(long_array) => {
                unsafe {
                    unchecked_extend(data, &long_array.len().to_be_bytes());
                }
                data.extend_from_slice(&slice_into_u8_big_endian(long_array));
            }
        }
    }

    pub fn write(&self, data: &mut Vec<u8>) {
        data.reserve(1 + 4);
        // SAFETY: We just reserved enough space for the tag ID and 4 bytes of tag data.
        unsafe {
            unchecked_push(data, self.id());
            self.unchecked_write_without_tag_type(data);
        }
    }

    pub fn byte(&self) -> Option<i8> {
        match self {
            NbtTag::Byte(byte) => Some(*byte),
            _ => None,
        }
    }
    pub fn byte_mut(&mut self) -> Option<&mut i8> {
        match self {
            NbtTag::Byte(byte) => Some(byte),
            _ => None,
        }
    }
    pub fn into_byte(self) -> Option<i8> {
        match self {
            NbtTag::Byte(byte) => Some(byte),
            _ => None,
        }
    }

    pub fn short(&self) -> Option<i16> {
        match self {
            NbtTag::Short(short) => Some(*short),
            _ => None,
        }
    }
    pub fn short_mut(&mut self) -> Option<&mut i16> {
        match self {
            NbtTag::Short(short) => Some(short),
            _ => None,
        }
    }
    pub fn into_short(self) -> Option<i16> {
        match self {
            NbtTag::Short(short) => Some(short),
            _ => None,
        }
    }

    pub fn int(&self) -> Option<i32> {
        match self {
            NbtTag::Int(int) => Some(*int),
            _ => None,
        }
    }
    pub fn int_mut(&mut self) -> Option<&mut i32> {
        match self {
            NbtTag::Int(int) => Some(int),
            _ => None,
        }
    }
    pub fn into_int(self) -> Option<i32> {
        match self {
            NbtTag::Int(int) => Some(int),
            _ => None,
        }
    }

    pub fn long(&self) -> Option<i64> {
        match self {
            NbtTag::Long(long) => Some(*long),
            _ => None,
        }
    }
    pub fn long_mut(&mut self) -> Option<&mut i64> {
        match self {
            NbtTag::Long(long) => Some(long),
            _ => None,
        }
    }
    pub fn into_long(self) -> Option<i64> {
        match self {
            NbtTag::Long(long) => Some(long),
            _ => None,
        }
    }

    pub fn float(&self) -> Option<f32> {
        match self {
            NbtTag::Float(float) => Some(*float),
            _ => None,
        }
    }
    pub fn float_mut(&mut self) -> Option<&mut f32> {
        match self {
            NbtTag::Float(float) => Some(float),
            _ => None,
        }
    }
    pub fn into_float(self) -> Option<f32> {
        match self {
            NbtTag::Float(float) => Some(float),
            _ => None,
        }
    }

    pub fn double(&self) -> Option<f64> {
        match self {
            NbtTag::Double(double) => Some(*double),
            _ => None,
        }
    }
    pub fn double_mut(&mut self) -> Option<&mut f64> {
        match self {
            NbtTag::Double(double) => Some(double),
            _ => None,
        }
    }
    pub fn into_double(self) -> Option<f64> {
        match self {
            NbtTag::Double(double) => Some(double),
            _ => None,
        }
    }

    pub fn byte_array(&self) -> Option<&[u8]> {
        match self {
            NbtTag::ByteArray(byte_array) => Some(byte_array),
            _ => None,
        }
    }
    pub fn byte_array_mut(&mut self) -> Option<&mut Vec<u8>> {
        match self {
            NbtTag::ByteArray(byte_array) => Some(byte_array),
            _ => None,
        }
    }
    pub fn into_byte_array(self) -> Option<Vec<u8>> {
        match self {
            NbtTag::ByteArray(byte_array) => Some(byte_array),
            _ => None,
        }
    }

    pub fn string(&self) -> Option<&Mutf8Str> {
        match self {
            NbtTag::String(string) => Some(string),
            _ => None,
        }
    }
    pub fn string_mut(&mut self) -> Option<&mut Mutf8String> {
        match self {
            NbtTag::String(string) => Some(string),
            _ => None,
        }
    }
    pub fn into_string(self) -> Option<Mutf8String> {
        match self {
            NbtTag::String(string) => Some(string),
            _ => None,
        }
    }

    pub fn list(&self) -> Option<&NbtList> {
        match self {
            NbtTag::List(list) => Some(list),
            _ => None,
        }
    }
    pub fn list_mut(&mut self) -> Option<&mut NbtList> {
        match self {
            NbtTag::List(list) => Some(list),
            _ => None,
        }
    }
    pub fn into_list(self) -> Option<NbtList> {
        match self {
            NbtTag::List(list) => Some(list),
            _ => None,
        }
    }

    pub fn compound(&self) -> Option<&NbtCompound> {
        match self {
            NbtTag::Compound(compound) => Some(compound),
            _ => None,
        }
    }
    pub fn compound_mut(&mut self) -> Option<&mut NbtCompound> {
        match self {
            NbtTag::Compound(compound) => Some(compound),
            _ => None,
        }
    }
    pub fn into_compound(self) -> Option<NbtCompound> {
        match self {
            NbtTag::Compound(compound) => Some(compound),
            _ => None,
        }
    }

    pub fn int_array(&self) -> Option<&[i32]> {
        match self {
            NbtTag::IntArray(int_array) => Some(int_array),
            _ => None,
        }
    }
    pub fn int_array_mut(&mut self) -> Option<&mut Vec<i32>> {
        match self {
            NbtTag::IntArray(int_array) => Some(int_array),
            _ => None,
        }
    }
    pub fn into_int_array(self) -> Option<Vec<i32>> {
        match self {
            NbtTag::IntArray(int_array) => Some(int_array),
            _ => None,
        }
    }

    pub fn long_array(&self) -> Option<&[i64]> {
        match self {
            NbtTag::LongArray(long_array) => Some(long_array),
            _ => None,
        }
    }
    pub fn long_array_mut(&mut self) -> Option<&mut Vec<i64>> {
        match self {
            NbtTag::LongArray(long_array) => Some(long_array),
            _ => None,
        }
    }
    pub fn into_long_array(self) -> Option<Vec<i64>> {
        match self {
            NbtTag::LongArray(long_array) => Some(long_array),
            _ => None,
        }
    }
}

impl From<NbtCompound> for BaseNbt {
    fn from(tag: NbtCompound) -> Self {
        Self {
            name: Mutf8String::from(""),
            tag,
        }
    }
}

impl From<Nbt> for NbtTag {
    fn from(value: Nbt) -> Self {
        match value {
            Nbt::Some(nbt) => NbtTag::Compound(nbt.tag),
            Nbt::None => NbtTag::Compound(NbtCompound::new()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use byteorder::{WriteBytesExt, BE};
    use flate2::read::GzDecoder;

    use super::*;

    #[test]
    fn hello_world() {
        let nbt = super::read(&mut Cursor::new(include_bytes!(
            "../../tests/hello_world.nbt"
        )))
        .unwrap()
        .unwrap();

        assert_eq!(
            nbt.string("name"),
            Some(Mutf8Str::from_str("Bananrama").as_ref())
        );
        assert_eq!(nbt.name().to_str(), "hello world");
    }

    #[test]
    fn simple_player() {
        let src = include_bytes!("../../tests/simple_player.dat").to_vec();
        let mut src_slice = src.as_slice();
        let mut decoded_src_decoder = GzDecoder::new(&mut src_slice);
        let mut decoded_src = Vec::new();
        decoded_src_decoder.read_to_end(&mut decoded_src).unwrap();
        let nbt = super::read(&mut Cursor::new(&decoded_src))
            .unwrap()
            .unwrap();

        assert_eq!(nbt.int("PersistentId"), Some(1946940766));
        assert_eq!(nbt.list("Rotation").unwrap().floats().unwrap().len(), 2);
    }

    #[test]
    fn complex_player() {
        let src = include_bytes!("../../tests/complex_player.dat").to_vec();
        let mut src_slice = src.as_slice();
        let mut decoded_src_decoder = GzDecoder::new(&mut src_slice);
        let mut decoded_src = Vec::new();
        decoded_src_decoder.read_to_end(&mut decoded_src).unwrap();
        let nbt = super::read(&mut Cursor::new(&decoded_src))
            .unwrap()
            .unwrap();

        assert_eq!(nbt.float("foodExhaustionLevel").unwrap() as u32, 2);
        assert_eq!(nbt.list("Rotation").unwrap().floats().unwrap().len(), 2);
    }

    #[test]
    fn read_write_complex_player() {
        let src = include_bytes!("../../tests/complex_player.dat").to_vec();
        let mut src_slice = src.as_slice();
        let mut decoded_src_decoder = GzDecoder::new(&mut src_slice);
        let mut decoded_src = Vec::new();
        decoded_src_decoder.read_to_end(&mut decoded_src).unwrap();
        let nbt = super::read(&mut Cursor::new(&decoded_src))
            .unwrap()
            .unwrap();

        let mut out = Vec::new();
        nbt.write(&mut out);
        let nbt = super::read(&mut Cursor::new(&out)).unwrap().unwrap();

        assert_eq!(nbt.float("foodExhaustionLevel").unwrap() as u32, 2);
        assert_eq!(nbt.list("Rotation").unwrap().floats().unwrap().len(), 2);
    }

    #[test]
    fn inttest_1023() {
        let nbt = super::read(&mut Cursor::new(include_bytes!(
            "../../tests/inttest1023.nbt"
        )))
        .unwrap()
        .unwrap();

        let ints = nbt.list("").unwrap().ints().unwrap();

        for (i, &item) in ints.iter().enumerate() {
            assert_eq!(i as i32, item);
        }
        assert_eq!(ints.len(), 1023);
    }

    #[test]
    fn inttest_1024() {
        let mut data = Vec::new();
        data.write_u8(COMPOUND_ID).unwrap();
        data.write_u16::<BE>(0).unwrap();
        data.write_u8(LIST_ID).unwrap();
        data.write_u16::<BE>(0).unwrap();
        data.write_u8(INT_ID).unwrap();
        data.write_i32::<BE>(1024).unwrap();
        for i in 0..1024 {
            data.write_i32::<BE>(i).unwrap();
        }
        data.write_u8(END_ID).unwrap();

        let nbt = super::read(&mut Cursor::new(&data)).unwrap().unwrap();
        let ints = nbt.list("").unwrap().ints().unwrap();
        for (i, &item) in ints.iter().enumerate() {
            assert_eq!(i as i32, item);
        }
        assert_eq!(ints.len(), 1024);
    }

    #[test]
    fn inttest_1021() {
        let mut data = Vec::new();
        data.write_u8(COMPOUND_ID).unwrap();
        data.write_u16::<BE>(0).unwrap();
        data.write_u8(LIST_ID).unwrap();
        data.write_u16::<BE>(0).unwrap();
        data.write_u8(INT_ID).unwrap();
        data.write_i32::<BE>(1021).unwrap();
        for i in 0..1021 {
            data.write_i32::<BE>(i).unwrap();
        }
        data.write_u8(END_ID).unwrap();

        let nbt = super::read(&mut Cursor::new(&data)).unwrap().unwrap();
        let ints = nbt.list("").unwrap().ints().unwrap();
        for (i, &item) in ints.iter().enumerate() {
            assert_eq!(i as i32, item);
        }
        assert_eq!(ints.len(), 1021);
    }

    #[test]
    fn longtest_1023() {
        let mut data = Vec::new();
        data.write_u8(COMPOUND_ID).unwrap();
        data.write_u16::<BE>(0).unwrap();
        data.write_u8(LIST_ID).unwrap();
        data.write_u16::<BE>(0).unwrap();
        data.write_u8(LONG_ID).unwrap();
        data.write_i32::<BE>(1023).unwrap();
        for i in 0..1023 {
            data.write_i64::<BE>(i).unwrap();
        }
        data.write_u8(END_ID).unwrap();

        let nbt = super::read(&mut Cursor::new(&data)).unwrap().unwrap();
        let ints = nbt.list("").unwrap().longs().unwrap();
        for (i, &item) in ints.iter().enumerate() {
            assert_eq!(i as i64, item);
        }
        assert_eq!(ints.len(), 1023);
    }
}

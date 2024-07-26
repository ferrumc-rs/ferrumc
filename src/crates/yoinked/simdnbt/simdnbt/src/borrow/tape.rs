use std::fmt::{self, Debug};

use crate::common::{
    BYTE_ARRAY_ID, BYTE_ID, COMPOUND_ID, DOUBLE_ID, FLOAT_ID, INT_ARRAY_ID, INT_ID, LONG_ARRAY_ID,
    LONG_ID, SHORT_ID, STRING_ID,
};

#[derive(Debug)]
pub struct MainTape {
    elements: Vec<TapeElement>,
}
impl MainTape {
    #[inline]
    pub fn push(&mut self, element: TapeElement) {
        self.elements.push(element);
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    #[inline]
    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut TapeElement {
        self.elements.get_unchecked_mut(index)
    }

    #[inline]
    pub fn as_ptr(&self) -> *const TapeElement {
        self.elements.as_ptr()
    }
}
impl Default for MainTape {
    fn default() -> Self {
        Self {
            elements: Vec::with_capacity(1024),
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union TapeElement {
    pub kind: (TapeTagKind, TapeTagValue),

    pub long: i64,
    pub double: f64,

    pub name: u64, // pointer to the original data
}
impl TapeElement {
    /// Returns how much we should increment the tape index to get to the next tag.
    ///
    /// # Safety
    /// The element must be a tag and not something else like a continuation of a long or double.
    pub unsafe fn skip_offset(&self) -> usize {
        match self.kind {
            (TapeTagKind::Long | TapeTagKind::Double, _) => 2,
            (
                TapeTagKind::Compound,
                TapeTagValue {
                    compound: (_, offset),
                },
            ) => u32::from(offset) as usize,
            (
                TapeTagKind::ListList,
                TapeTagValue {
                    list_list: (_, offset),
                },
            ) => u32::from(offset) as usize,
            (
                TapeTagKind::CompoundList,
                TapeTagValue {
                    compound_list: (_, offset),
                },
            ) => u32::from(offset) as usize,
            _ => 1,
        }
    }
}
impl Debug for TapeElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // just writes the u64
        write!(f, "TapeElement({:#016x})", unsafe { self.name })?;
        Ok(())
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union TapeTagValue {
    pub byte: i8,
    pub short: i16,
    pub int: i32,
    pub long: (), // value is in next tape element
    pub float: f32,
    pub double: (),                    // value is in next tape element
    pub byte_array: u56,               // pointer to the original data
    pub string: u56,                   // pointer to the original data
    pub compound: (u24, UnalignedU32), // length estimate + tape index offset to the end of the compound
    pub int_array: u56,                // pointer to the original data
    pub long_array: u56,               // pointer to the original data

    // lists
    pub empty_list: (),
    pub byte_list: u56,                       // pointer to the original data
    pub short_list: u56,                      // pointer to the original data
    pub int_list: u56,                        // pointer to the original data
    pub long_list: u56,                       // pointer to the original data
    pub float_list: u56,                      // pointer to the original data
    pub double_list: u56,                     // pointer to the original data
    pub byte_array_list: (u24, UnalignedU32), // padding + index to ExtraTapes which has a fat pointer that points to the original data
    pub string_list: (u24, UnalignedU32),     // padding + index to ExtraTapes
    pub list_list: (u24, UnalignedU32), // length estimate + tape index offset to the end of the list
    pub compound_list: (u24, UnalignedU32), // length estimate + tape index offset to the end of the list
    pub int_array_list: (u24, UnalignedU32), // padding + index to ExtraTapes
    pub long_array_list: (u24, UnalignedU32), // padding + index to ExtraTapes
}

#[derive(Debug, Copy, Clone)]
#[repr(packed)]
#[allow(non_camel_case_types)]
pub struct u56 {
    a: u8,
    b: u16,
    c: u32,
}
impl From<u64> for u56 {
    #[inline]
    fn from(value: u64) -> Self {
        let a = (value >> 48) as u8;
        let b = (value >> 32) as u16;
        let c = value as u32;
        Self { a, b, c }
    }
}
impl From<u56> for u64 {
    #[inline]
    fn from(value: u56) -> Self {
        let a = value.a as u64;
        let b = value.b as u64;
        let c = value.c as u64;
        (a << 48) | (b << 32) | c
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(packed)]
#[allow(non_camel_case_types)]
pub struct u24 {
    a: u8,
    b: u16,
}
impl From<u32> for u24 {
    #[inline]
    fn from(value: u32) -> Self {
        let a = (value >> 16) as u8;
        let b = value as u16;
        Self { a, b }
    }
}
impl From<u24> for u32 {
    #[inline]
    fn from(value: u24) -> Self {
        let a = value.a as u32;
        let b = value.b as u32;
        (a << 16) | b
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(packed)]
pub struct UnalignedU32(pub u32);
impl From<u32> for UnalignedU32 {
    #[inline]
    fn from(value: u32) -> Self {
        Self(value)
    }
}
impl From<UnalignedU32> for u32 {
    #[inline]
    fn from(value: UnalignedU32) -> Self {
        value.0
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(packed)]
pub struct UnalignedU16(pub u16);
impl From<u16> for UnalignedU16 {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl From<UnalignedU16> for u16 {
    #[inline]
    fn from(value: UnalignedU16) -> Self {
        value.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TapeTagKind {
    Byte = BYTE_ID,
    Short = SHORT_ID,
    Int = INT_ID,
    Long = LONG_ID,
    Float = FLOAT_ID,
    Double = DOUBLE_ID,
    ByteArray = BYTE_ARRAY_ID,
    String = STRING_ID,
    Compound = COMPOUND_ID,
    IntArray = INT_ARRAY_ID,
    LongArray = LONG_ARRAY_ID,

    EmptyList,
    ByteList,
    ShortList,
    IntList,
    LongList,
    FloatList,
    DoubleList,
    ByteArrayList,
    StringList,
    ListList,
    CompoundList,
    IntArrayList,
    LongArrayList,
}

impl TapeTagKind {
    pub fn is_list(&self) -> bool {
        matches!(
            self,
            TapeTagKind::EmptyList
                | TapeTagKind::ByteList
                | TapeTagKind::ShortList
                | TapeTagKind::IntList
                | TapeTagKind::LongList
                | TapeTagKind::FloatList
                | TapeTagKind::DoubleList
                | TapeTagKind::ByteArrayList
                | TapeTagKind::StringList
                | TapeTagKind::ListList
                | TapeTagKind::CompoundList
                | TapeTagKind::IntArrayList
                | TapeTagKind::LongArrayList
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u56() {
        // top 8 bits are cut off
        let value = 0x1234_5678_9abc_def0;
        let u56 { a, b, c } = u56::from(value);
        assert_eq!(a, 0x34);
        assert_eq!(b, 0x5678);
        assert_eq!(c, 0x9abc_def0);

        let value: u64 = u56 { a, b, c }.into();
        assert_eq!(value, 0x34_5678_9abc_def0);
    }
}

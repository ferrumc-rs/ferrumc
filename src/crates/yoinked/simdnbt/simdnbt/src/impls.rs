use crate::{FromNbtTag, owned, ToNbtTag};
use crate::borrow::NbtTag;
// Implementations for primitive types

// Special case for Vec<i8> (byte array)
impl FromNbtTag for Vec<i8> {
    fn from_nbt_tag(tag: NbtTag) -> Option<Self> {
        tag.byte_array().map(|bytes| bytes.iter().map(|&b| b as i8).collect())
    }
}

impl ToNbtTag for Vec<i8> {
    fn to_nbt_tag(self) -> owned::NbtTag {
        owned::NbtTag::ByteArray(self.into_iter().map(|b| b as u8).collect())
    }
}

// Special case for Vec<i32> (int array)
impl FromNbtTag for Vec<i32> {
    fn from_nbt_tag(tag: NbtTag) -> Option<Self> {
        tag.int_array().map(|ints| ints.iter().map(|&i| i).collect())
    }
}

impl ToNbtTag for Vec<i32> {
    fn to_nbt_tag(self) -> owned::NbtTag {
        owned::NbtTag::IntArray(self)
    }
}

// Special case for Vec<i64> (long array)
impl FromNbtTag for Vec<i64> {
    fn from_nbt_tag(tag: NbtTag) -> Option<Self> {
        tag.long_array().map(|longs| longs.iter().map(|&l| l).collect())
    }
}

impl ToNbtTag for Vec<i64> {
    fn to_nbt_tag(self) -> owned::NbtTag {
        owned::NbtTag::LongArray(self)
    }
}
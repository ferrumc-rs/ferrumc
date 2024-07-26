use std::mem::{self, MaybeUninit};

use crate::{
    common::{read_string, unchecked_push, unchecked_write_string, END_ID, MAX_DEPTH},
    error::NonRootError,
    mutf8::Mutf8String,
    reader::Reader,
    Mutf8Str, ToNbtTag,
};

use super::{list::NbtList, NbtTag};

/// A list of named tags. The order of the tags is preserved.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct NbtCompound {
    pub(crate) values: Vec<(Mutf8String, NbtTag)>,
}

impl NbtCompound {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_values(values: Vec<(Mutf8String, NbtTag)>) -> Self {
        Self { values }
    }

    pub(crate) fn read(data: &mut Reader<'_>) -> Result<Self, NonRootError> {
        Self::read_with_depth(data, 0)
    }

    pub(crate) fn read_with_depth_and_capacity(
        data: &mut Reader<'_>,
        depth: usize,
        capacity: usize,
    ) -> Result<Self, NonRootError> {
        if depth > MAX_DEPTH {
            return Err(NonRootError::max_depth_exceeded());
        }

        let mut tags_buffer = unsafe {
            MaybeUninit::<[MaybeUninit<(Mutf8String, NbtTag)>; 8]>::uninit().assume_init()
        };
        let mut tags_buffer_len: usize = 0;

        let mut values = Vec::with_capacity(capacity);
        loop {
            let tag_type = data.read_u8().map_err(|_| NonRootError::unexpected_eof())?;
            if tag_type == END_ID {
                break;
            }
            let tag_name = read_string(data)?.to_owned();
            let tag = NbtTag::read_with_type(data, tag_type, depth)?;

            tags_buffer[tags_buffer_len] = MaybeUninit::new((tag_name, tag));
            tags_buffer_len += 1;
            if tags_buffer_len == tags_buffer.len() {
                // writing the tags in groups like this is slightly faster
                for i in 0..tags_buffer_len {
                    values.push(unsafe { tags_buffer.get_unchecked(i).assume_init_read() });
                }
                tags_buffer_len = 0;
            }
        }

        for i in 0..tags_buffer_len {
            values.push(unsafe { tags_buffer.get_unchecked(i).assume_init_read() });
        }

        Ok(Self { values })
    }

    pub(crate) fn read_with_depth(
        data: &mut Reader<'_>,
        depth: usize,
    ) -> Result<Self, NonRootError> {
        Self::read_with_depth_and_capacity(data, depth, 8)
    }

    pub fn write(&self, data: &mut Vec<u8>) {
        for (name, tag) in &self.values {
            // reserve 4 bytes extra so we can avoid reallocating for small tags
            data.reserve(1 + 2 + name.len() + 4);
            // SAFETY: We just reserved enough space for the tag ID, the name length, the name, and
            // 4 bytes of tag data.
            unsafe {
                unchecked_push(data, tag.id());
                unchecked_write_string(data, name);
                tag.unchecked_write_without_tag_type(data);
            }
        }
        data.push(END_ID);
    }

    #[inline]
    pub fn get(&self, name: &str) -> Option<&NbtTag> {
        let name = Mutf8Str::from_str(name);
        let name = name.as_ref();
        for (key, value) in &self.values {
            if key.as_str() == name {
                return Some(value);
            }
        }
        None
    }

    #[inline]
    pub fn get_mut(&mut self, name: &str) -> Option<&mut NbtTag> {
        let name = Mutf8Str::from_str(name);
        let name = name.as_ref();
        for (key, value) in &mut self.values {
            if key.as_str() == name {
                return Some(value);
            }
        }
        None
    }

    /// Get an owned tag from the compound by swapping it with a dummy tag.
    pub fn take(&mut self, name: &str) -> Option<NbtTag> {
        let name = Mutf8Str::from_str(name);
        let name = name.as_ref();
        for i in 0..self.values.len() {
            if self.values[i].0.as_str() == name {
                let mut value = NbtTag::Byte(0);
                mem::swap(&mut self.values[i].1, &mut value);
                return Some(value);
            }
        }
        None
    }

    /// Returns whether there is a tag with the given name.
    pub fn contains(&self, name: &str) -> bool {
        let name = Mutf8Str::from_str(name);
        let name = name.as_ref();
        for (key, _) in &self.values {
            if key.as_str() == name {
                return true;
            }
        }
        false
    }

    pub fn byte(&self, name: &str) -> Option<i8> {
        self.get(name).and_then(|tag| tag.byte())
    }
    pub fn byte_mut(&mut self, name: &str) -> Option<&mut i8> {
        self.get_mut(name).and_then(|tag| tag.byte_mut())
    }
    pub fn short(&self, name: &str) -> Option<i16> {
        self.get(name).and_then(|tag| tag.short())
    }
    pub fn short_mut(&mut self, name: &str) -> Option<&mut i16> {
        self.get_mut(name).and_then(|tag| tag.short_mut())
    }
    pub fn int(&self, name: &str) -> Option<i32> {
        self.get(name).and_then(|tag| tag.int())
    }
    pub fn int_mut(&mut self, name: &str) -> Option<&mut i32> {
        self.get_mut(name).and_then(|tag| tag.int_mut())
    }
    pub fn long(&self, name: &str) -> Option<i64> {
        self.get(name).and_then(|tag| tag.long())
    }
    pub fn long_mut(&mut self, name: &str) -> Option<&mut i64> {
        self.get_mut(name).and_then(|tag| tag.long_mut())
    }
    pub fn float(&self, name: &str) -> Option<f32> {
        self.get(name).and_then(|tag| tag.float())
    }
    pub fn float_mut(&mut self, name: &str) -> Option<&mut f32> {
        self.get_mut(name).and_then(|tag| tag.float_mut())
    }
    pub fn double(&self, name: &str) -> Option<f64> {
        self.get(name).and_then(|tag| tag.double())
    }
    pub fn double_mut(&mut self, name: &str) -> Option<&mut f64> {
        self.get_mut(name).and_then(|tag| tag.double_mut())
    }
    pub fn byte_array(&self, name: &str) -> Option<&[u8]> {
        self.get(name).and_then(|tag| tag.byte_array())
    }
    pub fn byte_array_mut(&mut self, name: &str) -> Option<&mut Vec<u8>> {
        self.get_mut(name).and_then(|tag| tag.byte_array_mut())
    }
    pub fn string(&self, name: &str) -> Option<&Mutf8Str> {
        self.get(name).and_then(|tag| tag.string())
    }
    pub fn string_mut(&mut self, name: &str) -> Option<&mut Mutf8String> {
        self.get_mut(name).and_then(|tag| tag.string_mut())
    }
    pub fn list(&self, name: &str) -> Option<&NbtList> {
        self.get(name).and_then(|tag| tag.list())
    }
    pub fn list_mut(&mut self, name: &str) -> Option<&mut NbtList> {
        self.get_mut(name).and_then(|tag| tag.list_mut())
    }
    pub fn compound(&self, name: &str) -> Option<&NbtCompound> {
        self.get(name).and_then(|tag| tag.compound())
    }
    pub fn compound_mut(&mut self, name: &str) -> Option<&mut NbtCompound> {
        self.get_mut(name).and_then(|tag| tag.compound_mut())
    }
    pub fn int_array(&self, name: &str) -> Option<&[i32]> {
        self.get(name).and_then(|tag| tag.int_array())
    }
    pub fn int_array_mut(&mut self, name: &str) -> Option<&mut Vec<i32>> {
        self.get_mut(name).and_then(|tag| tag.int_array_mut())
    }
    pub fn long_array(&self, name: &str) -> Option<&[i64]> {
        self.get(name).and_then(|tag| tag.long_array())
    }
    pub fn long_array_mut(&mut self, name: &str) -> Option<&mut Vec<i64>> {
        self.get_mut(name).and_then(|tag| tag.long_array_mut())
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Mutf8Str, &NbtTag)> {
        self.values.iter().map(|(k, v)| (k.as_str(), v))
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Mutf8Str, &mut NbtTag)> {
        self.values.iter_mut().map(|(k, v)| (k.as_str(), v))
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn keys(&self) -> impl Iterator<Item = &Mutf8Str> {
        self.values.iter().map(|(k, _)| k.as_str())
    }
    pub fn keys_mut(&mut self) -> impl Iterator<Item = &mut Mutf8String> {
        self.values.iter_mut().map(|(k, _)| k)
    }
    pub fn values(&self) -> impl Iterator<Item = &NbtTag> {
        self.values.iter().map(|(_, v)| v)
    }
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut NbtTag> {
        self.values.iter_mut().map(|(_, v)| v)
    }
    pub fn clear(&mut self) {
        self.values.clear();
    }
    pub fn insert(&mut self, name: impl Into<Mutf8String>, tag: impl ToNbtTag) {
        let name = name.into();
        let tag = tag.to_nbt_tag();
        self.values.push((name, tag));
    }
    pub fn extend(
        &mut self,
        other: impl IntoIterator<Item = (impl Into<Mutf8String>, impl ToNbtTag)>,
    ) {
        self.values.extend(
            other
                .into_iter()
                .map(|(name, tag)| (name.into(), tag.to_nbt_tag())),
        );
    }
    pub fn remove(&mut self, name: &str) -> Option<NbtTag> {
        let name = Mutf8Str::from_str(name);
        let name = name.as_ref();
        for i in 0..self.values.len() {
            if self.values[i].0.as_str() == name {
                return Some(self.values.remove(i).1);
            }
        }
        None
    }
}

impl IntoIterator for NbtCompound {
    type Item = (Mutf8String, NbtTag);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

use std::{hint::unreachable_unchecked, mem::MaybeUninit};

use crate::{
    common::{
        read_int_array, read_long_array, read_string, read_with_u32_length, unchecked_extend,
        unchecked_push, unchecked_write_string, write_string, BYTE_ARRAY_ID, BYTE_ID, COMPOUND_ID,
        DOUBLE_ID, END_ID, FLOAT_ID, INT_ARRAY_ID, INT_ID, LIST_ID, LONG_ARRAY_ID, LONG_ID,
        MAX_DEPTH, SHORT_ID, STRING_ID,
    },
    error::NonRootError,
    reader::Reader,
    Mutf8Str,
};

use super::{
    extra_tapes::ExtraTapes,
    list::{self, NbtList},
    tape::{TapeElement, TapeTagKind, TapeTagValue, UnalignedU16},
    NbtTag, Tapes,
};

#[derive(Debug, Clone, Copy)]
pub struct NbtCompound<'a: 'tape, 'tape> {
    pub(crate) element: *const TapeElement, // includes the initial compound element
    pub(crate) extra_tapes: &'tape ExtraTapes<'a>,
}

impl<'a: 'tape, 'tape> NbtCompound<'a, 'tape> {
    pub(crate) fn read(
        // compounds have no header so nothing to read
        _data: &mut Reader<'a>,
        tapes: &'tape mut Tapes<'a>,
        stack: &mut ParsingStack,
    ) -> Result<(), NonRootError> {
        let index_of_compound_element = tapes.main.len();

        stack.push(ParsingStackElement::Compound {
            index_of_compound_element: index_of_compound_element as u32,
        })?;
        tapes.main.push(TapeElement {
            kind: (
                TapeTagKind::Compound,
                TapeTagValue {
                    // this gets overwritten later
                    compound: (0.into(), 0.into()),
                },
            ),
        });

        Ok(())
    }

    pub fn write(&self, data: &mut Vec<u8>) {
        for (name, tag) in self.iter() {
            // reserve 4 bytes extra so we can avoid reallocating for small tags
            data.reserve(1 + 2 + name.len() + 4);
            // SAFETY: We just reserved enough space for the tag ID, the name length, the name, and
            // 4 bytes of tag data.
            unsafe {
                unchecked_push(data, tag.id());
                unchecked_write_string(data, name);
            }

            write_tag(tag, data);
        }
        data.push(END_ID);
    }

    #[inline]
    pub fn get(&self, name: &str) -> Option<NbtTag<'a, 'tape>> {
        let name = Mutf8Str::from_str(name);
        let name = name.as_ref();
        for (key, value) in self.iter() {
            if key == name {
                return Some(value);
            }
        }
        None
    }

    /// Returns whether there is a tag with the given name.
    pub fn contains(&self, name: &str) -> bool {
        let name = Mutf8Str::from_str(name);
        let name = name.as_ref();
        for key in self.keys() {
            if key == name {
                return true;
            }
        }
        false
    }

    pub fn byte(&self, name: &str) -> Option<i8> {
        self.get(name).and_then(|tag| tag.byte())
    }
    pub fn short(&self, name: &str) -> Option<i16> {
        self.get(name).and_then(|tag| tag.short())
    }
    pub fn int(&self, name: &str) -> Option<i32> {
        self.get(name).and_then(|tag| tag.int())
    }
    pub fn long(&self, name: &str) -> Option<i64> {
        self.get(name).and_then(|tag| tag.long())
    }
    pub fn float(&self, name: &str) -> Option<f32> {
        self.get(name).and_then(|tag| tag.float())
    }
    pub fn double(&self, name: &str) -> Option<f64> {
        self.get(name).and_then(|tag| tag.double())
    }
    pub fn byte_array(&self, name: &str) -> Option<&'a [u8]> {
        self.get(name).and_then(|tag| tag.byte_array())
    }
    pub fn string(&self, name: &str) -> Option<&'a Mutf8Str> {
        self.get(name).and_then(|tag| tag.string())
    }
    pub fn list(&self, name: &str) -> Option<NbtList<'a, 'tape>> {
        self.get(name).and_then(|tag| tag.list())
    }
    pub fn compound(&self, name: &str) -> Option<NbtCompound<'a, 'tape>> {
        self.get(name).and_then(|tag| tag.compound())
    }
    pub fn int_array(&self, name: &str) -> Option<Vec<i32>> {
        self.get(name).and_then(|tag| tag.int_array())
    }
    pub fn long_array(&self, name: &str) -> Option<Vec<i64>> {
        self.get(name).and_then(|tag| tag.long_array())
    }

    /// Get the tape element kind and value for this compound.
    fn element(&self) -> (TapeTagKind, TapeTagValue) {
        unsafe { (*self.element).kind }
    }

    pub fn iter(&self) -> CompoundIter<'a, 'tape> {
        let (kind, value) = self.element();
        debug_assert_eq!(kind, TapeTagKind::Compound);

        let max_tape_offset = u32::from(unsafe { value.list_list.1 }) as usize;
        let tape_slice =
            unsafe { std::slice::from_raw_parts(self.element.add(1), max_tape_offset) };

        CompoundIter {
            current_tape_offset: 0,
            max_tape_offset,
            tape: tape_slice,
            extra_tapes: self.extra_tapes,
        }
    }

    /// Returns the number of tags directly in this compound.
    ///
    /// Note that due to an internal optimization, this function runs at `O(n)`
    /// if the compound has at least 2^24 items. Use [`Self::approx_len`] if you
    /// want to avoid that.
    pub fn len(&self) -> usize {
        let len = self.approx_len();
        if len < 2usize.pow(24) {
            len
        } else {
            self.iter().count()
        }
    }

    /// A version of [`Self::len`] that saturates at 2^24.
    pub fn approx_len(self) -> usize {
        let (kind, value) = self.element();
        debug_assert_eq!(kind, TapeTagKind::Compound);
        unsafe { u32::from(value.list_list.0) as usize }
    }

    pub fn is_empty(&self) -> bool {
        self.approx_len() == 0
    }
    #[allow(clippy::type_complexity)]
    pub fn keys(
        &self,
    ) -> std::iter::Map<
        CompoundIter<'a, 'tape>,
        fn((&'a Mutf8Str, NbtTag<'a, 'tape>)) -> &'a Mutf8Str,
    > {
        self.iter().map(|(k, _)| k)
    }

    pub fn to_owned(&self) -> crate::owned::NbtCompound {
        crate::owned::NbtCompound {
            values: self
                .iter()
                .map(|(k, v)| ((*k).to_owned(), v.to_owned()))
                .collect(),
        }
    }
}

impl PartialEq for NbtCompound<'_, '_> {
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}

pub struct CompoundIter<'a: 'tape, 'tape> {
    current_tape_offset: usize,
    max_tape_offset: usize,
    tape: &'tape [TapeElement],
    extra_tapes: &'tape ExtraTapes<'a>,
}
impl<'a: 'tape, 'tape> Iterator for CompoundIter<'a, 'tape> {
    type Item = (&'a Mutf8Str, NbtTag<'a, 'tape>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_tape_offset + 1 >= self.max_tape_offset {
            return None;
        }

        let name_length_ptr = unsafe { self.tape[self.current_tape_offset].name };
        let name_length_ptr = name_length_ptr as *const UnalignedU16;
        let name_length = u16::from(unsafe { *name_length_ptr }).swap_bytes();
        let name_pointer = unsafe { name_length_ptr.add(1) as *const u8 };
        let name_slice = unsafe { std::slice::from_raw_parts(name_pointer, name_length as usize) };
        let name = Mutf8Str::from_slice(name_slice);

        self.current_tape_offset += 1;

        let element = unsafe { self.tape.as_ptr().add(self.current_tape_offset) };
        let tag = NbtTag {
            element,
            extra_tapes: self.extra_tapes,
        };

        self.current_tape_offset += unsafe { (*element).skip_offset() };

        Some((name, tag))
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum ParsingStackElement {
    Compound { index_of_compound_element: u32 },
    ListOfCompounds { index_of_list_element: u32 },
    ListOfLists { index_of_list_element: u32 },
}

pub struct ParsingStack {
    stack: [MaybeUninit<ParsingStackElement>; MAX_DEPTH],
    remaining_elements_in_lists: [u32; MAX_DEPTH],
    depth: usize,
}

impl ParsingStack {
    pub fn new() -> Self {
        Self {
            stack: unsafe { MaybeUninit::uninit().assume_init() },
            remaining_elements_in_lists: [0; MAX_DEPTH],
            depth: 0,
        }
    }

    #[inline]
    pub fn push(&mut self, state: ParsingStackElement) -> Result<(), NonRootError> {
        unsafe { self.stack.get_unchecked_mut(self.depth).write(state) };
        self.depth += 1;

        if self.depth >= MAX_DEPTH {
            Err(NonRootError::max_depth_exceeded())
        } else {
            Ok(())
        }
    }

    #[inline]
    pub fn set_list_length(&mut self, length: u32) {
        unsafe {
            *self
                .remaining_elements_in_lists
                .get_unchecked_mut(self.depth - 1) = length;
        };
    }

    #[inline]
    pub fn decrement_list_length(&mut self) {
        unsafe {
            *self
                .remaining_elements_in_lists
                .get_unchecked_mut(self.depth - 1) -= 1;
        };
    }

    #[inline]
    pub fn remaining_elements_in_list(&self) -> u32 {
        unsafe {
            *self
                .remaining_elements_in_lists
                .get_unchecked(self.depth - 1)
        }
    }

    #[inline]
    pub fn pop(&mut self) -> ParsingStackElement {
        self.depth -= 1;
        unsafe { self.stack.get_unchecked(self.depth).assume_init() }
    }

    #[inline]
    pub fn peek(&self) -> ParsingStackElement {
        unsafe { self.stack.get_unchecked(self.depth - 1).assume_init() }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.depth == 0
    }

    #[inline]
    pub fn peek_mut(&mut self) -> &mut ParsingStackElement {
        unsafe {
            self.stack
                .get_unchecked_mut(self.depth - 1)
                .as_mut_ptr()
                .as_mut()
                .unwrap_unchecked()
        }
    }
}

#[inline(always)]
pub(crate) fn read_tag<'a>(
    data: &mut Reader<'a>,
    tapes: &mut Tapes<'a>,
    stack: &mut ParsingStack,
    tag_type: u8,
) -> Result<(), NonRootError> {
    match tag_type {
        COMPOUND_ID => return NbtCompound::read(data, tapes, stack),
        LIST_ID => return NbtList::read(data, tapes, stack),
        _ => {}
    }

    match tag_type {
        BYTE_ID => {
            let byte = data.read_i8()?;
            tapes.main.push(TapeElement {
                kind: (TapeTagKind::Byte, TapeTagValue { byte }),
            });
        }
        SHORT_ID => {
            let short = data.read_i16()?;
            tapes.main.push(TapeElement {
                kind: (TapeTagKind::Short, TapeTagValue { short }),
            });
        }
        INT_ID => {
            let int = data.read_i32()?;
            tapes.main.push(TapeElement {
                kind: (TapeTagKind::Int, TapeTagValue { int }),
            });
        }
        LONG_ID => {
            let long = data.read_i64()?;
            tapes.main.push(TapeElement {
                kind: (TapeTagKind::Long, TapeTagValue { long: () }),
            });
            tapes.main.push(TapeElement { long });
        }
        FLOAT_ID => {
            let float = data.read_f32()?;
            tapes.main.push(TapeElement {
                kind: (TapeTagKind::Float, TapeTagValue { float }),
            });
        }
        DOUBLE_ID => {
            let double = data.read_f64()?;
            tapes.main.push(TapeElement {
                kind: (TapeTagKind::Double, TapeTagValue { double: () }),
            });
            tapes.main.push(TapeElement { double });
        }
        BYTE_ARRAY_ID => {
            let byte_array_pointer = data.cur as u64;
            read_with_u32_length(data, 1)?;
            tapes.main.push(TapeElement {
                kind: (
                    TapeTagKind::ByteArray,
                    TapeTagValue {
                        byte_array: byte_array_pointer.into(),
                    },
                ),
            });
        }
        STRING_ID => {
            let string_pointer = data.cur as u64;

            // assert that the top 8 bits of the pointer are 0 (because we rely on this)
            debug_assert_eq!(string_pointer >> 56, 0);

            read_string(data)?;

            tapes.main.push(TapeElement {
                kind: (
                    TapeTagKind::String,
                    TapeTagValue {
                        string: string_pointer.into(),
                    },
                ),
            });
        }
        INT_ARRAY_ID => {
            let int_array_pointer = data.cur as u64;
            read_int_array(data)?;
            tapes.main.push(TapeElement {
                kind: (
                    TapeTagKind::IntArray,
                    TapeTagValue {
                        int_array: int_array_pointer.into(),
                    },
                ),
            });
        }
        LONG_ARRAY_ID => {
            let long_array_pointer = data.cur as u64;
            read_long_array(data)?;
            tapes.main.push(TapeElement {
                kind: (
                    TapeTagKind::LongArray,
                    TapeTagValue {
                        long_array: long_array_pointer.into(),
                    },
                ),
            });
        }
        _ => return Err(NonRootError::unknown_tag_id(tag_type)),
    };
    Ok(())
}

#[inline(always)]
pub(crate) fn read_tag_in_compound<'a>(
    data: &mut Reader<'a>,
    tapes: &mut Tapes<'a>,
    stack: &mut ParsingStack,
) -> Result<(), NonRootError> {
    let tag_type = data.read_u8()?;
    if tag_type == END_ID {
        handle_compound_end(tapes, stack);
        return Ok(());
    }

    let tag_name_pointer = data.cur as u64;
    debug_assert_eq!(tag_name_pointer >> 56, 0);
    read_string(data)?;
    tapes.main.push(TapeElement {
        name: tag_name_pointer,
    });

    read_tag(data, tapes, stack, tag_type)
}

#[inline(always)]
fn handle_compound_end(tapes: &mut Tapes, stack: &mut ParsingStack) {
    let ParsingStackElement::Compound {
        index_of_compound_element,
    } = stack.pop()
    else {
        unsafe { unreachable_unchecked() };
    };
    let index_after_end_element = tapes.main.len();

    unsafe {
        tapes
            .main
            .get_unchecked_mut(index_of_compound_element as usize)
            .kind
            .1
            .compound
            .1 = (index_after_end_element as u32 - index_of_compound_element).into();
    };
}

pub(crate) fn write_tag(tag: NbtTag, data: &mut Vec<u8>) {
    let (kind, value) = tag.element();
    match kind {
        TapeTagKind::Byte => unsafe {
            unchecked_push(data, tag.byte().unwrap() as u8);
        },
        TapeTagKind::Short => unsafe {
            unchecked_extend(data, &tag.short().unwrap().to_be_bytes());
        },
        TapeTagKind::Int => unsafe {
            unchecked_extend(data, &tag.int().unwrap().to_be_bytes());
        },
        TapeTagKind::Long => {
            data.extend_from_slice(&tag.long().unwrap().to_be_bytes());
        }
        TapeTagKind::Float => unsafe {
            unchecked_extend(data, &tag.float().unwrap().to_be_bytes());
        },
        TapeTagKind::Double => {
            data.extend_from_slice(&tag.double().unwrap().to_be_bytes());
        }
        TapeTagKind::ByteArray => {
            let byte_array = tag.byte_array().unwrap();
            unsafe {
                unchecked_extend(data, &byte_array.len().to_be_bytes());
            }
            data.extend_from_slice(byte_array);
        }
        TapeTagKind::String => {
            let string = tag.string().unwrap();
            write_string(data, string);
        }
        _ if kind.is_list() => {
            tag.list().unwrap().write(data);
        }
        TapeTagKind::Compound => {
            tag.compound().unwrap().write(data);
        }
        TapeTagKind::IntArray => {
            let int_array =
                unsafe { list::u32_prefixed_list_to_rawlist_unchecked::<i32>(value).unwrap() };
            unsafe {
                unchecked_extend(data, &int_array.len().to_be_bytes());
            }
            data.extend_from_slice(int_array.as_big_endian());
        }
        TapeTagKind::LongArray => {
            let long_array =
                unsafe { list::u32_prefixed_list_to_rawlist_unchecked::<i64>(value).unwrap() };
            unsafe {
                unchecked_extend(data, &long_array.len().to_be_bytes());
            }
            data.extend_from_slice(long_array.as_big_endian());
        }
        _ => unreachable!("Invalid tag kind {kind:?}"),
    }
}

use std::{hint::unreachable_unchecked, marker::PhantomData};

use crate::{
    common::{
        read_i8_array, read_int_array, read_long_array, read_string, read_u8_array,
        read_with_u32_length, slice_i8_into_u8, write_string, write_u32, write_with_u32_length,
        BYTE_ARRAY_ID, BYTE_ID, COMPOUND_ID, DOUBLE_ID, END_ID, FLOAT_ID, INT_ARRAY_ID, INT_ID,
        LIST_ID, LONG_ARRAY_ID, LONG_ID, SHORT_ID, STRING_ID,
    },
    error::NonRootError,
    raw_list::RawList,
    reader::Reader,
    swap_endianness::SwappableNumber,
    Mutf8Str,
};

use super::{
    compound::{ParsingStack, ParsingStackElement},
    extra_tapes::{ExtraTapeElement, ExtraTapes},
    tape::{TapeElement, TapeTagKind, TapeTagValue, UnalignedU32},
    NbtCompound, Tapes,
};

/// A list of NBT tags of a single type.
#[derive(Clone, Copy, Debug)]
pub struct NbtList<'a: 'tape, 'tape> {
    pub(crate) element: *const TapeElement, // the initial list element
    pub(crate) extra_tapes: &'tape ExtraTapes<'a>,
}
impl<'a, 'tape> NbtList<'a, 'tape> {
    pub(crate) fn read(
        data: &mut Reader<'a>,
        tapes: &mut Tapes<'a>,
        stack: &mut ParsingStack,
    ) -> Result<(), NonRootError> {
        let tag_type = data.read_u8()?;
        match tag_type {
            END_ID => {
                // the length is unused for this type of lists
                data.skip(4)?;
                tapes.main.push(TapeElement {
                    kind: (TapeTagKind::EmptyList, TapeTagValue { empty_list: () }),
                });
            }
            BYTE_ID => {
                let byte_list_pointer = data.cur as u64;
                let _ = read_i8_array(data)?;
                tapes.main.push(TapeElement {
                    kind: (
                        TapeTagKind::ByteList,
                        TapeTagValue {
                            byte_list: byte_list_pointer.into(),
                        },
                    ),
                });
            }
            SHORT_ID => {
                let short_list_pointer = data.cur as u64;
                read_with_u32_length(data, 2)?;
                tapes.main.push(TapeElement {
                    kind: (
                        TapeTagKind::ShortList,
                        TapeTagValue {
                            short_list: short_list_pointer.into(),
                        },
                    ),
                });
            }
            INT_ID => {
                let int_list_pointer = data.cur as u64;
                read_with_u32_length(data, 4)?;
                tapes.main.push(TapeElement {
                    kind: (
                        TapeTagKind::IntList,
                        TapeTagValue {
                            int_list: int_list_pointer.into(),
                        },
                    ),
                });
            }
            LONG_ID => {
                let long_list_pointer = data.cur as u64;
                read_with_u32_length(data, 8)?;
                tapes.main.push(TapeElement {
                    kind: (
                        TapeTagKind::LongList,
                        TapeTagValue {
                            long_list: long_list_pointer.into(),
                        },
                    ),
                });
            }
            FLOAT_ID => {
                let float_list_pointer = data.cur as u64;
                read_with_u32_length(data, 4)?;
                tapes.main.push(TapeElement {
                    kind: (
                        TapeTagKind::FloatList,
                        TapeTagValue {
                            float_list: float_list_pointer.into(),
                        },
                    ),
                });
            }
            DOUBLE_ID => {
                let double_list_pointer = data.cur as u64;
                read_with_u32_length(data, 8)?;
                tapes.main.push(TapeElement {
                    kind: (
                        TapeTagKind::DoubleList,
                        TapeTagValue {
                            double_list: double_list_pointer.into(),
                        },
                    ),
                });
            }
            BYTE_ARRAY_ID => {
                let index_of_element = tapes.extra.elements.len() as u32;
                tapes.main.push(TapeElement {
                    kind: (
                        TapeTagKind::ByteArrayList,
                        TapeTagValue {
                            byte_array_list: (0.into(), index_of_element.into()),
                        },
                    ),
                });

                let length = data.read_u32()?;
                tapes.extra.elements.push(ExtraTapeElement { length });
                for _ in 0..length {
                    let byte_array = read_u8_array(data)?;
                    tapes.extra.elements.push(ExtraTapeElement { byte_array });
                }
            }
            STRING_ID => {
                let index_of_element = tapes.extra.elements.len() as u32;
                tapes.main.push(TapeElement {
                    kind: (
                        TapeTagKind::StringList,
                        TapeTagValue {
                            string_list: (0.into(), index_of_element.into()),
                        },
                    ),
                });

                let length = data.read_u32()?;
                tapes.extra.elements.push(ExtraTapeElement { length });
                for _ in 0..length {
                    let string = read_string(data)?;
                    tapes.extra.elements.push(ExtraTapeElement { string });
                }
            }
            LIST_ID => {
                let length = data.read_u32()?;
                // length estimate + tape index offset to the end of the list
                let index_of_list_element = tapes.main.len();

                stack.push(ParsingStackElement::ListOfLists {
                    index_of_list_element: index_of_list_element as u32,
                })?;
                stack.set_list_length(length);
                tapes.main.push(TapeElement {
                    kind: (
                        TapeTagKind::ListList,
                        TapeTagValue {
                            // can't know the offset until after
                            list_list: (length.into(), 0.into()),
                        },
                    ),
                });
            }
            COMPOUND_ID => {
                let length = data.read_u32()?;
                // length estimate + tape index offset to the end of the compound
                let index_of_list_element = tapes.main.len();

                stack.push(ParsingStackElement::ListOfCompounds {
                    index_of_list_element: index_of_list_element as u32,
                })?;
                stack.set_list_length(length);
                tapes.main.push(TapeElement {
                    kind: (
                        TapeTagKind::CompoundList,
                        TapeTagValue {
                            // this gets overwritten after the list is fully read
                            compound_list: (length.into(), 0.into()),
                        },
                    ),
                });
            }
            INT_ARRAY_ID => {
                let index_of_element = tapes.extra.elements.len() as u32;
                tapes.main.push(TapeElement {
                    kind: (
                        TapeTagKind::IntArrayList,
                        TapeTagValue {
                            int_array_list: (0.into(), index_of_element.into()),
                        },
                    ),
                });
                let length = data.read_u32()?;
                tapes.extra.elements.push(ExtraTapeElement { length });
                for _ in 0..length {
                    let int_array = read_int_array(data)?;
                    tapes.extra.elements.push(ExtraTapeElement { int_array });
                }
            }
            LONG_ARRAY_ID => {
                let index_of_element = tapes.extra.elements.len() as u32;
                tapes.main.push(TapeElement {
                    kind: (
                        TapeTagKind::LongArrayList,
                        TapeTagValue {
                            long_array_list: (0.into(), index_of_element.into()),
                        },
                    ),
                });
                let length = data.read_u32()?;
                tapes.extra.elements.push(ExtraTapeElement { length });
                for _ in 0..length {
                    let long_array = read_long_array(data)?;
                    tapes.extra.elements.push(ExtraTapeElement { long_array });
                }
            }
            _ => return Err(NonRootError::unknown_tag_id(tag_type)),
        };
        Ok(())
    }

    pub fn write(&self, data: &mut Vec<u8>) {
        let (kind, _) = self.element();

        data.push(self.id());

        match kind {
            TapeTagKind::EmptyList => {
                data.extend(&0u32.to_be_bytes());
            }
            TapeTagKind::ByteList => {
                write_with_u32_length(data, 1, slice_i8_into_u8(self.bytes().unwrap()));
            }
            TapeTagKind::ShortList => {
                write_with_u32_length(
                    data,
                    2,
                    u32_prefixed_list_to_rawlist::<i16>(TapeTagKind::ShortList, self.element)
                        .unwrap()
                        .as_big_endian(),
                );
            }
            TapeTagKind::IntList => {
                write_with_u32_length(
                    data,
                    4,
                    u32_prefixed_list_to_rawlist::<i32>(TapeTagKind::IntList, self.element)
                        .unwrap()
                        .as_big_endian(),
                );
            }
            TapeTagKind::LongList => {
                write_with_u32_length(
                    data,
                    8,
                    u32_prefixed_list_to_rawlist::<i64>(TapeTagKind::LongList, self.element)
                        .unwrap()
                        .as_big_endian(),
                );
            }
            TapeTagKind::FloatList => {
                write_with_u32_length(
                    data,
                    4,
                    u32_prefixed_list_to_rawlist::<f32>(TapeTagKind::FloatList, self.element)
                        .unwrap()
                        .as_big_endian(),
                );
            }
            TapeTagKind::DoubleList => {
                write_with_u32_length(
                    data,
                    8,
                    u32_prefixed_list_to_rawlist::<f64>(TapeTagKind::DoubleList, self.element)
                        .unwrap()
                        .as_big_endian(),
                );
            }
            TapeTagKind::ByteArrayList => {
                let byte_arrays = self.byte_arrays().unwrap();
                for array in byte_arrays.iter() {
                    write_with_u32_length(data, 1, array);
                }
            }
            TapeTagKind::StringList => {
                let strings = self.strings().unwrap();
                for string in strings.iter() {
                    write_string(data, string);
                }
            }
            TapeTagKind::ListList => {
                let lists = self.lists().unwrap();
                for list in lists {
                    list.write(data);
                }
            }
            TapeTagKind::CompoundList => {
                let compounds = self.compounds().unwrap();
                write_u32(data, compounds.clone().len() as u32);
                for compound in compounds {
                    compound.write(data);
                }
            }
            TapeTagKind::IntArrayList => {
                let int_arrays = self.int_arrays().unwrap();
                for array in int_arrays.iter() {
                    write_with_u32_length(data, 4, array.as_big_endian());
                }
            }
            TapeTagKind::LongArrayList => {
                let long_arrays = self.long_arrays().unwrap();
                for array in long_arrays.iter() {
                    write_with_u32_length(data, 8, array.as_big_endian());
                }
            }
            _ => unreachable!(),
        }
    }

    /// Get the tape element kind and value for this list.
    fn element(&self) -> (TapeTagKind, TapeTagValue) {
        unsafe { (*self.element).kind }
    }

    /// Get the numerical ID of the tag type.
    #[inline]
    pub fn id(&self) -> u8 {
        match self.element().0 {
            TapeTagKind::EmptyList => END_ID,
            TapeTagKind::ByteList => BYTE_ID,
            TapeTagKind::ShortList => SHORT_ID,
            TapeTagKind::IntList => INT_ID,
            TapeTagKind::LongList => LONG_ID,
            TapeTagKind::FloatList => FLOAT_ID,
            TapeTagKind::DoubleList => DOUBLE_ID,
            TapeTagKind::ByteArrayList => BYTE_ARRAY_ID,
            TapeTagKind::StringList => STRING_ID,
            TapeTagKind::ListList => LIST_ID,
            TapeTagKind::CompoundList => COMPOUND_ID,
            TapeTagKind::IntArrayList => INT_ARRAY_ID,
            TapeTagKind::LongArrayList => LONG_ARRAY_ID,
            _ => unreachable!(),
        }
    }

    /// Returns whether the list is specifically a list with the `empty` tag type. This will return
    /// false if the list is any other type (even it has a length of zero).
    pub fn empty(&self) -> bool {
        self.element().0 == TapeTagKind::EmptyList
    }

    pub fn bytes(&self) -> Option<&[i8]> {
        let (kind, value) = self.element();
        if kind != TapeTagKind::ByteList {
            return None;
        }
        let length_ptr = u64::from(unsafe { value.byte_list }) as usize as *const UnalignedU32;
        let length = unsafe { u32::from(*length_ptr).swap_bytes() as usize };
        let byte_array =
            unsafe { std::slice::from_raw_parts(length_ptr.add(1) as *const i8, length) };
        Some(byte_array)
    }
    pub fn shorts(&self) -> Option<Vec<i16>> {
        u32_prefixed_list_to_vec(TapeTagKind::ShortList, self.element)
    }
    pub fn ints(&self) -> Option<Vec<i32>> {
        u32_prefixed_list_to_vec(TapeTagKind::IntList, self.element)
    }
    pub fn longs(&self) -> Option<Vec<i64>> {
        u32_prefixed_list_to_vec(TapeTagKind::LongList, self.element)
    }
    pub fn floats(&self) -> Option<Vec<f32>> {
        u32_prefixed_list_to_vec(TapeTagKind::FloatList, self.element)
    }
    pub fn doubles(&self) -> Option<Vec<f64>> {
        u32_prefixed_list_to_vec(TapeTagKind::DoubleList, self.element)
    }
    pub fn byte_arrays(&self) -> Option<&'a [&'a [u8]]> {
        let (kind, value) = self.element();
        if kind != TapeTagKind::ByteArrayList {
            return None;
        }
        let index_to_extra_tapes = u32::from(unsafe { value.byte_array_list.1 }) as usize;
        let length_ref = &self.extra_tapes.elements[index_to_extra_tapes];
        let length = unsafe { length_ref.length as usize };
        let slice = unsafe {
            std::slice::from_raw_parts(
                self.extra_tapes
                    .elements
                    .as_ptr()
                    .add(index_to_extra_tapes + 1)
                    .cast(),
                length,
            )
        };
        Some(slice)
    }
    pub fn strings(&self) -> Option<&'a [&'a Mutf8Str]> {
        let (kind, value) = self.element();
        if kind != TapeTagKind::StringList {
            return None;
        }
        let index_to_extra_tapes = u32::from(unsafe { value.string_list.1 }) as usize;
        let length_ref = &self.extra_tapes.elements[index_to_extra_tapes];
        let length = unsafe { length_ref.length as usize };
        let slice = unsafe {
            std::slice::from_raw_parts(
                self.extra_tapes
                    .elements
                    .as_ptr()
                    .add(index_to_extra_tapes + 1)
                    .cast(),
                length,
            )
        };
        Some(slice)
    }
    pub fn lists(&self) -> Option<ListList<'a, 'tape>> {
        let (kind, value) = self.element();
        if kind != TapeTagKind::ListList {
            return None;
        }

        let length = u32::from(unsafe { value.list_list.0 }) as usize;
        let max_tape_offset = u32::from(unsafe { value.list_list.1 }) as usize;

        Some(ListList {
            iter: ListListIter {
                current_tape_offset: 0, // it's an iterator, it starts at 0
                max_tape_offset,
                approx_length: length,
                tape: unsafe { self.element.add(1) }, // the first element is the listlist element so we don't include it
                extra_tapes: self.extra_tapes,
                _phantom: PhantomData,
            },
        })
    }

    pub fn compounds(&self) -> Option<CompoundList<'a, 'tape>> {
        let (kind, value) = self.element();
        if kind != TapeTagKind::CompoundList {
            return None;
        }

        let length = u32::from(unsafe { value.compound_list.0 }) as usize;

        let max_tape_offset = u32::from(unsafe { value.compound_list.1 }) as usize;
        let tape_slice =
            unsafe { std::slice::from_raw_parts(self.element.add(1), max_tape_offset) };

        Some(CompoundList {
            iter: CompoundListIter {
                current_tape_offset: 0,
                max_tape_offset,
                approx_length: length,
                tape: tape_slice,
                extra_tapes: self.extra_tapes,
            },
        })
    }
    pub fn int_arrays(&self) -> Option<&[RawList<i32>]> {
        let (kind, value) = self.element();
        if kind != TapeTagKind::IntArrayList {
            return None;
        }
        let index_to_extra_tapes = u32::from(unsafe { value.int_array_list.1 }) as usize;
        let length_ref = &self.extra_tapes.elements[index_to_extra_tapes];
        let length = unsafe { length_ref.length as usize };
        let slice = unsafe {
            std::slice::from_raw_parts(
                self.extra_tapes
                    .elements
                    .as_ptr()
                    .add(index_to_extra_tapes + 1)
                    .cast(),
                length,
            )
        };
        Some(slice)
    }
    pub fn long_arrays(&self) -> Option<&[RawList<i64>]> {
        let (kind, value) = self.element();
        if kind != TapeTagKind::LongArrayList {
            return None;
        }
        let index_to_extra_tapes = u32::from(unsafe { value.long_array_list.1 }) as usize;
        let length_ref = &self.extra_tapes.elements[index_to_extra_tapes];
        let length = unsafe { length_ref.length as usize };
        let slice = unsafe {
            std::slice::from_raw_parts(
                self.extra_tapes
                    .elements
                    .as_ptr()
                    .add(index_to_extra_tapes + 1)
                    .cast(),
                length,
            )
        };
        Some(slice)
    }

    pub fn to_owned(&self) -> crate::owned::NbtList {
        let (kind, _value) = self.element();

        match kind {
            TapeTagKind::EmptyList => crate::owned::NbtList::Empty,
            TapeTagKind::ByteList => crate::owned::NbtList::Byte(self.bytes().unwrap().to_vec()),
            TapeTagKind::ShortList => crate::owned::NbtList::Short(self.shorts().unwrap().to_vec()),
            TapeTagKind::IntList => crate::owned::NbtList::Int(self.ints().unwrap().to_vec()),
            TapeTagKind::LongList => crate::owned::NbtList::Long(self.longs().unwrap().to_vec()),
            TapeTagKind::FloatList => crate::owned::NbtList::Float(self.floats().unwrap().to_vec()),
            TapeTagKind::DoubleList => {
                crate::owned::NbtList::Double(self.doubles().unwrap().to_vec())
            }
            TapeTagKind::ByteArrayList => crate::owned::NbtList::ByteArray(
                self.byte_arrays()
                    .unwrap()
                    .iter()
                    .map(|array| array.to_vec())
                    .collect(),
            ),
            TapeTagKind::StringList => crate::owned::NbtList::String(
                self.strings()
                    .unwrap()
                    .iter()
                    .map(|&string| string.to_owned())
                    .collect(),
            ),
            TapeTagKind::ListList => crate::owned::NbtList::List(
                self.lists()
                    .unwrap()
                    .into_iter()
                    .map(|list| list.to_owned())
                    .collect(),
            ),
            TapeTagKind::CompoundList => crate::owned::NbtList::Compound(
                self.compounds()
                    .unwrap()
                    .into_iter()
                    .map(|compound| compound.to_owned())
                    .collect(),
            ),
            TapeTagKind::IntArrayList => crate::owned::NbtList::IntArray(
                self.int_arrays()
                    .unwrap()
                    .iter()
                    .map(|array| array.to_vec())
                    .collect::<Vec<_>>(),
            ),
            TapeTagKind::LongArrayList => crate::owned::NbtList::LongArray(
                self.long_arrays()
                    .unwrap()
                    .iter()
                    .map(|array| array.to_vec())
                    .collect::<Vec<_>>(),
            ),
            _ => unreachable!("this is an NbtList, no other kinds should be possible"),
        }
    }
}

impl PartialEq for NbtList<'_, '_> {
    fn eq(&self, other: &Self) -> bool {
        let (self_kind, _) = self.element();
        let (other_kind, _) = other.element();
        if self_kind != other_kind {
            return false;
        }
        match self_kind {
            TapeTagKind::EmptyList => true,
            TapeTagKind::ByteList => self.bytes().unwrap() == other.bytes().unwrap(),
            TapeTagKind::ShortList => self.shorts().unwrap() == other.shorts().unwrap(),
            TapeTagKind::IntList => self.ints().unwrap() == other.ints().unwrap(),
            TapeTagKind::LongList => self.longs().unwrap() == other.longs().unwrap(),
            TapeTagKind::FloatList => self.floats().unwrap() == other.floats().unwrap(),
            TapeTagKind::DoubleList => self.doubles().unwrap() == other.doubles().unwrap(),
            TapeTagKind::ByteArrayList => {
                self.byte_arrays().unwrap() == other.byte_arrays().unwrap()
            }
            TapeTagKind::StringList => self.strings().unwrap() == other.strings().unwrap(),
            TapeTagKind::ListList => self.lists().unwrap() == other.lists().unwrap(),
            TapeTagKind::CompoundList => self.compounds().unwrap() == other.compounds().unwrap(),
            TapeTagKind::IntArrayList => self.int_arrays().unwrap() == other.int_arrays().unwrap(),
            TapeTagKind::LongArrayList => {
                self.long_arrays().unwrap() == other.long_arrays().unwrap()
            }
            _ => unreachable!("this is an NbtList, no other kinds should be possible"),
        }
    }
}

/// A wrapper over [`ListListIter`] that acts more like a Vec.
#[derive(Clone, Default)]
pub struct ListList<'a, 'tape> {
    iter: ListListIter<'a, 'tape>,
}
impl<'a, 'tape> ListList<'a, 'tape> {
    /// Returns the number of tags directly in this list.
    ///
    /// Note that due to an internal optimization, this function runs at `O(n)`
    /// if the list has at least 2^24 items. Use [`Self::approx_len`] if you
    /// want to avoid that.
    pub fn len(self) -> usize {
        self.iter.len()
    }
    /// A version of [`Self::len`] that saturates at 2^24.
    pub fn approx_len(&self) -> usize {
        self.iter.approx_len()
    }
    /// Get the element at the given index. This is O(n) where n is index, so if you'll be calling
    /// this more than once you should probably just use the iterator.
    pub fn get(&self, index: usize) -> Option<NbtList<'a, 'tape>> {
        self.iter.clone().nth(index)
    }
    pub fn first(&self) -> Option<NbtList<'a, 'tape>> {
        self.iter.clone().next()
    }
    pub fn last(&self) -> Option<NbtList<'a, 'tape>> {
        self.iter.clone().last()
    }
}
impl<'a: 'tape, 'tape> IntoIterator for ListList<'a, 'tape> {
    type Item = NbtList<'a, 'tape>;
    type IntoIter = ListListIter<'a, 'tape>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter
    }
}
impl PartialEq for ListList<'_, '_> {
    fn eq(&self, other: &Self) -> bool {
        if self.iter.clone().approx_len() != other.iter.clone().approx_len() {
            return false;
        }
        if self.iter.clone().len() != other.iter.clone().len() {
            return false;
        }
        self.iter
            .clone()
            .zip(other.iter.clone())
            .all(|(a, b)| a == b)
    }
}
/// An iterator over a list of lists.
#[derive(Clone)]
pub struct ListListIter<'a, 'tape> {
    current_tape_offset: usize,
    max_tape_offset: usize,
    approx_length: usize,
    tape: *const TapeElement,
    extra_tapes: *const ExtraTapes<'a>,
    _phantom: PhantomData<&'tape ()>,
}
impl<'a: 'tape, 'tape> ListListIter<'a, 'tape> {
    /// Returns the number of tags directly in this list.
    ///
    /// Note that due to an internal optimization, this function runs at `O(n)`
    /// if the list has at least 2^24 items. Use [`Self::approx_len`] if you
    /// want to avoid that.
    pub fn len(self) -> usize {
        let len = self.approx_len();
        if len < 2usize.pow(24) {
            len
        } else {
            self.count()
        }
    }

    /// A version of [`Self::len`] that saturates at 2^24.
    pub fn approx_len(&self) -> usize {
        self.approx_length
    }
}
impl<'a: 'tape, 'tape> Iterator for ListListIter<'a, 'tape> {
    type Item = NbtList<'a, 'tape>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_tape_offset + 1 >= self.max_tape_offset {
            return None;
        }

        let element = unsafe { self.tape.add(self.current_tape_offset) };
        // println!("{:?}", unsafe { *element });
        let (kind, value) = unsafe { (*element).kind };
        debug_assert!(kind.is_list());

        let offset = if matches!(kind, TapeTagKind::CompoundList | TapeTagKind::ListList) {
            u32::from(unsafe { value.list_list.1 }) as usize
        } else {
            1
        };

        let nbt_list = NbtList {
            element,
            extra_tapes: unsafe { &*self.extra_tapes },
        };

        self.current_tape_offset += offset;
        Some(nbt_list)
    }
}
impl Default for ListListIter<'_, '_> {
    fn default() -> Self {
        ListListIter {
            current_tape_offset: 0,
            max_tape_offset: 0,
            approx_length: 0,
            tape: std::ptr::null(),
            // this won't ever get dereferenced because .next() will return immediately
            extra_tapes: std::ptr::null(),
            _phantom: PhantomData,
        }
    }
}

/// A wrapper over [`CompoundListIter`] that acts more like a Vec.
#[derive(Clone, Default)]
pub struct CompoundList<'a, 'tape> {
    iter: CompoundListIter<'a, 'tape>,
}
impl<'a, 'tape> CompoundList<'a, 'tape> {
    /// Returns the number of tags directly in this list.
    ///
    /// Note that due to an internal optimization, this function runs at `O(n)`
    /// if the list has at least 2^24 items. Use [`Self::approx_len`] if you
    /// want to avoid that.
    pub fn len(self) -> usize {
        self.iter.len()
    }
    /// A version of [`Self::len`] that saturates at 2^24.
    pub fn approx_len(&self) -> usize {
        self.iter.approx_len()
    }
    /// Get the element at the given index. This is `O(n)` where n is index, so
    /// if you'll be calling this more than once you should probably just use
    /// the iterator.
    pub fn get(&self, index: usize) -> Option<NbtCompound<'a, 'tape>> {
        self.iter.clone().nth(index)
    }
    pub fn first(&self) -> Option<NbtCompound<'a, 'tape>> {
        self.iter.clone().next()
    }
    pub fn last(&self) -> Option<NbtCompound<'a, 'tape>> {
        self.iter.clone().last()
    }
}
impl<'a: 'tape, 'tape> IntoIterator for CompoundList<'a, 'tape> {
    type Item = NbtCompound<'a, 'tape>;
    type IntoIter = CompoundListIter<'a, 'tape>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter
    }
}
impl PartialEq for CompoundList<'_, '_> {
    fn eq(&self, other: &Self) -> bool {
        if self.iter.clone().approx_len() != other.iter.clone().approx_len() {
            return false;
        }
        if self.iter.clone().len() != other.iter.clone().len() {
            return false;
        }
        self.iter
            .clone()
            .zip(other.iter.clone())
            .all(|(a, b)| a == b)
    }
}

#[derive(Clone)]
pub struct CompoundListIter<'a, 'tape> {
    current_tape_offset: usize,
    max_tape_offset: usize,
    approx_length: usize,
    tape: &'tape [TapeElement],
    extra_tapes: *const ExtraTapes<'a>,
}
impl<'a: 'tape, 'tape> CompoundListIter<'a, 'tape> {
    /// Returns the number of tags directly in this list.
    ///
    /// Note that due to an internal optimization, this function runs at `O(n)`
    /// if the list has at least 2^24 items. Use [`Self::approx_len`] if you
    /// want to avoid that.
    pub fn len(self) -> usize {
        let len = self.approx_len();
        if len < 2usize.pow(24) {
            len
        } else {
            self.count()
        }
    }

    /// A version of [`Self::len`] that saturates at 2^24.
    pub fn approx_len(&self) -> usize {
        self.approx_length
    }
}
impl<'a: 'tape, 'tape> Iterator for CompoundListIter<'a, 'tape> {
    type Item = NbtCompound<'a, 'tape>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_tape_offset + 1 >= self.max_tape_offset {
            return None;
        }

        let element = unsafe { self.tape.as_ptr().add(self.current_tape_offset) };
        let (kind, value) = unsafe { (*element).kind };
        debug_assert_eq!(kind, TapeTagKind::Compound);

        let offset = u32::from(unsafe { value.list_list.1 }) as usize;

        let compound = NbtCompound {
            element,
            extra_tapes: unsafe { &*self.extra_tapes },
        };

        self.current_tape_offset += offset;
        Some(compound)
    }
}
impl Default for CompoundListIter<'_, '_> {
    fn default() -> Self {
        CompoundListIter {
            current_tape_offset: 0,
            max_tape_offset: 0,
            approx_length: 0,
            tape: &[],
            // this won't ever get dereferenced because .next() will return immediately
            extra_tapes: std::ptr::null(),
        }
    }
}

pub(crate) fn u32_prefixed_list_to_rawlist<'a, T>(
    expected_kind: TapeTagKind,
    element: *const TapeElement,
) -> Option<RawList<'a, T>>
where
    T: Copy + SwappableNumber,
{
    let (kind, value) = unsafe { (*element).kind };
    if kind != expected_kind {
        return None;
    }

    unsafe { u32_prefixed_list_to_rawlist_unchecked(value) }
}

#[inline]
pub(crate) unsafe fn u32_prefixed_list_to_rawlist_unchecked<'a, T>(
    value: TapeTagValue,
) -> Option<RawList<'a, T>>
where
    T: Copy + SwappableNumber,
{
    // length is always a u32
    let length_ptr = u64::from(unsafe { value.int_list }) as usize as *const UnalignedU32;
    let length = unsafe { u32::from(*length_ptr).swap_bytes() as usize };
    let length_in_bytes = length * std::mem::size_of::<T>();
    let array_be =
        unsafe { std::slice::from_raw_parts(length_ptr.add(1) as *const u8, length_in_bytes) };
    Some(RawList::new(array_be))
}

pub(crate) fn u32_prefixed_list_to_vec<T>(
    expected_kind: TapeTagKind,
    element: *const TapeElement,
) -> Option<Vec<T>>
where
    T: Copy + SwappableNumber,
{
    u32_prefixed_list_to_rawlist(expected_kind, element).map(|rawlist| rawlist.to_vec())
}

#[inline]
pub fn read_list_in_list<'a>(
    data: &mut Reader<'a>,
    tapes: &mut Tapes<'a>,
    stack: &mut ParsingStack,
) -> Result<(), NonRootError> {
    let ParsingStackElement::ListOfLists {
        index_of_list_element,
    } = stack.peek()
    else {
        unsafe { unreachable_unchecked() };
    };

    let remaining = stack.remaining_elements_in_list();

    if remaining == 0 {
        stack.pop();

        let index_after_end_element = tapes.main.len();
        unsafe {
            tapes
                .main
                .get_unchecked_mut(index_of_list_element as usize)
                .kind
                .1
                .list_list
                .1 = (index_after_end_element as u32 - index_of_list_element).into();
        };
        return Ok(());
    }

    stack.decrement_list_length();

    NbtList::read(data, tapes, stack)
}

#[inline]
pub(crate) fn read_compound_in_list<'a>(
    data: &mut Reader<'a>,
    tapes: &mut Tapes<'a>,
    stack: &mut ParsingStack,
) -> Result<(), NonRootError> {
    let ParsingStackElement::ListOfCompounds {
        index_of_list_element,
    } = stack.peek()
    else {
        unsafe { unreachable_unchecked() };
    };

    let remaining = stack.remaining_elements_in_list();

    if remaining == 0 {
        stack.pop();

        let index_after_end_element = tapes.main.len();
        unsafe {
            tapes
                .main
                .get_unchecked_mut(index_of_list_element as usize)
                .kind
                .1
                .compound_list
                .1 = (index_after_end_element as u32 - index_of_list_element).into();
        };
        return Ok(());
    }

    stack.decrement_list_length();

    NbtCompound::read(data, tapes, stack)
}

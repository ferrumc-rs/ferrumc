use std::{marker::PhantomData, mem};

use crate::swap_endianness::{swap_endianness, swap_endianness_as_u8, SwappableNumber};

/// A list of numbers that's kept as big-endian in memory.

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct RawList<'a, T> {
    data: &'a [u8],
    _marker: PhantomData<T>,
}
impl<'a, T> RawList<'a, T> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            _marker: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len() / mem::size_of::<T>()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn into_inner(self) -> &'a [u8] {
        self.data
    }
}

impl<T: SwappableNumber> RawList<'_, T> {
    pub fn to_vec(&self) -> Vec<T>
    where
        T: Copy + SwappableNumber,
    {
        swap_endianness(self.data)
    }

    pub fn to_little_endian(&self) -> Vec<u8> {
        swap_endianness_as_u8::<T>(self.data)
    }

    pub fn as_big_endian(&self) -> &[u8] {
        self.data
    }
}

impl<T> IntoIterator for RawList<'_, T>
where
    T: Copy + SwappableNumber,
{
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.to_vec().into_iter()
    }
}
impl<T> IntoIterator for &RawList<'_, T>
where
    T: Copy + SwappableNumber,
{
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.to_vec().into_iter()
    }
}

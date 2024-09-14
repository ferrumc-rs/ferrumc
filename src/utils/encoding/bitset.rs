/*#![allow(dead_code)]

use ferrumc_codec::network_types::varint::VarInt;
use crate::utils::error::Error;
use ferrumc_codec::enc::Encode;
use tokio::io::{AsyncSeek, AsyncWrite};

pub struct BitSet {
    len: VarInt,
    data: Vec<u64>,
}

impl BitSet {
    pub(crate) fn new() -> Self {
        BitSet {
            data: vec![0; 1],
            len: VarInt::from(0),
        }
    }

    pub(crate) fn set(&mut self, index: u32) {
        let word_index = (index / 64) as usize;
        let bit_index = index % 64;
        if word_index >= self.data.len() {
            self.data.resize(word_index + 1, 0);
        }
        self.data[word_index] |= 1 << bit_index;
    }

    fn to_vec(&self) -> Vec<i64> {
        self.data.iter().map(|&x| x as i64).collect()
    }
}

impl Encode for BitSet {
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), Error>
    where
        T: AsyncWrite + Unpin,
    {
        self.len.encode(bytes).await?;
        self.data.encode(bytes).await?;
        Ok(())
    }
}
*/

use std::ops::Index;

use ferrumc_codec::enc::NetEncode;
use ferrumc_codec::network_types::varint::VarInt;
use tokio::io::{AsyncWrite, AsyncWriteExt};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BitSet {
    data: Vec<u64>,
    size: usize,
}

impl BitSet {
    pub fn new(size: usize) -> Self {
        let num_blocks = (size + 63) / 64;
        BitSet {
            data: vec![0; num_blocks],
            size,
        }
    }

    pub fn empty() -> Self {
        BitSet {
            data: Vec::new(),
            size: 0,
        }
    }

    pub fn set(&mut self, index: usize) {
        if index < self.size {
            let block = index / 64;
            let bit = index % 64;
            self.data[block] |= 1 << bit;
        }
    }

    pub fn clear(&mut self, index: usize) {
        if index < self.size {
            let block = index / 64;
            let bit = index % 64;
            self.data[block] &= !(1 << bit);
        }
    }

    pub fn get(&self, index: usize) -> bool {
        if index < self.size {
            let block = index / 64;
            let bit = index % 64;
            (self.data[block] & (1 << bit)) != 0
        } else {
            false
        }
    }

    pub fn toggle(&mut self, index: usize) {
        if index < self.size {
            let block = index / 64;
            let bit = index % 64;
            self.data[block] ^= 1 << bit;
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn count_ones(&self) -> usize {
        self.data
            .iter()
            .map(|&block| block.count_ones() as usize)
            .sum()
    }

    pub fn clear_all(&mut self) {
        self.data.fill(0);
    }

    pub fn set_all(&mut self) {
        self.data.fill(u64::MAX);
        // Clear any bits beyond the set size
        if self.size % 64 != 0 {
            let last_block = self.data.last_mut().unwrap();
            *last_block &= (1 << (self.size % 64)) - 1;
        }
    }
}

impl FromIterator<usize> for BitSet {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        let mut bs = BitSet::empty();
        for i in iter {
            bs.set(i);
        }
        bs
    }
}

impl Index<usize> for BitSet {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        if self.get(index) {
            &true
        } else {
            &false
        }
    }
}

impl NetEncode for BitSet {
    async fn net_encode<T>(&self, bytes: &mut T) -> Result<(), ferrumc_codec::CodecError>
    where
        T: AsyncWrite + Unpin,
    {
        // Bit sets of type BitSet are prefixed by their length in longs.
        // Field Name 	Field Type 	Meaning
        // Length 	VarInt 	Number of longs in the following array. May be 0 (if no bits are set).
        // Data 	Array of Long 	A packed representation of the bit set as created by BitSet.toLongArray.
        let len = VarInt::from(self.data.len() as i32);
        len.net_encode(bytes).await?;
        for &word in &self.data {
            let word = word.to_be_bytes();
            bytes.write_all(&word).await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitset_operations() {
        let mut bs = BitSet::new(100);

        assert_eq!(bs.len(), 100);
        assert!(!bs.is_empty());

        bs.set(50);
        assert!(bs.get(50));
        assert!(!bs.get(51));

        bs.toggle(51);
        assert!(bs.get(51));

        bs.clear(50);
        assert!(!bs.get(50));

        assert_eq!(bs.count_ones(), 1);

        bs.set_all();
        assert_eq!(bs.count_ones(), 100);

        bs.clear_all();
        assert_eq!(bs.count_ones(), 0);
    }

    #[test]
    fn test_bitset_indexing() {
        let mut bs = BitSet::new(100);

        bs.set(50);
        assert!(bs[50]);
        assert!(!bs[51]);
    }
}

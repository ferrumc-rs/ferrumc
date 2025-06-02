use crate::encode::{NetEncode, NetEncodeOpts, NetEncodeResult};
use crate::net_types::var_int::VarInt;
use std::io::Write;
use std::ops::Not;
use tokio::io::AsyncWrite;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Clone)]
pub struct BitSet(Vec<u64>);

impl BitSet {
    pub fn new(size: usize) -> Self {
        let num_blocks = size.div_ceil(64);
        Self(vec![0; num_blocks])
    }

    pub fn set(&mut self, index: usize, is_set: bool) {
        let word_index = index / 64;
        let bit_index = index % 64;
        if word_index >= self.0.len() {
            self.0.resize(word_index + 1, 0);
        }
        if is_set {
            self.0[word_index] |= 1 << bit_index;
        } else {
            self.0[word_index] &= !(1 << bit_index);
        }
    }

    pub fn get(&self, index: usize) -> bool {
        let word_index = index / 64;
        let bit_index = index % 64;
        if word_index >= self.0.len() {
            return false;
        }
        self.0[word_index] & (1 << bit_index) != 0
    }

    pub fn flip(&mut self, index: usize) {
        let word_index = index / 64;
        let bit_index = index % 64;
        if word_index >= self.0.len() {
            self.0.resize(word_index + 1, 0);
        }
        self.0[word_index] ^= 1 << bit_index;
    }

    pub fn set_all(&mut self, is_set: bool) {
        let value = if is_set { 0xFFFFFFFFFFFFFFFF } else { 0 };
        for val in &mut self.0 {
            *val = value;
        }
    }
}

impl NetEncode for BitSet {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> NetEncodeResult<()> {
        VarInt::from(self.0.len()).encode(writer, opts)?;
        writer.write_all(&ferrumc_general_purpose::simd::arrays::u64_slice_to_u8_be(
            &self.0,
        ))?;
        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> NetEncodeResult<()> {
        VarInt::from(self.0.len())
            .encode_async(writer, opts)
            .await?;
        writer
            .write_all(&ferrumc_general_purpose::simd::arrays::u64_slice_to_u8_be(
                &self.0,
            ))
            .await?;
        Ok(())
    }
}

impl Not for BitSet {
    type Output = Self;

    fn not(mut self) -> Self::Output {
        for val in &mut self.0 {
            *val = !*val;
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let bitset = BitSet::new(0);
        assert!(bitset.0.is_empty());
    }

    #[test]
    fn test_set_and_get() {
        let mut bitset = BitSet::new(0);
        bitset.set(10, true);
        assert!(bitset.get(10));
        assert!(!bitset.get(9));
        bitset.set(10, false);
        assert!(!bitset.get(10));
    }

    #[test]
    fn test_flip() {
        let mut bitset = BitSet::new(0);
        bitset.flip(5);
        assert!(bitset.get(5));
        bitset.flip(5);
        assert!(!bitset.get(5));
    }

    #[test]
    fn test_resize() {
        let mut bitset = BitSet::new(0);
        bitset.set(128, true);
        assert!(bitset.get(128));
        assert!(!bitset.get(127));
    }

    #[test]
    fn test_set_all() {
        // Test setting all bits to true
        let mut bitset = BitSet::new(128); // Create a bitset of size 128
        bitset.set_all(true); // Set all bits to true
        for i in 0..128 {
            assert!(bitset.get(i), "Bit at index {i} should be set to true");
        }

        // Test setting all bits to false
        bitset.set_all(false); // Set all bits to false
        for i in 0..128 {
            assert!(!bitset.get(i), "Bit at index {i} should be set to false");
        }
    }

    #[test]
    fn test_not_trait() {
        let mut bitset = BitSet::new(128); // Create a bitset of size 128

        // Set some bits to true
        bitset.set(5, true);
        bitset.set(10, true);
        bitset.set(20, true);

        // Apply the NOT operation, which should invert the bits
        let inverted = !bitset;

        // Check that the bits that were set to true are now false and vice versa
        assert!(
            !inverted.get(5),
            "Bit at index 5 should be false after inversion"
        );
        assert!(
            !inverted.get(10),
            "Bit at index 10 should be false after inversion"
        );
        assert!(
            !inverted.get(20),
            "Bit at index 20 should be false after inversion"
        );

        // Check that all other bits are now true
        for i in 0..128 {
            if i != 5 && i != 10 && i != 20 {
                assert!(
                    inverted.get(i),
                    "Bit at index {i} should be true after inversion"
                );
            }
        }
    }
}

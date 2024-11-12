use std::io::Write;
use tokio::io::{AsyncWrite, AsyncWriteExt};
use crate::encode::{NetEncode, NetEncodeOpts, NetEncodeResult};
use crate::net_types::var_int::VarInt;


pub struct BitSet(Vec<u64>);

impl BitSet {
    pub fn new() -> Self {
        Self(Vec::new())
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
}

impl NetEncode for BitSet {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> NetEncodeResult<()> {
        VarInt::from(self.0.len()).encode(writer, opts)?;
        for val in &self.0 {
            writer.write_all(&val.to_be_bytes())?;
        }
        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(&self, writer: &mut W, opts: &NetEncodeOpts) -> NetEncodeResult<()> {
        VarInt::from(self.0.len() as i32).encode_async(writer, opts).await?;
        for val in &self.0 {
            writer.write_all(&val.to_be_bytes()).await?;
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let bitset = BitSet::new();
        assert!(bitset.0.is_empty());
    }

    #[test]
    fn test_set_and_get() {
        let mut bitset = BitSet::new();
        bitset.set(10, true);
        assert!(bitset.get(10));
        assert!(!bitset.get(9));
        bitset.set(10, false);
        assert!(!bitset.get(10));
    }

    #[test]
    fn test_flip() {
        let mut bitset = BitSet::new();
        bitset.flip(5);
        assert!(bitset.get(5));
        bitset.flip(5);
        assert!(!bitset.get(5));
    }

    #[test]
    fn test_resize() {
        let mut bitset = BitSet::new();
        bitset.set(128, true);
        assert!(bitset.get(128));
        assert!(!bitset.get(127));
    }
}

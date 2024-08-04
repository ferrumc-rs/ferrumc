#![allow(dead_code)]

use tokio::io::{AsyncSeek, AsyncWrite};
use crate::utils::encoding::varint::VarInt;
use crate::utils::error::Error;
use crate::utils::type_impls::Encode;

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
        T: AsyncWrite + AsyncSeek + Unpin
    {
        self.len.encode(bytes).await?;
        self.data.encode(bytes).await?;
        Ok(())
    }
}
use crate::chunk::section::paletted::PalettedSection;
use crate::chunk::section::uniform::UniformSection;
use crate::chunk::section::{AIR, CHUNK_SECTION_LENGTH};
use crate::chunk::BlockStateId;
use bitcode_derive::{Decode, Encode};
use deepsize::DeepSizeOf;

#[derive(Clone, DeepSizeOf, Encode, Decode)]
pub struct DirectSection(pub(crate) Box<[BlockStateId]>, u16);

impl Default for DirectSection {
    fn default() -> Self {
        Self(vec![AIR; CHUNK_SECTION_LENGTH].into_boxed_slice(), 0)
    }
}

impl DirectSection {
    #[inline]
    pub fn set_block(&mut self, idx: usize, block: BlockStateId) {
        if self.0[idx] == AIR && block != AIR {
            self.1 += 1
        } else if self.0[idx] != AIR && block == AIR {
            self.1 -= 1
        }

        self.0[idx] = block;
    }

    #[inline]
    pub fn get_block(&self, idx: usize) -> BlockStateId {
        self.0[idx]
    }

    pub fn block_count(&self) -> u16 {
        self.1
    }
}

impl From<&mut UniformSection> for DirectSection {
    fn from(s: &mut UniformSection) -> Self {
        Self(
            vec![s.get_block(); CHUNK_SECTION_LENGTH].into_boxed_slice(),
            if s.get_block() == AIR { 0 } else { 4096 },
        )
    }
}

impl From<&mut PalettedSection> for DirectSection {
    fn from(s: &mut PalettedSection) -> Self {
        let mut vec = vec![AIR; CHUNK_SECTION_LENGTH];
        let mut count = 0;

        for (block_idx, val) in vec.iter_mut().enumerate() {
            let block = s.get_block(block_idx);
            *val = s.get_block(block_idx);

            if block != AIR {
                count += 1
            }
        }

        Self(vec.into_boxed_slice(), count)
    }
}

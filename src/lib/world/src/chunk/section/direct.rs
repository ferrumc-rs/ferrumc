use crate::chunk::section::paletted::PalettedSection;
use crate::chunk::section::uniform::UniformSection;
use crate::chunk::section::{AIR, CHUNK_SECTION_LENGTH};
use crate::chunk::BlockStateId;
use bitcode_derive::{Decode, Encode};
use deepsize::DeepSizeOf;

// Currently there are less block state ids than u16::MAX, so we can store ids as u16s to cut down on memory usage
type CompactBlockStateId = u16;

const AIR_COMPACT: CompactBlockStateId = AIR.raw() as CompactBlockStateId;

#[derive(Clone, DeepSizeOf, Encode, Decode)]
pub struct DirectSection(pub(crate) Box<[CompactBlockStateId]>, u16);

impl Default for DirectSection {
    fn default() -> Self {
        Self(
            vec![AIR_COMPACT; CHUNK_SECTION_LENGTH].into_boxed_slice(),
            0,
        )
    }
}

impl DirectSection {
    #[inline]
    pub fn set_block(&mut self, idx: usize, block: BlockStateId) {
        if self.0[idx] == AIR_COMPACT && block != AIR {
            self.1 += 1
        } else if self.0[idx] != AIR_COMPACT && block == AIR {
            self.1 -= 1
        }

        self.0[idx] = block.raw() as CompactBlockStateId;
    }

    #[inline]
    pub fn get_block(&self, idx: usize) -> BlockStateId {
        BlockStateId::new(self.0[idx] as _)
    }

    pub fn block_count(&self) -> u16 {
        self.1
    }
}

impl From<&mut UniformSection> for DirectSection {
    fn from(s: &mut UniformSection) -> Self {
        Self(
            vec![s.get_block().raw() as CompactBlockStateId; CHUNK_SECTION_LENGTH]
                .into_boxed_slice(),
            if s.get_block() == AIR { 0 } else { 4096 },
        )
    }
}

impl From<&mut PalettedSection> for DirectSection {
    fn from(s: &mut PalettedSection) -> Self {
        let mut vec = vec![AIR_COMPACT; CHUNK_SECTION_LENGTH];
        let mut count = 0;

        for (block_idx, val) in vec.iter_mut().enumerate() {
            let block = s.get_block(block_idx);
            *val = s.get_block(block_idx).raw() as CompactBlockStateId;

            if block != AIR {
                count += 1
            }
        }

        Self(vec.into_boxed_slice(), count)
    }
}

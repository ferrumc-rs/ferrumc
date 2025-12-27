use crate::chunk::BlockStateId;
use crate::chunk::section::CHUNK_SECTION_LENGTH;
use crate::chunk::section::paletted::PalettedSection;
use crate::chunk::section::uniform::UniformSection;

pub struct DirectSection(Box<[BlockStateId]>);

impl DirectSection {
    pub fn new() -> Self {
        Self(vec![0; CHUNK_SECTION_LENGTH].into_boxed_slice())
    }

    #[inline]
    pub fn set_block(&mut self, idx: usize, block: BlockStateId) {
        self.0[idx] = block;
    }

    #[inline]
    pub fn get_block(&self, idx: usize) -> BlockStateId {
        self.0[idx]
    }
}

impl From<&mut UniformSection> for DirectSection {
    fn from(s: &mut UniformSection) -> Self {
        Self(vec![s.get_block(); CHUNK_SECTION_LENGTH].into_boxed_slice())
    }
}

impl From<&mut PalettedSection> for DirectSection {
    fn from(s: &mut PalettedSection) -> Self {
        let mut vec = vec![0; CHUNK_SECTION_LENGTH];

        for block_idx in 0..CHUNK_SECTION_LENGTH {
            vec[block_idx] = s.get_block(block_idx);
        }

        Self(vec.into_boxed_slice())
    }
}
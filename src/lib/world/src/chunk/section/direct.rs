use deepsize::DeepSizeOf;
use crate::chunk::BlockStateId;
use crate::chunk::section::CHUNK_SECTION_LENGTH;
use crate::chunk::section::paletted::PalettedSection;
use crate::chunk::section::uniform::UniformSection;

#[derive(Clone, DeepSizeOf)]
pub struct DirectSection(pub(crate) Box<[BlockStateId]>, u16);

impl DirectSection {
    pub fn new() -> Self {
        Self(vec![0; CHUNK_SECTION_LENGTH].into_boxed_slice(), 0)
    }

    #[inline]
    pub fn set_block(&mut self, idx: usize, block: BlockStateId) {
        if self.0[idx] == 0 && block != 0 { self.1 += 1 }
        else if self.0[idx] != 0 && block == 0 { self.1 -= 1 }

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
        Self(vec![s.get_block(); CHUNK_SECTION_LENGTH].into_boxed_slice(), if s.get_block() == 0 { 0 } else { 4096 })
    }
}

impl From<&mut PalettedSection> for DirectSection {
    fn from(s: &mut PalettedSection) -> Self {
        let mut vec = vec![0; CHUNK_SECTION_LENGTH];
        let mut count = 0;

        for block_idx in 0..CHUNK_SECTION_LENGTH {
            let block = s.get_block(block_idx);
            vec[block_idx] = s.get_block(block_idx);

            if block != 0 { count += 1 }
        }

        Self(vec.into_boxed_slice(), count)
    }
}
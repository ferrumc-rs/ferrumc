use crate::chunk::BlockStateId;
use crate::chunk::palette::BlockPalette;
use crate::chunk::section::CHUNK_SECTION_LENGTH;
use crate::chunk::section::uniform::UniformSection;

pub struct PalettedSection {
    palette: BlockPalette,
    block_data: Box<[u8]>,
    bit_width: u8,
}

impl PalettedSection {
    pub fn new() -> Self {
        Self {
            palette: BlockPalette::new(),
            block_data: vec![0; CHUNK_SECTION_LENGTH / 8].into_boxed_slice(),
            bit_width: 1,
        }
    }

    pub fn get_block(&self, idx: usize) -> BlockStateId {
        todo!()
    }

    pub fn set_block(&mut self, idx: usize, state: BlockStateId) {
        todo!()
    }

    fn resize(&mut self, new_bit_width: u8) {
        todo!()
    }
}

impl From<UniformSection> for PalettedSection {
    fn from(s: UniformSection) -> Self {
        let mut palette = BlockPalette::new();
        let _ = palette.add_block(s.get_block());

        Self { palette, block_data: vec![u8::MAX; CHUNK_SECTION_LENGTH / 8].into_boxed_slice(), bit_width: 1 }
    }
}
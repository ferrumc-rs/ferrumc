use crate::chunk::BlockStateId;
use crate::chunk::palette::{BlockPalette, PaletteIndex};
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

    pub fn new_with_block_count(block_count: u8) -> Self {
        let bit_width = BlockPalette::bit_width_for_len(block_count as _);

        Self {
            palette: BlockPalette::new_with_entry_count(block_count as _),
            block_data: vec![0; CHUNK_SECTION_LENGTH / (8 / bit_width as usize)].into_boxed_slice(),
            bit_width,
        }
    }

    pub fn get_block(&self, idx: usize) -> BlockStateId {
        let idx = Self::unpack_value(&self.block_data, idx, self.bit_width);

        self.palette.translate_idx(idx as PaletteIndex).expect("idx should be within the palette")
    }

    pub fn set_block(&mut self, idx: usize, state: BlockStateId) -> Option<()> {
        let prev_idx = Self::unpack_value(&self.block_data, idx, self.bit_width);
        self.palette.remove_block(prev_idx as PaletteIndex);

        let new_idx = self.palette.add_block(state);
        if let Some(new_bit_width) = new_idx.new_bit_width() {
            if new_bit_width > 8 {
                return None;
            }
            self.resize(new_bit_width);
        }

        Self::pack_value(&mut self.block_data, idx, self.bit_width, new_idx.unwrap() as u8);
        Some(())
    }

    fn resize(&mut self, new_bit_width: u8) {
        todo!()
    }

    fn pack_value(buffer: &mut [u8], idx: usize, bit_width: u8, value: u8) {
        debug_assert!(bit_width.is_power_of_two());
        debug_assert!(bit_width <= 8);
        debug_assert!(value < (1 << bit_width));

        let entries_per_byte = 8 / bit_width as usize;
        let byte_idx = idx / entries_per_byte;
        let bit_offset = (idx % entries_per_byte) * bit_width as usize;

        let mask = ((1 << bit_width) - 1) << bit_offset;
        buffer[byte_idx] &= !mask;
        buffer[byte_idx] |= value << bit_offset;
    }

    fn unpack_value(buffer: &[u8], idx: usize, bit_width: u8) -> u8 {
        debug_assert!(bit_width.is_power_of_two());
        debug_assert!(bit_width <= 8);

        let entries_per_byte = 8 / bit_width as usize;
        let byte_idx = idx / entries_per_byte;
        let bit_offset = (idx % entries_per_byte) * bit_width as usize;

        (buffer[byte_idx] >> bit_offset) & ((1 << bit_width) - 1)
    }
}

impl From<UniformSection> for PalettedSection {
    fn from(s: UniformSection) -> Self {
        let mut palette = BlockPalette::new();
        let _ = palette.add_block(s.get_block());

        Self { palette, block_data: vec![u8::MAX; CHUNK_SECTION_LENGTH / 8].into_boxed_slice(), bit_width: 1 }
    }
}
use crate::chunk::palette::{BlockPalette, PaletteIndex};
use crate::chunk::section::uniform::UniformSection;
use crate::chunk::section::{AIR, CHUNK_SECTION_LENGTH};
use crate::chunk::BlockStateId;
use bitcode_derive::{Decode, Encode};
use deepsize::DeepSizeOf;
use std::num::NonZeroU16;

#[derive(Clone, DeepSizeOf, Encode, Decode)]
pub struct PalettedSection {
    pub(crate) palette: BlockPalette,
    pub(crate) block_data: Box<[u64]>,
    pub(crate) bit_width: u8,
}

impl Default for PalettedSection {
    fn default() -> Self {
        Self {
            palette: BlockPalette::new(),
            block_data: vec![0u64; (CHUNK_SECTION_LENGTH / 8) / size_of::<u64>()]
                .into_boxed_slice(),
            bit_width: 1,
        }
    }
}

impl PalettedSection {
    pub fn new_with_block_count(block_count: u8) -> Self {
        let bit_width = BlockPalette::bit_width_for_len(block_count as _);

        Self {
            palette: BlockPalette::new_with_entry_count(block_count as _),
            block_data: vec![
                0u64;
                (CHUNK_SECTION_LENGTH / (8 / bit_width as usize)) / size_of::<u64>()
            ]
            .into_boxed_slice(),
            bit_width,
        }
    }

    pub fn get_block(&self, idx: usize) -> BlockStateId {
        let idx = Self::unpack_value(&self.block_data, idx, self.bit_width);

        self.palette
            .translate_idx(idx as PaletteIndex)
            .expect("idx should be within the palette")
    }

    pub fn set_block(&mut self, idx: usize, state: BlockStateId) -> Option<()> {
        let prev_idx = Self::unpack_value(&self.block_data, idx, self.bit_width);
        self.palette.remove_block(prev_idx as PaletteIndex);

        let (new_idx, new_bit_width) = self.palette.add_block(state);
        if let Some(new_bit_width) = new_bit_width {
            if new_bit_width > 8 {
                return None;
            }

            self.resize(new_bit_width);
            self.bit_width = new_bit_width;
        }

        Self::pack_value(&mut self.block_data, idx, self.bit_width, new_idx as u8);
        Some(())
    }

    pub fn block_count(&self) -> u16 {
        self.palette.block_count()
    }

    fn resize(&mut self, new_bit_width: u8) {
        let mut new_buffer =
            vec![0u64; (CHUNK_SECTION_LENGTH / (8 / new_bit_width as usize)) / size_of::<u64>()]
                .into_boxed_slice();

        for block_idx in 0..CHUNK_SECTION_LENGTH {
            let id = Self::unpack_value(&self.block_data, block_idx, self.bit_width);
            Self::pack_value(&mut new_buffer, block_idx, new_bit_width, id);
        }

        self.bit_width = new_bit_width;
        self.block_data = new_buffer;
    }

    #[inline]
    pub(crate) fn pack_value(buffer: &mut [u64], idx: usize, bit_width: u8, value: u8) {
        debug_assert!(bit_width.is_power_of_two());
        debug_assert!(bit_width <= 8);
        debug_assert!((value as u16) < (1u16 << bit_width), "value < (1 << bit_width) failed: bit_width {}, value {}", bit_width, value);

        let bit_width = bit_width as usize;

        let entries_per_long = u64::BITS as usize / bit_width;
        let entry_mask = ((1u64 << bit_width) - 1) as usize;
        let long_index = idx / entries_per_long;
        let bit_idx = idx % entries_per_long * bit_width;

        buffer[long_index] &= !(entry_mask << bit_idx) as u64;
        buffer[long_index] |= (value as u64) << bit_idx;
    }

    #[inline]
    pub(crate) fn unpack_value(buffer: &[u64], idx: usize, bit_width: u8) -> u8 {
        debug_assert!(bit_width.is_power_of_two());
        debug_assert!(bit_width <= 8);

        let bit_width = bit_width as usize;

        let entries_per_long = u64::BITS as usize / bit_width;
        let entry_mask = ((1u64 << bit_width) - 1) as usize;
        let long_index = idx / entries_per_long;
        let bit_idx = idx % entries_per_long * bit_width;

        ((buffer[long_index] >> bit_idx) & entry_mask as u64) as u8
    }

    #[inline]
    pub(crate) fn unpack_value_unaligned(buffer: &[u64], idx: usize, bit_width: u8) -> u8 {
        assert!(bit_width <= 8, "should be using direct sampling");

        let bit_width = bit_width as usize;

        let entries_per_long = u64::BITS as usize / bit_width;
        let entry_mask = ((1u64 << bit_width) - 1) as usize;
        let long_index = idx / entries_per_long;
        let bit_idx = idx % entries_per_long * bit_width;

        ((buffer[long_index] >> bit_idx) & entry_mask as u64) as u8
    }
}

impl From<&mut UniformSection> for PalettedSection {
    fn from(s: &mut UniformSection) -> Self {
        if s.get_block() != AIR {
            let mut palette = BlockPalette::new();
            let _ = palette.add_block_with_count(
                s.get_block(),
                NonZeroU16::new(CHUNK_SECTION_LENGTH as _)
                    .expect("CHUNK_SECTION_LENGTH should not be 0"),
            );

            Self {
                palette,
                block_data: vec![u64::MAX; (CHUNK_SECTION_LENGTH / 8) / size_of::<u64>()]
                    .into_boxed_slice(),
                bit_width: 1,
            }
        } else {
            Self::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::chunk::section::paletted::PalettedSection;

    #[test]
    fn test_pack_unpack() {
        let mut arr = [0, 0, 0, 0];
        let bit_width = 4;

        PalettedSection::pack_value(&mut arr, 0, bit_width, 1);
        PalettedSection::pack_value(&mut arr, 5, bit_width, 15);

        assert_eq!(PalettedSection::unpack_value(&arr, 0, bit_width), 1);
        assert_eq!(PalettedSection::unpack_value(&arr, 5, bit_width), 15);
    }
}

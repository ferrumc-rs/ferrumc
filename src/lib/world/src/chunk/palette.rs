use crate::chunk::section::AIR;
use crate::chunk::BlockStateId;
use bitcode_derive::{Decode, Encode};
use deepsize::DeepSizeOf;
use std::num::NonZeroU16;

pub type PaletteIndex = u16;

const NON_ZERO_ONE: NonZeroU16 = match NonZeroU16::new(1) {
    Some(v) => v,
    None => unreachable!(),
};

#[derive(Clone, DeepSizeOf, Encode, Decode)]
pub struct BlockPalette {
    pub(crate) palette: Vec<Option<(BlockStateId, NonZeroU16)>>,
    pub(crate) free_count: u16,
}

impl BlockPalette {
    pub fn new() -> BlockPalette {
        BlockPalette {
            palette: vec![Some((AIR, NonZeroU16::MAX))],
            free_count: 0,
        }
    }

    pub fn new_with_entry_count(entries: usize) -> BlockPalette {
        let mut palette = vec![Some((AIR, NonZeroU16::MAX))];
        palette.resize_with(entries + 1, || None);

        BlockPalette {
            palette,
            free_count: entries as u16,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.palette.len()
    }

    pub fn palette_data(&self) -> Vec<BlockStateId> {
        self.palette
            .iter()
            .map(|val| val.unwrap_or((AIR, NonZeroU16::MAX)).0)
            .collect::<Vec<_>>()
    }

    pub fn translate_idx(&self, idx: PaletteIndex) -> Option<BlockStateId> {
        let idx = idx as usize;

        if self.palette.len() <= idx {
            None
        } else {
            self.palette[idx].as_ref().map(|(s, _)| *s)
        }
    }

    pub fn add_block(&mut self, id: BlockStateId) -> (PaletteIndex, Option<u8>) {
        if id == AIR {
            return (0, None);
        } // Air is always palette idx 0

        for (idx, val) in self.palette.iter_mut().enumerate() {
            if let Some((block_id, count)) = val {
                if *block_id == id {
                    *count = NonZeroU16::new(
                        count
                            .get()
                            .checked_add(1)
                            .expect("count should never exceed 4096"),
                    )
                    .expect("addition should not overflow");
                    return (idx as PaletteIndex, None);
                }
            }
        }

        if self.free_count == 0 {
            if self.palette.len() >= 4096 {
                panic!("Palette size should not be growing past 4096 because 4096 is the number of blocks per section")
            }

            let curr_bit_width = Self::bit_width_for_len(self.palette.len());
            let new_bit_width = Self::bit_width_for_len(self.palette.len() + 1);

            self.palette.push(Some((id, NON_ZERO_ONE)));
            let idx = (self.palette.len() - 1) as PaletteIndex;

            if curr_bit_width != new_bit_width {
                (idx, Some(new_bit_width))
            } else {
                (idx, None)
            }
        } else {
            let Some((idx, empty_entry)) = self
                .palette
                .iter_mut()
                .enumerate()
                .find(|(_, val)| val.is_none())
            else {
                panic!("palette should contain empty entry if free_count != 0");
            };

            let _ = empty_entry.insert((id, NON_ZERO_ONE));
            self.free_count -= 1;
            (idx as PaletteIndex, None)
        }
    }

    pub fn add_block_with_count(
        &mut self,
        id: BlockStateId,
        count: NonZeroU16,
    ) -> (PaletteIndex, Option<u8>) {
        let res = self.add_block(id);

        self.palette[res.0 as usize] = Some((id, count));

        res
    }

    pub fn remove_block(&mut self, idx: PaletteIndex) {
        if idx == 0 {
            return;
        } // Air is always ignored

        let idx = idx as usize;

        debug_assert!(
            self.palette.len() > idx,
            "Palette index {} out of bounds for palette length {}",
            idx,
            self.palette.len(),
        );

        debug_assert!(
            self.palette[idx].is_some(),
            "Palette does not contain an entry for idx {}",
            idx,
        );

        let (_, count) = &mut self.palette[idx].unwrap();

        if count.get() == 1 {
            self.palette[idx] = None;
            self.free_count += 1;
        } else {
            *count = NonZeroU16::new(count.get() - 1).expect("count should be greater than 1");
        }
    }

    pub fn block_count(&self) -> u16 {
        self.palette
            .iter()
            .flatten()
            .map(|(state, count)| if *state != AIR { count.get() } else { 0 })
            .sum()
    }

    #[allow(dead_code)] // this will eventually be used for saving to the disk
    pub fn get_minimum_bit_width(&self) -> u8 {
        let len = self.palette.iter().flatten().count();

        Self::bit_width_for_len(len)
    }

    pub fn bit_width_for_len(len: usize) -> u8 {
        let bits = if len <= 1 {
            1
        } else {
            (usize::BITS - len.leading_zeros()) as u8
        };

        bits.next_power_of_two()
    }
}

#[cfg(test)]
mod tests {
    use crate::chunk::palette::BlockPalette;

    #[test]
    fn test_bit_width_calc() {
        let len_a = 1;
        let len_b = 3;
        let len_c = 5;
        let len_d = 12;
        let len_e = 42;

        assert_eq!(BlockPalette::bit_width_for_len(len_a), 1);
        assert_eq!(BlockPalette::bit_width_for_len(len_b), 2);
        assert_eq!(BlockPalette::bit_width_for_len(len_c), 4);
        assert_eq!(BlockPalette::bit_width_for_len(len_d), 4);
        assert_eq!(BlockPalette::bit_width_for_len(len_e), 8);
    }
}

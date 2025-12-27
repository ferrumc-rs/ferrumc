use std::num::NonZeroU16;
use crate::chunk::BlockStateId;

pub type PaletteIndex = u16;

const NON_ZERO_ONE: NonZeroU16 = match NonZeroU16::new(1) {
    Some(v) => v,
    None => unreachable!(),
};

pub enum PaletteResult<T> {
    Normal(T),
    RequiresResize(T, u8),
}

impl<T> PaletteResult<T> {
    pub fn new_bit_width(&self) -> Option<u8> {
        if let PaletteResult::RequiresResize(_, new_bit_width) = self {
            Some(*new_bit_width)
        } else {
            None
        }
    }

    pub fn unwrap(self) -> T {
        match self {
            PaletteResult::Normal(val) => val,
            PaletteResult::RequiresResize(val, _) => val,
        }
    }
}

pub struct BlockPalette {
    palette: Vec<Option<(BlockStateId, NonZeroU16)>>,
    free_count: u16,
}

impl BlockPalette {
    pub fn new() -> BlockPalette {
        BlockPalette {
            palette: vec![
                Some((0, NonZeroU16::MAX))
            ],
            free_count: 0,
        }
    }

    pub fn new_with_entry_count(entries: usize) -> BlockPalette {
        BlockPalette {
            palette: vec![
                Some((0, NonZeroU16::MAX));
                entries + 1
            ],
            free_count: entries as u16,
        }
    }

    pub fn translate_idx(&self, idx: PaletteIndex) -> Option<BlockStateId> {
        let idx = idx as usize;

        if self.palette.len() <= idx {
            None
        } else {
            self.palette[idx].as_ref().and_then(|(id, _)| Some(*id))
        }
    }

    pub fn add_block(&mut self, id: BlockStateId) -> PaletteResult<PaletteIndex> {
        if id == 0 { return PaletteResult::Normal(0) } // Air is always palette idx 0

        for (idx, val) in self.palette.iter_mut().enumerate() {
            if let Some((block_id, count)) = val {
                if *block_id == id {
                    *count = NonZeroU16::new(count.get().checked_add(1).expect("count should never exceed 4096")).expect("addition should not overflow");
                    return PaletteResult::Normal(idx as PaletteIndex);
                }
            }
        }

        if self.free_count == 0 {
            if self.palette.len() >= 4096 { panic!("Palette size should not be growing past 4096 because 4096 is the number of blocks per section") }

            let curr_bit_width = Self::bit_width_for_len(self.palette.len());
            let new_bit_width = Self::bit_width_for_len(self.palette.len() + 1);

            self.palette.push(Some((id, NON_ZERO_ONE)));
            let idx = (self.palette.len() - 1) as PaletteIndex;

            if curr_bit_width != new_bit_width {
                PaletteResult::RequiresResize(idx, new_bit_width)
            } else {
                PaletteResult::Normal(idx)
            }
        } else {
            let Some((idx, empty_entry)) = self.palette.iter_mut().enumerate().find(|(idx, val)| val.is_none()) else {
                panic!("palette should contain empty entry if free_count != 0");
            };

            let _ =empty_entry.insert((id, NON_ZERO_ONE));
            self.free_count -= 1;
            PaletteResult::Normal(idx as PaletteIndex)
        }
    }

    pub fn remove_block(&mut self, idx: PaletteIndex) {
        if idx == 0 { return; }

        let idx = idx as usize;

        debug_assert!(
            self.palette.len() > idx,
            "Palette index {} out of bounds for palette length {}",
            idx, self.palette.len(),
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

    pub fn get_minimum_bit_width(&self) -> u8 {
        let len = self.palette.iter().flatten().count();

        Self::bit_width_for_len(len)
    }

    pub fn bit_width_for_len(len: usize) -> u8 {
        let bits = if len <= 1 {
            1
        } else {
            (usize::BITS - (len - 1).leading_zeros()) as u8
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
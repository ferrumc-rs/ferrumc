use crate::palette::utils::{calculate_bits_per_entry, write_index};
use crate::palette::{Palette, PaletteType, MIN_BITS_PER_ENTRY};

impl<T> Palette<T>
where
    T: Clone + Default + PartialEq + Eq + std::hash::Hash,
{
    pub(crate) fn set_single(&mut self, index: usize, new_value: T) {
        let current_value = match &self.palette_type {
            PaletteType::Single(v) => v.clone(),
            _ => panic!("set_single called on non-single palette"),
        };

        // Out-of-bounds same value: just extend length
        if new_value == current_value {
            if index >= self.length {
                self.length = index + 1;
            }
            return;
        }

        // Different value: transition to Indirect
        let length = if index >= self.length {
            // Extend logical length first
            self.length = index + 1;
            self.length
        } else {
            self.length
        };

        let old_count = (length - 1) as u32;
        let new_count = 1u32;

        let mut bits_per_entry = calculate_bits_per_entry(2);
        if bits_per_entry < MIN_BITS_PER_ENTRY {
            bits_per_entry = MIN_BITS_PER_ENTRY;
        }

        let entries_per_i64 = 64 / bits_per_entry as usize;
        let data_len = length.div_ceil(entries_per_i64);
        let mut data = vec![0i64; data_len];

        for i in 0..length {
            let palette_index = if i == index { 1 } else { 0 };
            write_index(&mut data, bits_per_entry, i, palette_index as i64);
        }

        self.palette_type = PaletteType::Indirect {
            bits_per_entry,
            data,
            palette: vec![(old_count, current_value), (new_count, new_value)],
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::palette::{Palette, PaletteType, INDIRECT_THRESHOLD};

    #[test]
    fn set_single_within_bounds_same_value_extends_length() {
        let mut palette = Palette::new(4, 25u32, INDIRECT_THRESHOLD);
        palette.set_single(2, 25);
        assert!(matches!(palette.palette_type, PaletteType::Single(_)));
        assert_eq!(palette.length, 4);
        assert_eq!(palette.get(2), Some(&25));
    }

    #[test]
    fn set_single_within_bounds_different_value_converts_to_indirect() {
        let mut palette = Palette::new(3, 42u32, INDIRECT_THRESHOLD);
        palette.set_single(2, 7);
        match &palette.palette_type {
            PaletteType::Indirect { bits_per_entry, .. } => assert_eq!(*bits_per_entry, 4),
            _ => panic!("expected Indirect"),
        }
    }

    #[test]
    fn set_single_out_of_bounds_same_value_extends_length() {
        let mut palette = Palette::new(2, 10u32, INDIRECT_THRESHOLD);
        palette.set_single(5, 10);
        assert!(matches!(palette.palette_type, PaletteType::Single(_)));
        assert_eq!(palette.length, 6);
        assert_eq!(palette.get(5), Some(&10));
    }

    #[test]
    fn set_single_out_of_bounds_different_value_converts_to_indirect() {
        let mut palette = Palette::new(2, 15u32, INDIRECT_THRESHOLD);
        palette.set_single(4, 30);
        match &palette.palette_type {
            PaletteType::Indirect { bits_per_entry, .. } => assert_eq!(*bits_per_entry, 4),
            _ => panic!("expected Indirect"),
        }
    }

    #[test]
    #[should_panic(expected = "set_single called on non-single palette")]
    fn set_single_non_single_palette_panics() {
        let mut palette = Palette::from(vec![1, 2, 3]);
        palette.set_single(1, 4);
    }
}

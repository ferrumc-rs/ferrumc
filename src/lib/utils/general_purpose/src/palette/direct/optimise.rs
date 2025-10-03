// Downgrade to indirect if bpe drops below threshold, downgrade to single if there is only one type of element

use crate::palette::{Palette, PaletteType};
use std::hash::Hash;

impl<T> Palette<T>
where
    T: Clone + Default + PartialEq + Eq + Hash,
{
    pub(crate) fn optimise_direct(&mut self) {
        if let PaletteType::Direct(values) = &self.palette_type {
            let unique_values = crate::palette::utils::calculate_unique_values(values);
            let new_bpe = crate::palette::utils::calculate_bits_per_entry(unique_values);
            if new_bpe < self.indirect_threshold {
                // Downgrade to indirect
                let new_palette = Palette::from(values.clone());
                self.palette_type = new_palette.palette_type;
            } else if unique_values == 1 {
                // Downgrade to single value
                let single_value = values[0].clone();
                self.palette_type = PaletteType::Single(single_value);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::palette::{Palette, PaletteType};

    #[test]
    fn optimise_direct_downgrades_to_indirect_when_bpe_below_threshold() {
        let mut palette = Palette {
            palette_type: PaletteType::Direct(vec![1, 2, 3, 4, 5, 6]),
            indirect_threshold: 8,
            length: 6,
        };
        palette.optimise_direct();
        assert!(matches!(palette.palette_type, PaletteType::Indirect { .. }));
    }

    #[test]
    fn optimise_direct_downgrades_to_single_when_only_one_unique_value() {
        let mut palette = Palette {
            palette_type: PaletteType::Direct(vec![1, 1, 1]),
            indirect_threshold: 4,
            length: 3,
        };
        palette.optimise_direct();
        assert!(matches!(palette.palette_type, PaletteType::Single(value) if value == 1));
    }

    #[test]
    fn optimise_direct_does_not_downgrade_when_bpe_above_threshold() {
        let mut palette = Palette {
            palette_type: PaletteType::Direct(vec![1, 2, 3]),
            indirect_threshold: 2,
            length: 3,
        };
        palette.optimise_direct();
        assert!(matches!(palette.palette_type, PaletteType::Direct(_)));
    }

    #[test]
    fn optimise_direct_handles_empty_values_gracefully() {
        let mut palette = Palette {
            palette_type: PaletteType::<u32>::Direct(vec![]),
            indirect_threshold: 4,
            length: 0,
        };
        palette.optimise_direct();
        assert!(matches!(palette.palette_type, PaletteType::Direct(_)));
    }
}

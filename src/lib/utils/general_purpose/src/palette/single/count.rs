use crate::palette::Palette;

impl<T> Palette<T>
where
    T: Clone + Default + PartialEq,
{
    pub fn count_single(&self, value: &T) -> usize {
        match &self.palette_type {
            // Single variant: Check if the value matches the stored value.
            crate::palette::PaletteType::Single(v) => {
                if v == value {
                    self.length
                } else {
                    0
                }
            }
            _ => panic!("count_single called on non-Single palette type"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::palette::{Palette, PaletteType, INDIRECT_THRESHOLD};

    #[test]
    fn count_single_value_matches() {
        let palette = Palette {
            palette_type: PaletteType::Single(42),
            length: 10,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        assert_eq!(palette.count_single(&42), 10);
    }

    #[test]
    fn count_single_value_does_not_match() {
        let palette = Palette {
            palette_type: PaletteType::Single(42),
            length: 10,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        assert_eq!(palette.count_single(&7), 0);
    }

    #[test]
    #[should_panic(expected = "count_single called on non-Single palette type")]
    fn count_single_non_single_palette_type() {
        let palette = Palette {
            palette_type: PaletteType::Direct(vec![1, 2, 3]),
            length: 3,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        palette.count_single(&1);
    }
}

use crate::palette::{Palette, PaletteType};

impl<T> Palette<T>
where
    T: Clone + Default + PartialEq,
{
    pub(crate) fn count_indirect(&self, value: &T) -> usize {
        if let PaletteType::Indirect { palette, .. } = &self.palette_type {
            palette
                .iter()
                .find(|v| v.1 == *value)
                .map(|v| v.0)
                .unwrap_or(0) as usize
        } else {
            panic!("indirect count called on non-indirect palette");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::palette::{Palette, PaletteType, INDIRECT_THRESHOLD};

    #[test]
    fn count_existing_value_returns_correct_count() {
        let palette = Palette {
            palette_type: PaletteType::Indirect {
                palette: vec![(3, 42), (2, 7)],
                data: vec![],
                bits_per_entry: 4,
            },
            length: 5,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        assert_eq!(palette.count_indirect(&42), 3);
    }

    #[test]
    fn count_non_existing_value_returns_zero() {
        let palette = Palette {
            palette_type: PaletteType::Indirect {
                palette: vec![(3, 42), (2, 7)],
                data: vec![],
                bits_per_entry: 4,
            },
            length: 5,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        assert_eq!(palette.count_indirect(&99), 0);
    }

    #[test]
    #[should_panic(expected = "indirect count called on non-indirect palette")]
    fn count_non_indirect_palette_panics() {
        let palette = Palette {
            palette_type: PaletteType::Single(42),
            length: 5,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        palette.count_indirect(&42);
    }
}

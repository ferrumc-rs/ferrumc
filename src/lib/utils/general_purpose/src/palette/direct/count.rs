use crate::palette::{Palette, PaletteType};

impl<T> Palette<T>
where
    T: Clone + Default + PartialEq,
{
    pub(crate) fn count_direct(&self, value: &T) -> usize {
        if let PaletteType::Direct(data) = &self.palette_type {
            data.iter().filter(|&v| v == value).count()
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::palette::{Palette, PaletteType, INDIRECT_THRESHOLD};

    #[test]
    fn count_direct_value_present_returns_correct_count() {
        let palette = Palette {
            palette_type: PaletteType::Direct(vec![1, 2, 3, 2, 1]),
            length: 5,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        assert_eq!(palette.count_direct(&2), 2);
    }

    #[test]
    fn count_direct_value_not_present_returns_zero() {
        let palette = Palette {
            palette_type: PaletteType::Direct(vec![1, 2, 3]),
            length: 3,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        assert_eq!(palette.count_direct(&4), 0);
    }

    #[test]
    fn count_direct_empty_palette_returns_zero() {
        let palette = Palette {
            palette_type: PaletteType::Direct(Vec::new()),
            length: 0,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        assert_eq!(palette.count_direct(&1), 0);
    }

    #[test]
    fn count_direct_non_direct_palette_returns_zero() {
        let palette = Palette {
            palette_type: PaletteType::Single(42),
            length: 5,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        assert_eq!(palette.count_direct(&42), 0);
    }
}

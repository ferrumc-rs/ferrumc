use crate::palette::Palette;

impl<T> Palette<T> {
    pub fn get_count(&self, value: &T) -> usize
    where
        T: Eq,
    {
        match &self.palette_type {
            crate::palette::PaletteType::Single(v) => {
                if v == value {
                    self.length
                } else {
                    0
                }
            }
            crate::palette::PaletteType::Indirect { palette, .. } => palette
                .iter()
                .find(|(_, v)| v == value)
                .map(|(c, _)| *c as usize)
                .unwrap_or(0),
            crate::palette::PaletteType::Direct(values) => {
                values.iter().filter(|v| *v == value).count()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::palette::{Palette, PaletteType};

    #[test]
    fn test_single_variant_match() {
        let palette = Palette {
            palette_type: PaletteType::Single(42),
            length: 5,
        };
        assert_eq!(palette.get_count(&42), 5);
    }

    #[test]
    fn test_single_variant_no_match() {
        let palette = Palette {
            palette_type: PaletteType::Single(42),
            length: 5,
        };
        assert_eq!(palette.get_count(&7), 0);
    }

    #[test]
    fn test_indirect_variant() {
        let palette = Palette::from([1, 2, 1].to_vec());
        assert_eq!(palette.get_count(&1), 2);
        assert_eq!(palette.get_count(&2), 1);
        assert_eq!(palette.get_count(&3), 0);
    }

    #[test]
    fn test_direct_variant() {
        let palette = Palette {
            palette_type: PaletteType::Direct(vec![1, 2, 1, 3]),
            length: 4,
        };
        assert_eq!(palette.get_count(&1), 2);
        assert_eq!(palette.get_count(&2), 1);
        assert_eq!(palette.get_count(&3), 1);
        assert_eq!(palette.get_count(&4), 0);
    }
}

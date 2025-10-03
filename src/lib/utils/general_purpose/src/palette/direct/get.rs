use crate::palette::{Palette, PaletteType};

impl<T> Palette<T>
where
    T: Clone + Default + PartialEq,
{
    pub(crate) fn get_direct(&self, index: usize) -> Option<&T> {
        if let PaletteType::Direct(values) = &self.palette_type {
            values.get(index)
        } else {
            panic!("Called get_direct on a non-direct palette");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::palette::{Palette, PaletteType, INDIRECT_THRESHOLD};

    #[test]
    fn get_direct_within_bounds_returns_correct_value() {
        let palette = Palette {
            palette_type: PaletteType::Direct(vec![10, 20, 30, 40]),
            length: 4,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        assert_eq!(palette.get_direct(0), Some(&10));
        assert_eq!(palette.get_direct(1), Some(&20));
        assert_eq!(palette.get_direct(2), Some(&30));
        assert_eq!(palette.get_direct(3), Some(&40));
    }

    #[test]
    fn get_direct_out_of_bounds_returns_none() {
        let palette = Palette {
            palette_type: PaletteType::Direct(vec![10, 20, 30, 40]),
            length: 4,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        assert_eq!(palette.get_direct(4), None);
        assert_eq!(palette.get_direct(100), None);
    }

    #[test]
    #[should_panic(expected = "Called get_direct on a non-direct palette")]
    fn get_direct_non_direct_palette_panics() {
        let palette = Palette {
            palette_type: PaletteType::Single(42),
            length: 1,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        palette.get_direct(0);
    }

    #[test]
    fn get_direct_empty_palette_returns_none() {
        let palette = Palette {
            palette_type: PaletteType::Direct(Vec::<u64>::new()),
            length: 0,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        assert_eq!(palette.get_direct(0), None);
    }
}

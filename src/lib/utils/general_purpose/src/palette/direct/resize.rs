use crate::palette::{Palette, PaletteType};

impl<T> Palette<T>
where
    T: Clone + Default + PartialEq,
{
    pub(crate) fn resize_direct(&mut self, new_size: usize) {
        if let PaletteType::Direct(values) = &mut self.palette_type {
            values.resize(new_size, T::default());
            self.length = new_size;
        } else {
            panic!("Palette is not in direct mode");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::palette::{Palette, PaletteType, INDIRECT_THRESHOLD};

    #[test]
    fn resize_direct_increases_size_with_default_values() {
        let mut palette = Palette {
            palette_type: PaletteType::Direct(vec![1, 2, 3]),
            length: 3,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        palette.resize_direct(5);
        assert_eq!(palette.length, 5);
        assert_eq!(
            palette.palette_type,
            PaletteType::Direct(vec![1, 2, 3, 0, 0])
        );
    }

    #[test]
    fn resize_direct_decreases_size_truncating_values() {
        let mut palette = Palette {
            palette_type: PaletteType::Direct(vec![1, 2, 3, 4, 5]),
            length: 5,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        palette.resize_direct(3);
        assert_eq!(palette.length, 3);
        assert_eq!(palette.palette_type, PaletteType::Direct(vec![1, 2, 3]));
    }

    #[test]
    #[should_panic(expected = "Palette is not in direct mode")]
    fn resize_direct_non_direct_palette_panics() {
        let mut palette = Palette {
            palette_type: PaletteType::Single(42),
            length: 1,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        palette.resize_direct(5);
    }

    #[test]
    fn resize_direct_to_same_size_does_nothing() {
        let mut palette = Palette {
            palette_type: PaletteType::Direct(vec![1, 2, 3]),
            length: 3,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        palette.resize_direct(3);
        assert_eq!(palette.length, 3);
        assert_eq!(palette.palette_type, PaletteType::Direct(vec![1, 2, 3]));
    }

    #[test]
    fn resize_direct_empty_palette_increases_size() {
        let mut palette = Palette {
            palette_type: PaletteType::Direct(Vec::new()),
            length: 0,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        palette.resize_direct(4);
        assert_eq!(palette.length, 4);
        assert_eq!(palette.palette_type, PaletteType::Direct(vec![0, 0, 0, 0]));
    }
}

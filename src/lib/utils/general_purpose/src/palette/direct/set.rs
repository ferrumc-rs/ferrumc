use crate::palette::{Palette, PaletteType};

impl<T> Palette<T>
where
    T: Clone + Default + PartialEq,
{
    pub(crate) fn set_direct(&mut self, index: usize, value: T) {
        if let PaletteType::Direct(values) = &mut self.palette_type {
            if index >= values.len() {
                panic!("Index out of bounds");
            }
            values[index] = value;
        } else {
            panic!("Palette is not in direct mode");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::palette::{Palette, PaletteType, INDIRECT_THRESHOLD};

    #[test]
    fn set_direct_within_bounds_updates_value() {
        let mut palette = Palette {
            palette_type: PaletteType::Direct(vec![1, 2, 3, 4]),
            length: 4,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        palette.set_direct(2, 99);
        assert_eq!(palette.palette_type, PaletteType::Direct(vec![1, 2, 99, 4]));
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn set_direct_out_of_bounds_panics() {
        let mut palette = Palette {
            palette_type: PaletteType::Direct(vec![1, 2, 3]),
            length: 3,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        palette.set_direct(5, 99);
    }

    #[test]
    #[should_panic(expected = "Palette is not in direct mode")]
    fn set_direct_non_direct_palette_panics() {
        let mut palette = Palette {
            palette_type: PaletteType::Single(42),
            length: 1,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        palette.set_direct(0, 99);
    }
}

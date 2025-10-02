use crate::palette::Palette;

impl<T> Palette<T>
where
    T: Clone + Default + PartialEq,
{
    pub(crate) fn get_single(&self, index: usize) -> Option<&T> {
        if index >= self.length {
            return None;
        }
        match &self.palette_type {
            crate::palette::PaletteType::Single(value) => Some(value),
            _ => panic!("get_single called on non-single palette"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::palette::{Palette, INDIRECT_THRESHOLD};

    #[test]
    fn get_single_within_bounds_returns_value() {
        let palette = Palette {
            palette_type: crate::palette::PaletteType::Single(42),
            length: 5,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        assert_eq!(palette.get_single(3), Some(&42));
    }

    #[test]
    fn get_single_out_of_bounds_returns_none() {
        let palette = Palette {
            palette_type: crate::palette::PaletteType::Single(42),
            length: 5,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        assert_eq!(palette.get_single(5), None);
    }

    #[test]
    #[should_panic(expected = "get_single called on non-single palette")]
    fn get_single_non_single_palette_panics() {
        let palette = Palette {
            palette_type: crate::palette::PaletteType::Direct(vec![1, 2, 3]),
            length: 3,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        palette.get_single(1);
    }
}

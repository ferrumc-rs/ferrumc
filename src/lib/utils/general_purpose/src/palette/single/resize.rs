use crate::palette::Palette;
use std::hash::Hash;

impl<T> Palette<T>
where
    T: Clone + Default + Eq + Hash,
{
    pub(crate) fn resize_single(&mut self, new_length: usize) {
        if let crate::palette::PaletteType::Single(old_value) = &self.palette_type {
            self.length = new_length;
        } else {
            panic!("resize_single called on non-Single palette");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::palette::{Palette, INDIRECT_THRESHOLD};

    #[test]
    fn resize_single_same_value_extends_length() {
        let mut palette = Palette::new(3, 42u32, INDIRECT_THRESHOLD);
        palette.resize_single(5);
        assert_eq!(palette.len(), 5);
    }

    #[test]
    fn resize_single_different_value_converts_to_direct() {
        let mut palette = Palette::new(3, 42u32, INDIRECT_THRESHOLD);
        palette.resize_single(5);
        assert_eq!(palette.len(), 5);
        assert_eq!(palette.get(0), Some(&42));
        assert_eq!(palette.get(2), Some(&42));
    }

    #[test]
    #[should_panic(expected = "resize_single called on non-Single palette")]
    fn resize_single_non_single_palette_panics() {
        let mut palette = Palette::from(vec![1, 2, 3]);
        palette.resize_single(5);
    }
}

use crate::palette::Palette;

impl<T> Palette<T>
where
    T: Clone + Default + PartialEq,
{
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.length {
            return None;
        }
        match &self.palette_type {
            crate::palette::PaletteType::Single(value) => self.get_single(index),
            crate::palette::PaletteType::Indirect { .. } => self.get_indirect(index),
            crate::palette::PaletteType::Direct(values) => self.get_direct(index),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::palette::{Palette, INDIRECT_THRESHOLD};

    #[test]
    fn test_get_single_palette() {
        let palette = Palette::new(1, 42, INDIRECT_THRESHOLD);
        assert_eq!(palette.get(0), Some(&42));
        assert_eq!(palette.get(1), None);
    }

    #[test]
    fn test_get_direct_palette() {
        let palette = Palette::from(vec![10, 20, 30]);
        assert_eq!(palette.get(0), Some(&10));
        assert_eq!(palette.get(1), Some(&20));
        assert_eq!(palette.get(2), Some(&30));
        assert_eq!(palette.get(3), None);
    }

    #[test]
    fn test_get_indirect_palette() {
        let palette = Palette::from(vec![200, 300, 200, 300]);
        assert_eq!(palette.get(0), Some(&200));
        assert_eq!(palette.get(1), Some(&300));
        assert_eq!(palette.get(2), Some(&200));
        assert_eq!(palette.get(3), Some(&300));
        assert_eq!(palette.get(4), None);
    }
}

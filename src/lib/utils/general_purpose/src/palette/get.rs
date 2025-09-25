use crate::palette::Palette;

impl<T> Palette<T> {
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.length {
            return None;
        }
        match &self.palette_type {
            crate::palette::PaletteType::Single(value) => Some(value),
            crate::palette::PaletteType::Indirect { bits_per_entry, data, palette } =>
                {
                    let bits_per_entry = *bits_per_entry as usize;
                    let u64_index = (index * bits_per_entry) / 64;
                    let target_u64 = data.get(u64_index)?;
                    let bit_offset = (index * bits_per_entry) % 64;
                    let palette_index = (*target_u64 >> bit_offset) & ((1 << bits_per_entry) - 1);
                    palette.get(palette_index as usize).map(|x| &x.1)
                }
            crate::palette::PaletteType::Direct(values) => values.get(index),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::palette::Palette;

    #[test]
    fn test_get_single_palette() {
        let palette = Palette::new(1, 42);
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
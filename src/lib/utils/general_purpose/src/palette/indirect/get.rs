use crate::palette::Palette;

use crate::palette::utils::read_index;

impl<T> Palette<T>
where
    T: Clone + Default + PartialEq,
{
    pub(crate) fn get_indirect(&self, index: usize) -> Option<&T> {
        if let crate::palette::PaletteType::Indirect {
            bits_per_entry,
            data,
            palette,
        } = &self.palette_type
        {
            if index >= self.length {
                return None;
            }
            let pi = read_index(data, *bits_per_entry, index) as usize;
            palette.get(pi).map(|x| &x.1)
        } else {
            panic!("get_indirect called on non-indirect palette");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::palette::{Palette, INDIRECT_THRESHOLD};

    #[test]
    fn get_indirect_within_bounds_returns_correct_value() {
        let palette = Palette::from(vec![10, 20, 30, 40]);
        assert!(matches!(
            palette.palette_type,
            crate::palette::PaletteType::Indirect { .. }
        ));
        assert_eq!(palette.get_indirect(0), Some(&10));
        assert_eq!(palette.get_indirect(1), Some(&20));
        assert_eq!(palette.get_indirect(2), Some(&30));
        assert_eq!(palette.get_indirect(3), Some(&40));
    }

    #[test]
    fn get_indirect_out_of_bounds_returns_none() {
        let palette = Palette {
            palette_type: crate::palette::PaletteType::Indirect {
                bits_per_entry: 4,
                data: vec![0b0001_0010_0011_0100],
                palette: vec![(1, 10), (1, 20), (1, 30), (1, 40)],
            },
            length: 4,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        assert_eq!(palette.get_indirect(4), None);
    }

    #[test]
    #[should_panic(expected = "get_indirect called on non-indirect palette")]
    fn get_indirect_non_indirect_palette_panics() {
        let palette = Palette {
            palette_type: crate::palette::PaletteType::Single(42),
            length: 1,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        palette.get_indirect(0);
    }
}

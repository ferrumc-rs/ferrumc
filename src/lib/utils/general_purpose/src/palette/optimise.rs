use crate::palette::{Palette, PaletteType};
use std::hash::Hash;

impl<T: Clone + PartialEq + Hash + Eq + Default> Palette<T> {
    pub fn optimise(&mut self) {
        match self.palette_type {
            PaletteType::Single(_) => self.optimise_single(),
            PaletteType::Indirect { .. } => self.optimise_indirect(),
            PaletteType::Direct(_) => self.optimise_direct(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::palette::{PaletteType, INDIRECT_THRESHOLD};

    #[test]
    fn test_optimise_with_empty_palette() {
        let mut palette: Palette<u8> = Palette {
            length: 0,
            palette_type: PaletteType::Indirect {
                bits_per_entry: 0,
                data: vec![],
                palette: vec![],
            },
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        palette.optimise();
        if let PaletteType::Indirect { palette, .. } = palette.palette_type {
            assert!(palette.is_empty());
        } else {
            panic!("Expected PaletteType::Indirect");
        }
    }

    #[test]
    fn test_optimise_with_single_entry() {
        let mut palette = Palette {
            length: 1,
            palette_type: PaletteType::Indirect {
                bits_per_entry: 1,
                data: vec![0],
                palette: vec![(1, 42)],
            },
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        palette.optimise();
        match palette.palette_type {
            PaletteType::Single(value) => assert_eq!(value, 42),
            PaletteType::Indirect { .. } => panic!("Expected PaletteType::Single, got Indirect"),
            PaletteType::Direct(_) => panic!("Expected PaletteType::Single, got Direct"),
        }
    }

    #[test]
    fn test_optimise_removes_unused_entries() {
        let mut p = Palette::from(vec![42, 43, 44, 42, 42]);
        assert!(matches!(p.palette_type, PaletteType::Indirect { .. }));
        // Manually add an unused entry
        p.set(1, 42);
        p.optimise();
        assert_eq!(*p.get(1).unwrap(), 42);
        if let PaletteType::Indirect { palette, .. } = p.palette_type {
            assert_eq!(palette.len(), 2);
        } else {
            panic!("Expected PaletteType::Indirect");
        }
    }
}

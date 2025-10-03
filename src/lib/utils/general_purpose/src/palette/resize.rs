use crate::palette::{Palette, PaletteType};
use std::hash::Hash;

impl<T: Clone + Eq + Hash + Default> Palette<T> {
    pub fn resize(&mut self, new_length: usize) {
        match self.palette_type {
            PaletteType::Single(_) => self.resize_single(new_length),
            PaletteType::Indirect { .. } => self.resize_indirect(new_length),
            PaletteType::Direct(_) => self.resize_direct(new_length),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::palette::{Palette, PaletteType, INDIRECT_THRESHOLD};
    use std::assert_matches::assert_matches;

    #[test]
    fn resize_single_same_value_stays_single() {
        let mut p = Palette::new(2, 5u32, INDIRECT_THRESHOLD);
        assert_matches!(p.palette_type, PaletteType::Single(_));
        p.resize(5);
        assert_matches!(p.palette_type, PaletteType::Single(_));
        assert_eq!(p.len(), 5);
    }

    #[test]
    fn resize_indirect_with_existing_value_stays_indirect() {
        let mut p: Palette<u32> = Palette::from(vec![1, 2, 1, 2]);
        p.resize(6);
        match p.palette_type {
            PaletteType::Indirect { .. } => {}
            _ => panic!("expected Indirect"),
        }
        assert_eq!(p.len(), 6);
    }

    #[test]
    fn resize_indirect_add_new_value_within_16bpe_stays_indirect() {
        let mut p: Palette<u32> = Palette::from(vec![10, 20, 10, 20]);
        p.resize(6);
        match &p.palette_type {
            PaletteType::Indirect { bits_per_entry, .. } => assert_eq!(*bits_per_entry, 4),
            _ => panic!("expected Indirect"),
        }
        assert_eq!(p.len(), 6);
    }
}

use crate::palette::{Palette, PaletteType};
use std::hash::Hash;

impl<T: Clone + Eq + Hash> Palette<T> {
    pub fn resize(&mut self, new_length: usize, new_value: T) {
        // Downsizing will be implemented later: only adjust logical length.
        if new_length <= self.length {
            self.length = new_length;
            return;
        }

        let old_len = self.length;

        // Take ownership of the current palette_type to avoid borrow conflicts
        let new_palette_type =
            match std::mem::replace(&mut self.palette_type, PaletteType::Direct(Vec::new())) {
                PaletteType::Single(old_value) => {
                    if old_value == new_value {
                        // Same value; just extend length
                        self.length = new_length;
                        PaletteType::Single(old_value)
                    } else {
                        self.length = new_length;
                        Palette::from(vec![old_value.clone(); new_length]).palette_type
                    }
                }
                PaletteType::Indirect { palette, .. } => {
                    // Check if new_value already exists in palette
                    let mut values = (0..palette.len() - 1)
                        .map(|i| self.get(i).unwrap().clone())
                        .collect::<Vec<_>>();
                    let size_difference = new_length - old_len;
                    for _ in 0..size_difference {
                        values.push(new_value.clone())
                    }
                    Palette::from(values).palette_type
                }
                PaletteType::Direct(mut values) => {
                    if new_length > values.len() {
                        values.resize(new_length, new_value);
                    } else if new_length > old_len {
                        for _ in old_len..new_length {
                            values.push(new_value.clone());
                        }
                    }
                    self.length = new_length;
                    PaletteType::Direct(values)
                }
            };

        self.palette_type = new_palette_type;
    }
}

// Bit-packed helpers

#[cfg(test)]
mod tests {
    use crate::palette::{Palette, PaletteType};

    #[test]
    fn resize_single_same_value_stays_single() {
        let mut p = Palette::new(2, 5u32);
        p.resize(5, 5);
        assert!(matches!(p.palette_type, PaletteType::Single(_)));
        assert_eq!(p.len(), 5);
        // spot check via get
        assert_eq!(p.get(4), Some(&5));
    }

    #[test]
    fn resize_single_different_value_becomes_indirect_bpe_min_4() {
        let mut p = Palette::new(2, 5u32);
        p.resize(5, 7);
        match &p.palette_type {
            PaletteType::Indirect { bits_per_entry, .. } => assert_eq!(*bits_per_entry, 4),
            _ => panic!("expected Indirect"),
        }
        assert_eq!(p.len(), 5);
        assert_eq!(p.get(0), Some(&5));
        assert_eq!(p.get(1), Some(&5));
        assert_eq!(p.get(2), Some(&7));
        assert_eq!(p.get(3), Some(&7));
        assert_eq!(p.get(4), Some(&7));
    }

    #[test]
    fn resize_indirect_with_existing_value_stays_indirect() {
        let mut p: Palette<u32> = Palette::from(vec![1, 2, 1, 2]);
        p.resize(6, 1);
        match p.palette_type {
            PaletteType::Indirect { .. } => {}
            _ => panic!("expected Indirect"),
        }
        assert_eq!(p.len(), 6);
        assert_eq!(p.get(4), Some(&1));
        assert_eq!(p.get(5), Some(&1));
    }

    #[test]
    fn resize_indirect_add_new_value_within_16bpe_stays_indirect() {
        let mut p: Palette<u32> = Palette::from(vec![1, 2, 1, 2]);
        p.resize(6, 3);
        match &p.palette_type {
            PaletteType::Indirect { bits_per_entry, .. } => {
                // palette size 3 -> bpe still 4 (min 4)
                assert_eq!(*bits_per_entry, 4);
            }
            _ => panic!("expected Indirect"),
        }
        assert_eq!(p.get(4), Some(&3));
        assert_eq!(p.get(5), Some(&3));
    }

    #[test]
    fn resize_indirect_exceed_16bpe_becomes_direct() {
        // Construct an indirect palette with 65536 unique entries in the palette vector.
        // Only one actual entry is used (index 0), so data is minimal.
        let palette_vec: Vec<(u16, u32)> = (0..=65535).map(|i| (65535, i as u32)).collect();
        let mut p = Palette {
            length: 1,
            palette_type: PaletteType::Indirect {
                bits_per_entry: 16,
                data: vec![0u64], // index 0
                palette: palette_vec,
            },
        };

        // Adding a new distinct value will make palette size 65537 -> bpe 17 (>16), so bump to Direct.
        p.resize(3, 999_999u32);
        match &p.palette_type {
            PaletteType::Direct(v) => {
                assert_eq!(v.len(), 3);
                assert_eq!(v[0], 0); // original value at palette index 0
                assert_eq!(v[1], 999_999);
                assert_eq!(v[2], 999_999);
            }
            _ => panic!("expected Direct"),
        }
        assert_eq!(p.len(), 3);
    }
}

use crate::palette::{Palette, PaletteType};
use std::hash::Hash;

impl<T: Clone + Eq + Hash + Default> Palette<T> {
    pub fn set(&mut self, index: usize, new_value: T) {
        if index >= self.length {
            self.resize(index + 1);
        }
        match self.palette_type {
            PaletteType::Single(_) => self.set_single(index, new_value),
            PaletteType::Indirect { .. } => self.set_indirect(index, new_value),
            PaletteType::Direct(_) => self.set_direct(index, new_value),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::palette::{Palette, PaletteType, INDIRECT_THRESHOLD};
    use std::assert_matches::assert_matches;

    #[test]
    fn set_within_single_same_value_no_change() {
        let mut p = Palette::new(3, 9u32, INDIRECT_THRESHOLD);
        p.set(1, 9);
        assert!(matches!(p.palette_type, PaletteType::Single(_)));
        assert_eq!(p.get(0), Some(&9));
        assert_eq!(p.get(1), Some(&9));
        assert_eq!(p.get(2), Some(&9));
    }

    #[test]
    fn set_within_single_new_value_becomes_indirect() {
        let mut p = Palette::new(4, 1u32, INDIRECT_THRESHOLD);
        assert_matches!(p.palette_type, PaletteType::Single(_));
        p.set(2, 7);
        match &p.palette_type {
            PaletteType::Indirect { bits_per_entry, .. } => assert_eq!(*bits_per_entry, 4),
            _ => panic!("expected Indirect"),
        }
        assert_eq!(p.get(2), Some(&7));
    }

    #[test]
    fn set_indirect_existing_value() {
        let mut p: Palette<u32> = Palette::from(vec![1, 2, 1, 2]);
        p.set(0, 2);
        assert_eq!(p.get(0), Some(&2));
        assert!(matches!(p.palette_type, PaletteType::Indirect { .. }));
    }

    #[test]
    fn set_indirect_add_new_value_within_16bpe() {
        let mut p: Palette<u32> = Palette::from(vec![10, 20, 10, 20]);
        p.set(1, 30);
        match &p.palette_type {
            PaletteType::Indirect { bits_per_entry, .. } => assert_eq!(*bits_per_entry, 4),
            _ => panic!("expected Indirect"),
        }
        assert_eq!(p.get(0), Some(&10));
        assert_eq!(p.get(1), Some(&30));
    }

    #[test]
    fn set_out_of_bounds_triggers_resize() {
        let mut p = Palette::new(2, 5, INDIRECT_THRESHOLD);
        p.set(4, 7);
        assert_eq!(p.len(), 5);
        assert_eq!(p.get(4), Some(&7));
    }

    #[test]
    fn set_indirect_exceed_16bpe_becomes_direct() {
        let mut p: Palette<u32> =
            Palette::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
        p.indirect_threshold = 3;
        assert_matches!(p.palette_type, PaletteType::Indirect { .. });
        p.set(0, 16);
        assert_matches!(p.palette_type, PaletteType::Direct(_));
        assert_eq!(p.get(0), Some(&16));
        assert_eq!(p.get(1), Some(&2));
        assert_eq!(p.get(14), Some(&15));
    }

    #[test]
    fn test_tiny() {
        let mut p = Palette::new(1, 100u8, INDIRECT_THRESHOLD);
        p.set(0, 100);
        p.set(1, 101);
        p.set(2, 102);
        p.set(3, 103);
        p.set(4, 104);
        p.set(5, 105);
        p.set(6, 106);
        p.set(7, 107);
        p.set(8, 108);
        p.set(9, 109);
        assert_eq!(p.length, 10);
        assert_eq!(p.get(0), Some(&100));
        assert_eq!(p.get(1), Some(&101));
        assert_eq!(p.get(2), Some(&102));
        assert_eq!(p.get(3), Some(&103));
        assert_eq!(p.get(4), Some(&104));
        assert_eq!(p.get(5), Some(&105));
        assert_eq!(p.get(6), Some(&106));
        assert_eq!(p.get(7), Some(&107));
        assert_eq!(p.get(8), Some(&108));
        assert_eq!(p.get(9), Some(&109));
    }

    #[test]
    fn test_scaling() {
        for size in 1..100 {
            use rand::Rng;
            let mut p = Palette::new(size, 0u8, INDIRECT_THRESHOLD);
            if let PaletteType::Single(v) = &p.palette_type {
                assert_eq!(*v, 0, "Failed at size {}", size);
            } else {
                panic!("expected Single palette type");
            };
            let random_values: Vec<u8> = (0..size)
                .map(|_| rand::rng().random_range(u8::MIN..=size as u8))
                .collect();
            for (i, &v) in random_values.iter().enumerate() {
                p.set(i, v);
            }
            assert_eq!(p.length, size, "Failed at size {}", size);
            for (i, &v) in random_values.iter().enumerate() {
                assert_eq!(p.get(i), Some(&v), "Failed at size {}", size);
            }
        }
    }

    #[test]
    fn test_small_random() {
        let size = 32;
        use rand::Rng;
        let mut p = Palette::new(size, 0u8, INDIRECT_THRESHOLD);
        if let PaletteType::Single(v) = &p.palette_type {
            assert_eq!(*v, 0);
        } else {
            panic!("expected Single palette type");
        };
        let random_values: Vec<u8> = (0..size)
            .map(|_| rand::rng().random_range(u8::MIN..=size as u8))
            .collect();
        for (i, &v) in random_values.iter().enumerate() {
            p.set(i, v);
        }
        assert_eq!(p.length, size);
        for (i, &v) in random_values.iter().enumerate() {
            assert_eq!(p.get(i), Some(&v));
        }
    }

    #[test]
    fn test_large() {
        let values: Vec<u16> = (0..5000).map(|v| v as u16).collect();
        let p = Palette::from(values.clone());
        assert_eq!(p.length, 5000);
        for (i, &v) in values.iter().enumerate() {
            assert_eq!(p.get(i), Some(&v));
        }
    }

    #[test]
    fn test_massive_random() {
        use rand::Rng;
        let mut p = Palette::new(10000, 0u32, INDIRECT_THRESHOLD);
        if let PaletteType::Single(v) = &p.palette_type {
            assert_eq!(*v, 0);
        } else {
            panic!("expected Single palette type");
        };
        let random_values: Vec<u32> = (0..10000)
            .map(|_| rand::rng().random_range(u32::MIN..=u32::MAX))
            .collect();
        for (i, &v) in random_values.iter().enumerate() {
            p.set(i, v);
        }
        assert_eq!(p.length, 10000);
        for (i, &v) in random_values.iter().enumerate() {
            assert_eq!(p.get(i), Some(&v));
        }
    }
}

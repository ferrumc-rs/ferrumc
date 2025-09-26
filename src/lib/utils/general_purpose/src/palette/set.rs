use crate::palette::utils::{calculate_bits_per_entry, read_index, write_index};
use crate::palette::{Palette, PaletteType, MIN_BITS_PER_ENTRY};
use std::hash::Hash;

impl<T: Clone + Eq + Hash> Palette<T> {
    pub fn set(&mut self, index: usize, new_value: T) {
        // If out of bounds, resize to fit and fill with new_value
        if index >= self.length {
            if let PaletteType::Indirect {
                data,
                bits_per_entry,
                ..
            } = &mut self.palette_type
            {
                let entries_per_i64 = (64f64 / *bits_per_entry as f64).floor() as usize;
                let i64_index = index / entries_per_i64;
                let extra_u64s_needed = i64_index + 1 - data.len();
                for _ in 0..extra_u64s_needed {
                    data.push(0u64);
                }
            }
            self.length = index + 1;
        }

        let existing_count = self.get_count(&new_value);

        // Work on a moved-out palette_type to avoid borrow conflicts
        let new_palette_type =
            match std::mem::replace(&mut self.palette_type, PaletteType::Direct(Vec::new())) {
                PaletteType::Single(existing_value) => {
                    if existing_value == new_value {
                        // No change needed
                        PaletteType::Single(existing_value)
                    } else {
                        // Transition to Indirect
                        let palette = vec![
                            (self.length as u16 - 1, existing_value.clone()),
                            (1, new_value.clone()),
                        ];
                        let bits_per_entry = MIN_BITS_PER_ENTRY;
                        let mut data =
                            vec![0u64; (self.length * bits_per_entry as usize).div_ceil(64)];
                        for i in 0..self.length {
                            let palette_index = if i == index { 1 } else { 0 };
                            write_index(&mut data, bits_per_entry, i, palette_index);
                        }
                        PaletteType::Indirect {
                            bits_per_entry,
                            data,
                            palette,
                        }
                    }
                }
                PaletteType::Indirect {
                    mut bits_per_entry,
                    mut data,
                    mut palette,
                } => {
                    let old_palette_index = read_index(&data, bits_per_entry, index) as usize;
                    if palette[old_palette_index].1 == new_value {
                        // No change needed
                        PaletteType::Indirect {
                            bits_per_entry,
                            data,
                            palette,
                        }
                    } else if existing_count > 0 {
                        let palette_index = palette
                            .iter()
                            .position(|(_, v)| *v == new_value)
                            .expect("Value not found in palette");
                        // Existing value, just update index
                        write_index(&mut data, bits_per_entry, index, palette_index as u64);
                        // Update counts
                        if let Some(count) = palette.get_mut(old_palette_index) {
                            count.0 -= 1;
                        }
                        if let Some(count) = palette.get_mut(palette_index) {
                            count.0 += 1;
                        }
                        PaletteType::Indirect {
                            bits_per_entry,
                            data,
                            palette,
                        }
                    } else {
                        // New value
                        let unique_values = palette.len() + 1;
                        if calculate_bits_per_entry(unique_values) < 15 {
                            // Can still fit in Indirect
                            palette.push((1, new_value.clone()));
                            write_index(
                                &mut data,
                                bits_per_entry,
                                index,
                                (unique_values - 1) as u64,
                            );
                            // Update counts
                            if let Some(count) = palette.get_mut(old_palette_index) {
                                count.0 -= 1;
                            }
                            bits_per_entry = calculate_bits_per_entry(unique_values);
                            PaletteType::Indirect {
                                bits_per_entry,
                                data,
                                palette,
                            }
                        } else {
                            // Transition to Direct
                            let mut values = Vec::with_capacity(self.length);
                            for i in 0..self.length {
                                let index = read_index(&data, bits_per_entry, i);
                                values.push(palette[index as usize].1.clone());
                            }
                            values[index] = new_value.clone();
                            PaletteType::Direct(values)
                        }
                    }
                }
                PaletteType::Direct(mut values) => {
                    values[index] = new_value;
                    PaletteType::Direct(values)
                }
            };

        self.palette_type = new_palette_type;
    }
}

#[cfg(test)]
mod tests {
    use crate::palette::{Palette, PaletteType};

    #[test]
    fn set_within_single_same_value_no_change() {
        let mut p = Palette::new(3, 9u32);
        p.set(1, 9);
        assert!(matches!(p.palette_type, PaletteType::Single(_)));
        assert_eq!(p.get(0), Some(&9));
        assert_eq!(p.get(1), Some(&9));
        assert_eq!(p.get(2), Some(&9));
    }

    #[test]
    fn set_within_single_new_value_becomes_indirect() {
        let mut p = Palette::new(4, 1u32);
        p.set(2, 7);
        match &p.palette_type {
            PaletteType::Indirect { bits_per_entry, .. } => assert_eq!(*bits_per_entry, 4),
            _ => panic!("expected Indirect"),
        }
        assert_eq!(p.get(0), Some(&1));
        assert_eq!(p.get(1), Some(&1));
        assert_eq!(p.get(2), Some(&7));
        assert_eq!(p.get(3), Some(&1));
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
        let mut p = Palette::new(2, 5u32);
        p.set(4, 7);
        assert_eq!(p.len(), 5);
        // Newly appended slots filled with 7 due to resize semantics
        assert_eq!(p.get(2), Some(&7));
        assert_eq!(p.get(3), Some(&7));
        assert_eq!(p.get(4), Some(&7));
    }

    #[test]
    fn set_indirect_exceed_16bpe_becomes_direct() {
        let palette_vec: Vec<(u16, u32)> = (0..=65535).map(|i| (65535, i as u32)).collect();
        let mut p = Palette {
            length: 1,
            palette_type: PaletteType::Indirect {
                bits_per_entry: 16,
                data: vec![0u64], // index 0
                palette: palette_vec,
            },
        };
        p.set(0, 999_999u32);
        match &p.palette_type {
            PaletteType::Direct(v) => {
                assert_eq!(v.len(), 1);
                assert_eq!(v[0], 999_999);
            }
            _ => panic!("expected Direct"),
        }
    }

    #[test]
    fn test_small_random() {
        use rand::Rng;
        let mut p = Palette::new(200, 0u8);
        if let PaletteType::Single(v) = &p.palette_type {
            assert_eq!(*v, 0);
        } else {
            panic!("expected Single palette type");
        };
        let random_values: Vec<u8> = (0..200)
            .map(|_| rand::rng().random_range(u8::MIN..=200))
            .collect();
        for (i, &v) in random_values.iter().enumerate() {
            p.set(i, v);
        }
        assert_eq!(p.length, 200);
        for (i, &v) in random_values.iter().enumerate() {
            assert_eq!(p.get(i), Some(&v));
        }
    }

    #[test]
    fn test_large_random() {
        use rand::Rng;
        let mut p = Palette::new(5000, 0u16);
        if let PaletteType::Single(v) = &p.palette_type {
            assert_eq!(*v, 0);
        } else {
            panic!("expected Single palette type");
        };
        let random_values: Vec<u16> = (0..5000)
            .map(|_| rand::rng().random_range(u16::MIN..=u16::MAX))
            .collect();
        for (i, &v) in random_values.iter().enumerate() {
            p.set(i, v);
        }
        assert_eq!(p.length, 5000);
        for (i, &v) in random_values.iter().enumerate() {
            assert_eq!(p.get(i), Some(&v));
        }
    }

    #[test]
    fn test_massive_random() {
        use rand::Rng;
        let mut p = Palette::new(700000, 0u32);
        if let PaletteType::Single(v) = &p.palette_type {
            assert_eq!(*v, 0);
        } else {
            panic!("expected Single palette type");
        };
        let random_values: Vec<u32> = (0..700000)
            .map(|_| rand::rng().random_range(u32::MIN..=u32::MAX))
            .collect();
        for (i, &v) in random_values.iter().enumerate() {
            p.set(i, v);
        }
        assert_eq!(p.length, 700000);
        for (i, &v) in random_values.iter().enumerate() {
            assert_eq!(p.get(i), Some(&v));
        }
    }
}

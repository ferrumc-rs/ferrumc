use super::resize::{read_index, write_index};
use crate::palette::{calculate_bits_per_entry, Palette, PaletteType, MIN_BITS_PER_ENTRY};

impl<T: Clone + Eq> Palette<T> {
    pub fn set(&mut self, index: usize, new_value: T) {
        // If out of bounds, resize to fit and fill with new_value
        if index >= self.length {
            self.resize(index + 1, new_value);
            return;
        }

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
                        let bits_per_entry = MIN_BITS_PER_ENTRY; // 2 unique values fit in 1 bit
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
                    } else if let Some((palette_index, _)) =
                        palette.iter().enumerate().find(|(_, v)| v.1 == new_value)
                    {
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
                        if calculate_bits_per_entry(unique_values) <= 15 {
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
                                let bits_per_entry =
                                    calculate_bits_per_entry(palette.len()) as usize;
                                let u64_index = (i * bits_per_entry) / 64;
                                let target_u64 = data[u64_index];
                                let bit_offset = (i * bits_per_entry) % 64;
                                let palette_index =
                                    (target_u64 >> bit_offset) & ((1 << bits_per_entry) - 1);
                                values.push(palette[palette_index as usize].1.clone());
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
}

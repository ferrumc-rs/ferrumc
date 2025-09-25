use crate::palette::{calculate_bits_per_entry, Palette, PaletteType, MIN_BITS_PER_ENTRY};

impl<T: Clone + Eq> Palette<T> {
    pub fn resize(&mut self, new_length: usize, new_value: T) {
        if new_length == self.length {
            return;
        }
        // Downsizing will be implemented later: only adjust logical length.
        if new_length < self.length {
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
                        // Different value; transition to Indirect with 2 entries
                        let bits_per_entry = MIN_BITS_PER_ENTRY; // 2 unique values fit in 1 bit
                        let palette = vec![
                            (old_len as u16, old_value.clone()),
                            (new_length as u16 - old_len as u16, new_value.clone()),
                        ];
                        let mut data =
                            vec![0u64; (new_length * bits_per_entry as usize).div_ceil(64)];
                        for i in 0..new_length {
                            let palette_index = if i < old_len { 0 } else { 1 };
                            write_index(&mut data, bits_per_entry, i, palette_index);
                        }
                        self.length = new_length;
                        PaletteType::Indirect {
                            bits_per_entry,
                            data,
                            palette,
                        }
                    }
                }
                PaletteType::Indirect {
                    bits_per_entry,
                    mut data,
                    mut palette,
                } => {
                    // Check if new_value already exists in palette
                    if let Some((palette_index, _)) =
                        palette.iter().enumerate().find(|(_, v)| v.1 == new_value)
                    {
                        // Existing value; just extend length and update indices
                        let old_length = self.length;
                        self.length = new_length;
                        // Resize data to accommodate new length
                        let required_u64s = (new_length * bits_per_entry as usize).div_ceil(64);
                        if data.len() < required_u64s {
                            data.resize(required_u64s, 0);
                        }
                        for i in old_length..new_length {
                            write_index(&mut data, bits_per_entry, i, palette_index as u64);
                        }
                        // Update count
                        if let Some(count) = palette.get_mut(palette_index) {
                            count.0 += (new_length - old_length) as u16;
                        }
                        PaletteType::Indirect {
                            bits_per_entry,
                            data,
                            palette,
                        }
                    } else {
                        // New value; check if we can still fit in Indirect
                        let unique_values = palette.len() + 1;
                        if calculate_bits_per_entry(unique_values) <= 15 {
                            // Can still fit in Indirect
                            palette.push((new_length as u16 - old_len as u16, new_value.clone()));
                            let old_length = self.length;
                            self.length = new_length;
                            // Resize data to accommodate new length
                            let required_u64s = (new_length * bits_per_entry as usize).div_ceil(64);
                            if data.len() < required_u64s {
                                data.resize(required_u64s, 0);
                            }
                            for i in old_length..new_length {
                                write_index(
                                    &mut data,
                                    bits_per_entry,
                                    i,
                                    (unique_values - 1) as u64,
                                );
                            }
                            PaletteType::Indirect {
                                bits_per_entry,
                                data,
                                palette,
                            }
                        } else {
                            // Exceeds 15 bpe; transition to Direct
                            let mut values: Vec<T> = Vec::with_capacity(new_length);
                            for i in 0..old_len {
                                let palette_index = read_index(&data, bits_per_entry, i) as usize;
                                values.push(palette[palette_index].1.clone());
                            }
                            for _ in old_len..new_length {
                                values.push(new_value.clone());
                            }
                            self.length = new_length;
                            PaletteType::Direct(values)
                        }
                    }
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

pub(super) fn read_index(data: &[u64], bits_per_entry: u8, index: usize) -> u64 {
    let bpe = bits_per_entry as usize;
    let bit_index = index * bpe;
    let word_index = bit_index / 64;
    let bit_offset = bit_index % 64;

    let mask128 = if bpe >= 64 {
        u128::MAX
    } else {
        (1u128 << bpe) - 1
    };
    let low_part = if word_index < data.len() {
        (data[word_index] >> bit_offset) as u128
    } else {
        0
    };

    if bit_offset + bpe <= 64 {
        (low_part & mask128) as u64
    } else {
        let high_bits = bit_offset + bpe - 64;
        let high_part = if word_index + 1 < data.len() {
            (data[word_index + 1] & ((1u64 << high_bits) - 1)) as u128
        } else {
            0
        };
        let combined = low_part | (high_part << (64 - bit_offset));
        (combined & mask128) as u64
    }
}

pub(super) fn write_index(data: &mut [u64], bits_per_entry: u8, index: usize, value: u64) {
    let bpe = bits_per_entry as usize;
    let bit_index = index * bpe;
    let word_index = bit_index / 64;
    let bit_offset = bit_index % 64;

    let mask = if bpe >= 64 {
        u64::MAX
    } else {
        (1u64 << bpe) - 1
    };
    let v = value & mask;

    if bit_offset + bpe <= 64 {
        // Entirely within one word
        let clear_mask = !(mask << bit_offset);
        data[word_index] &= clear_mask;
        data[word_index] |= v << bit_offset;
    } else {
        // Do not bridge u64s: start at next word
        let word_index = word_index + 1;
        let bit_offset = 0;
        let clear_mask = !(mask << bit_offset);
        data[word_index] &= clear_mask;
        data[word_index] |= v << bit_offset;
    }
}

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

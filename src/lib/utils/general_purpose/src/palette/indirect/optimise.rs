use crate::palette::{Palette, PaletteType, MIN_BITS_PER_ENTRY};

impl<T> Palette<T>
where
    T: Clone + Default + PartialEq,
{
    pub(crate) fn optimise_indirect(&mut self) {
        let (old_bits_per_entry, old_data, old_palette, length) =
            match std::mem::take(&mut self.palette_type) {
                PaletteType::Indirect {
                    bits_per_entry,
                    data,
                    palette,
                } => (bits_per_entry, data, palette, self.length),
                other => {
                    self.palette_type = other;
                    panic!("optimise_indirect called on non-indirect palette");
                }
            };

        // Filter out unused entries and build old_index -> new_index map
        let mut index_map: Vec<Option<u16>> = Vec::with_capacity(old_palette.len());
        let mut new_palette = Vec::with_capacity(old_palette.len());
        for (old_idx, (count, value)) in old_palette.into_iter().enumerate() {
            if count > 0 {
                let new_idx = new_palette.len() as u16;
                index_map.push(Some(new_idx));
                new_palette.push((count, value));
            } else {
                index_map.push(None);
            }
        }

        // If nothing removed, just restore (but still allow bits_per_entry shrink)
        let removed_any = index_map.iter().any(|m| m.is_none());
        // Extract all old indices
        let get_index = |data: &[u64], bpe: u8, i: usize| -> u64 {
            let bpe_usize = bpe as usize;
            let bit_index = i * bpe_usize;
            let u64_index = bit_index / 64;
            let bit_offset = bit_index % 64;
            let mask = if bpe == 64 {
                u64::MAX
            } else {
                (1u64 << bpe) - 1
            };
            if bit_offset + bpe_usize <= 64 {
                (data[u64_index] >> bit_offset) & mask
            } else {
                let low = data[u64_index] >> bit_offset;
                let high = data[u64_index + 1] << (64 - bit_offset);
                (low | high) & mask
            }
        };

        let mut remapped_indices: Vec<u16> = Vec::with_capacity(length);
        for i in 0..length {
            let old_index = get_index(&old_data, old_bits_per_entry, i) as usize;
            if old_index >= index_map.len() {
                panic!("Corrupt data: palette index out of range");
            }
            let new_index = if removed_any {
                index_map[old_index].expect("Data referenced a removed palette entry")
            } else {
                old_index as u16
            };
            remapped_indices.push(new_index);
        }

        // Recalculate bits_per_entry
        let needed_bits = if new_palette.is_empty() || new_palette.len() == 1 {
            MIN_BITS_PER_ENTRY
        } else {
            let max_index = new_palette.len() - 1;
            let mut bits = 0u8;
            while (1usize << bits) <= max_index {
                bits += 1;
            }
            bits.max(MIN_BITS_PER_ENTRY)
        };

        // Pack new indices
        let total_bits = length * needed_bits as usize;
        let mut new_data = vec![0u64; (total_bits + 63) / 64];

        let put_index = |data: &mut [u64], bpe: u8, i: usize, value: u64| {
            let bpe_usize = bpe as usize;
            let bit_index = i * bpe_usize;
            let u64_index = bit_index / 64;
            let bit_offset = bit_index % 64;
            let mask = if bpe == 64 {
                u64::MAX
            } else {
                (1u64 << bpe) - 1
            };
            if bit_offset + bpe_usize <= 64 {
                data[u64_index] &= !(mask << bit_offset);
                data[u64_index] |= (value & mask) << bit_offset;
            } else {
                let low_bits = 64 - bit_offset;
                let high_bits = bpe_usize - low_bits;
                let low_mask = (1u64 << low_bits) - 1;
                let high_mask = (1u64 << high_bits) - 1;

                data[u64_index] &= !(low_mask << bit_offset);
                data[u64_index] |= (value & low_mask) << bit_offset;

                let high_part = value >> low_bits;
                data[u64_index + 1] &= !high_mask;
                data[u64_index + 1] |= high_part & high_mask;
            }
        };

        for (i, &idx) in remapped_indices.iter().enumerate() {
            put_index(&mut new_data, needed_bits, i, idx as u64);
        }

        if new_palette.len() == 1 {
            // If only one entry remains, convert to Single
            self.palette_type = PaletteType::Single(new_palette[0].1.clone());
            self.length = length;
            return;
        }

        self.palette_type = PaletteType::Indirect {
            bits_per_entry: needed_bits,
            data: new_data,
            palette: new_palette,
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::palette::utils::pack_indices;
    use crate::palette::{Palette, INDIRECT_THRESHOLD};

    #[test]
    fn optimise_indirect_removes_unused_palette_entries() {
        // Palette entries (index -> (count, value)):
        // 0: (2, 10)
        // 1: (0, 20)  <-- will be removed
        // 2: (1, 30)
        // Data indices reference only 0,2,0 to avoid pointing at the removed entry.
        let mut palette = Palette {
            palette_type: crate::palette::PaletteType::Indirect {
                bits_per_entry: 4,
                data: pack_indices(&[0u16, 2, 0], 4),
                palette: vec![(2, 10), (0, 20), (1, 30)],
            },
            length: 3,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        palette.optimise_indirect();
        match &palette.palette_type {
            crate::palette::PaletteType::Indirect {
                bits_per_entry,
                data,
                palette,
            } => {
                // After removal, palette order becomes: old0 -> new0, old2 -> new1
                // Remapped indices: [0,1,0]
                assert_eq!(*bits_per_entry, 1.max(crate::palette::MIN_BITS_PER_ENTRY)); // stays at MIN_BITS_PER_ENTRY (4)
                let _expected = pack_indices(&[0u16, 1, 0], 1);
                // Because MIN_BITS_PER_ENTRY = 4, data is stored with 4 bits per entry; re-pack to 4-bit form for comparison.
                let expected_padded = pack_indices(&[0u16, 1, 0], 4);
                assert_eq!(*data, expected_padded);
                assert_eq!(palette.len(), 2);
            }
            _ => panic!("expected Indirect"),
        }
    }

    #[test]
    fn optimise_indirect_rebuilds_data_with_new_indices() {
        use crate::palette::utils::pack_indices;

        // Palette entries (index -> (count, value)):
        // 0: (2, 10)
        // 1: (0, 20)  <-- will be removed
        // 2: (1, 30)
        // 3: (1, 40)
        // Data indices reference only 0,2,3,0 to avoid pointing at the removed entry.
        let mut palette = Palette {
            palette_type: crate::palette::PaletteType::Indirect {
                bits_per_entry: 4,
                data: pack_indices(&[0u16, 2, 3, 0], 4),
                palette: vec![(2, 10), (0, 20), (1, 30), (1, 40)],
            },
            length: 4,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        palette.optimise_indirect();
        match &palette.palette_type {
            crate::palette::PaletteType::Indirect {
                bits_per_entry,
                data,
                palette,
            } => {
                // After removal, palette order becomes: old0 -> new0, old2 -> new1, old3 -> new2
                // Remapped indices: [0,1,2,0]
                assert_eq!(*bits_per_entry, 2.max(crate::palette::MIN_BITS_PER_ENTRY)); // stays at MIN_BITS_PER_ENTRY (4)
                let _expected = pack_indices(&[0u16, 1, 2, 0], 2);
                // Because MIN_BITS_PER_ENTRY = 4, data is stored with 4 bits per entry; re-pack to 4-bit form for comparison.
                let expected_padded = pack_indices(&[0u16, 1, 2, 0], 4);
                assert_eq!(*data, expected_padded);
                assert_eq!(palette.len(), 3);
            }
            _ => panic!("expected Indirect"),
        }
    }

    #[test]
    fn optimise_indirect_recalculates_bits_per_entry() {
        use crate::palette::utils::pack_indices;

        // Palette entries (index -> (count, value)):
        // 0: (1, 10)
        // 1: (1, 20)
        // 2: (1, 30)
        // 3: (1, 40)
        // Data indices reference all entries.
        let mut palette = Palette {
            palette_type: crate::palette::PaletteType::Indirect {
                bits_per_entry: 4,
                data: pack_indices(&[0u16, 1, 2, 3], 4),
                palette: vec![(1, 10), (1, 20), (1, 30), (1, 40)],
            },
            length: 4,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        palette.optimise_indirect();
        match &palette.palette_type {
            crate::palette::PaletteType::Indirect {
                bits_per_entry,
                data,
                palette,
            } => {
                // No entries removed; bits_per_entry should remain the same.
                assert_eq!(*bits_per_entry, 4);
                let expected = pack_indices(&[0u16, 1, 2, 3], 4);
                assert_eq!(*data, expected);
                assert_eq!(palette.len(), 4);
            }
            _ => panic!("expected Indirect"),
        }
    }

    #[test]
    #[should_panic(expected = "optimise_indirect called on non-indirect palette")]
    fn optimise_indirect_non_indirect_palette_panics() {
        let mut palette = Palette {
            palette_type: crate::palette::PaletteType::Single(42),
            length: 5,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        palette.optimise_indirect();
    }
}

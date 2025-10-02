use crate::palette::utils::{calculate_bits_per_entry, read_index, write_index};
use crate::palette::{Palette, PaletteType, MIN_BITS_PER_ENTRY};

impl<T> Palette<T>
where
    T: Clone + Default + PartialEq,
{
    pub(crate) fn set_indirect(&mut self, index: usize, value: T) {
        let (bits_per_entry, data_ptr, palette_ref, length, threshold);
        match &self.palette_type {
            PaletteType::Indirect {
                bits_per_entry: b,
                data,
                palette,
            } => {
                bits_per_entry = *b;
                data_ptr = data.as_ptr(); // only for length safety reasoning
                palette_ref = palette;
                length = self.length;
                threshold = self.indirect_threshold;
            }
            _ => panic!("set_indirect called on non-indirect palette"),
        }
        if index >= self.length {
            panic!("Index out of bounds");
        }

        // Work with mutable borrow now
        let (bits_per_entry_mut, data, palette) = match &mut self.palette_type {
            PaletteType::Indirect {
                bits_per_entry,
                data,
                palette,
            } => (bits_per_entry, data, palette),
            _ => unreachable!(),
        };

        let old_pi = read_index(data, *bits_per_entry_mut, index) as usize;
        let old_value = &palette[old_pi].1;
        if *old_value == value {
            return;
        }

        // Fast path: value exists already
        if let Some(existing_pos) = palette.iter().position(|(_, v)| *v == value) {
            // Update counts
            if old_pi != existing_pos {
                // decrement old
                {
                    let old_entry = &mut palette[old_pi];
                    old_entry.0 -= 1;
                }
                // increment existing
                palette[existing_pos].0 += 1;
                // write index
                write_index(data, *bits_per_entry_mut, index, existing_pos as u64);

                // If old entry now zero, remove and remap indices > removed
                if palette[old_pi].0 == 0 {
                    remove_palette_entry_and_reindex(
                        data,
                        palette,
                        *bits_per_entry_mut,
                        old_pi,
                        self.length,
                    );
                }

                if palette.len() == 1 {
                    // Collapse to Single
                    let only = palette[0].1.clone();
                    self.palette_type = PaletteType::Single(only);
                }
            }
            return;
        }

        // New unique value
        let current_unique = palette.len();
        let needed_bits = calculate_bits_per_entry(current_unique + 1).max(MIN_BITS_PER_ENTRY);

        // Convert to Direct if bpe would exceed threshold
        if needed_bits > threshold {
            // Build direct vector
            let mut direct = Vec::with_capacity(self.length);
            for i in 0..self.length {
                if i == index {
                    direct.push(value.clone());
                } else {
                    let pi = read_index(data, *bits_per_entry_mut, i) as usize;
                    direct.push(palette[pi].1.clone());
                }
            }
            self.palette_type = PaletteType::Direct(direct);
            return;
        }

        // If bits need to grow, repack
        if needed_bits > *bits_per_entry_mut {
            let old_bits = *bits_per_entry_mut;
            let entries_per_u64_new = 64 / needed_bits as usize;
            let data_len_new = (self.length + entries_per_u64_new - 1) / entries_per_u64_new;
            let mut new_data = vec![0u64; data_len_new];

            // Palette index for the new value will be appended (current_unique)
            for i in 0..self.length {
                if i == index {
                    write_index(&mut new_data, needed_bits, i, current_unique as u64);
                } else {
                    let pi = read_index(data, old_bits, i);
                    write_index(&mut new_data, needed_bits, i, pi);
                }
            }

            // Adjust counts
            palette[old_pi].0 -= 1;
            palette.push((1, value));
            *bits_per_entry_mut = needed_bits;
            *data = new_data;

            if palette[old_pi].0 == 0 {
                remove_palette_entry_and_reindex(
                    data,
                    palette,
                    *bits_per_entry_mut,
                    old_pi,
                    self.length,
                );
            }

            if palette.len() == 1 {
                let only = palette[0].1.clone();
                self.palette_type = PaletteType::Single(only);
            }
            return;
        }

        // Bits unchanged: just append palette entry if still capacity fits
        // Write new index
        write_index(data, *bits_per_entry_mut, index, current_unique as u64);
        // Update counts
        palette[old_pi].0 -= 1;
        palette.push((1, value));

        if palette[old_pi].0 == 0 {
            remove_palette_entry_and_reindex(
                data,
                palette,
                *bits_per_entry_mut,
                old_pi,
                self.length,
            );
        }

        if palette.len() == 1 {
            let only = palette[0].1.clone();
            self.palette_type = PaletteType::Single(only);
        }
    }
}

fn remove_palette_entry_and_reindex<T: Clone + Default + PartialEq>(
    data: &mut [u64],
    palette: &mut Vec<(u32, T)>,
    bits_per_entry: u8,
    removed_index: usize,
    length: usize,
) {
    palette.remove(removed_index);

    if palette.len() == 0 {
        return;
    }

    // If we removed the last element, indices remain valid
    if removed_index == palette.len() {
        return;
    }

    // Need to decrement all stored palette indices > removed_index
    for i in 0..length {
        let pi = read_index(data, bits_per_entry, i);
        if (pi as usize) > removed_index {
            write_index(data, bits_per_entry, i, pi - 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::palette::{Palette, INDIRECT_THRESHOLD};

    #[test]
    fn set_indirect_within_bounds_updates_value() {
        let mut palette = Palette {
            palette_type: crate::palette::PaletteType::Indirect {
                bits_per_entry: 4,
                data: vec![0b0000_0000_0000_0000],
                palette: vec![(1, 10), (1, 20), (1, 30), (1, 40)],
            },
            length: 4,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        palette.set_indirect(2, 42);
        assert_eq!(palette.get(2), Some(&42));
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn set_indirect_out_of_bounds_panics() {
        let mut palette = Palette {
            palette_type: crate::palette::PaletteType::Indirect {
                bits_per_entry: 4,
                data: vec![0b0000_0000_0000_0000],
                palette: vec![(1, 10), (1, 20), (1, 30), (1, 40)],
            },
            length: 4,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        palette.set_indirect(4, 50);
    }

    #[test]
    fn set_indirect_adds_new_value_to_palette() {
        let mut palette = Palette {
            palette_type: crate::palette::PaletteType::Indirect {
                bits_per_entry: 4,
                data: vec![0b0000_0000_0000_0000],
                palette: vec![(1, 10), (1, 20), (1, 30), (1, 40)],
            },
            length: 4,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        palette.set_indirect(1, 20);
        assert_eq!(palette.get(1), Some(&20));
    }

    #[test]
    fn set_indirect_existing_value_reuses_palette_index() {
        let mut palette = Palette {
            palette_type: crate::palette::PaletteType::Indirect {
                bits_per_entry: 4,
                data: vec![0b0000_0000_0000_0000],
                palette: vec![(1, 10), (1, 20), (1, 30), (1, 40)],
            },
            length: 4,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        palette.set_indirect(1, 10);
        assert_eq!(palette.get(1), Some(&10));
    }

    #[test]
    #[should_panic(expected = "set_indirect called on non-indirect palette")]
    fn set_indirect_on_non_indirect_panics() {
        let mut palette = Palette::new(3, 9u32, INDIRECT_THRESHOLD);
        palette.set_indirect(1, 10);
    }
}

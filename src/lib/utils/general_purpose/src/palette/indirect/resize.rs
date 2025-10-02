use crate::palette::Palette;

impl<T> Palette<T>
where
    T: Clone + Default + PartialEq,
{
    pub(crate) fn resize_indirect(&mut self, new_length: usize) {
        if let crate::palette::PaletteType::Indirect {
            bits_per_entry,
            data,
            palette,
        } = &mut self.palette_type
        {
            if new_length == self.length {
                return;
            }

            let old_length = self.length;

            let entries_per_u64 = 64 / *bits_per_entry as usize;
            let needed_u64s = (new_length + entries_per_u64 - 1) / entries_per_u64;

            if new_length < old_length {
                data.truncate(needed_u64s);
                // (Optional future improvement: decrement counts for removed tail entries.)
            } else {
                // Growing: ensure capacity, newly added indices default to palette index 0.
                if data.len() < needed_u64s {
                    data.extend(std::iter::repeat(0u64).take(needed_u64s - data.len()));
                }
                let added = new_length - old_length;
                if added > 0 {
                    if let Some(first) = palette.get_mut(0) {
                        first.0 = first.0.saturating_add(added as u32);
                    }
                }
            }

            self.length = new_length;
        } else {
            panic!("resize_indirect called on non-indirect palette");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::palette::utils::pack_indices;
    use crate::palette::{Palette, INDIRECT_THRESHOLD};

    #[test]
    fn resize_indirect_increases_length_and_clears_new_entries() {
        let mut palette = Palette {
            palette_type: crate::palette::PaletteType::Indirect {
                bits_per_entry: 4,
                data: pack_indices(&[0, 1, 2, 3], 4),
                palette: vec![(1, 10), (1, 20), (1, 30), (1, 40)],
            },
            length: 4,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        palette.resize_indirect(8);
        assert_eq!(palette.length, 8);
        assert_eq!(palette.get(0), Some(&10));
        assert_eq!(palette.get(3), Some(&40));
        assert_eq!(palette.get(4), Some(&10));
        assert_eq!(palette.get(7), Some(&10));
    }

    #[test]
    fn resize_indirect_decreases_length_and_truncates_data() {
        let mut palette = Palette {
            palette_type: crate::palette::PaletteType::Indirect {
                bits_per_entry: 4,
                data: vec![0b0001_0010_0011_0100, 0b0101_0110_0111_1000],
                palette: vec![(1, 10), (1, 20), (1, 30), (1, 40)],
            },
            length: 8,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        palette.resize_indirect(4);
        assert_eq!(palette.length, 4);
        assert_eq!(palette.get(4), None);
    }

    #[test]
    fn resize_indirect_same_length_no_change() {
        let mut palette = Palette {
            palette_type: crate::palette::PaletteType::Indirect {
                bits_per_entry: 4,
                data: pack_indices(&[0, 1, 2, 3], 4),
                palette: vec![(1, 10), (1, 20), (1, 30), (1, 40)],
            },
            length: 4,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        palette.resize_indirect(4);
        assert_eq!(palette.length, 4);
        assert_eq!(palette.get(0), Some(&10));
        assert_eq!(palette.get(3), Some(&40));
    }

    #[test]
    #[should_panic(expected = "resize_indirect called on non-indirect palette")]
    fn resize_indirect_non_indirect_palette_panics() {
        let mut palette = Palette {
            palette_type: crate::palette::PaletteType::Single(42),
            length: 4,
            indirect_threshold: INDIRECT_THRESHOLD,
        };
        palette.resize_indirect(8);
    }
}

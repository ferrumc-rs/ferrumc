use crate::palette::resize::{read_index, write_index};
use crate::palette::{calculate_bits_per_entry, Palette, PaletteType};
use std::hash::Hash;

impl<T: Clone + PartialEq + Hash + Eq> Palette<T> {
    pub fn optimise(&mut self) {
        // Clear out unused entries from palette and recalculate bits_per_entry
        if let PaletteType::Indirect {
            bits_per_entry,
            data,
            palette,
        } = &mut self.palette_type
        {
            let mut removed_indices = vec![];
            for (i, (count, _)) in palette.iter().enumerate() {
                if *count == 0 {
                    removed_indices.push(i);
                }
            }
            for &index in removed_indices.iter().rev() {
                palette.remove(index);
                // Update data to remove references to this palette index
                for i in 0..self.length {
                    let current_index = read_index(data, *bits_per_entry, i) as usize;
                    if current_index == index {
                        // This should not happen as count is 0, but just in case
                        write_index(data, *bits_per_entry, i, 0);
                    } else if current_index > index {
                        // Shift down by one
                        write_index(data, *bits_per_entry, i, (current_index - 1) as u64);
                    }
                }
            }
        }
        // Now attempt to downgrade to Single if in indirect mode, or try to downgrade to Indirect if in Direct mode
        let new_palette_type = match &self.palette_type {
            PaletteType::Single(value) => PaletteType::Single(value.clone()),
            PaletteType::Indirect {
                bits_per_entry,
                data,
                palette,
            } => {
                if palette.len() == 1 {
                    PaletteType::Single(palette[0].1.clone())
                } else {
                    let new_bits_per_entry = calculate_bits_per_entry(palette.len());
                    if new_bits_per_entry < *bits_per_entry {
                        // Rebuild data with new bits_per_entry
                        let mut new_data =
                            vec![0u64; (self.length * new_bits_per_entry as usize).div_ceil(64)];
                        for i in 0..self.length {
                            let palette_index = read_index(data, *bits_per_entry, i);
                            write_index(&mut new_data, new_bits_per_entry, i, palette_index);
                        }
                        PaletteType::Indirect {
                            bits_per_entry: new_bits_per_entry,
                            data: new_data,
                            palette: palette.clone(),
                        }
                    } else {
                        // No change
                        PaletteType::Indirect {
                            bits_per_entry: *bits_per_entry,
                            data: data.clone(),
                            palette: palette.clone(),
                        }
                    }
                }
            }
            PaletteType::Direct(values) => {
                let unique_values: Vec<(u16, T)> = {
                    use std::collections::HashMap;
                    let mut counts: HashMap<&T, u16> = HashMap::new();
                    for value in values {
                        *counts.entry(value).or_default() += 1;
                    }
                    counts.into_iter().map(|(v, c)| (c, v.clone())).collect()
                };
                if unique_values.len() == 1 {
                    PaletteType::Single(unique_values[0].1.clone())
                } else if calculate_bits_per_entry(unique_values.len()) <= 15 {
                    // Transition to Indirect
                    let bits_per_entry = calculate_bits_per_entry(unique_values.len());
                    let mut data = vec![0u64; (self.length * bits_per_entry as usize).div_ceil(64)];
                    for (i, value) in values.iter().enumerate() {
                        let palette_index =
                            unique_values.iter().position(|(_, v)| v == value).unwrap();
                        write_index(&mut data, bits_per_entry, i, palette_index as u64);
                    }
                    PaletteType::Indirect {
                        bits_per_entry,
                        data,
                        palette: unique_values,
                    }
                } else {
                    // Stay Direct
                    PaletteType::Direct(values.clone())
                }
            }
        };
        self.palette_type = new_palette_type;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::palette::PaletteType;

    #[test]
    fn test_optimise_with_empty_palette() {
        let mut palette: Palette<u8> = Palette {
            length: 0,
            palette_type: PaletteType::Indirect {
                bits_per_entry: 0,
                data: vec![],
                palette: vec![],
            },
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
        };
        palette.optimise();
        if let PaletteType::Single(value) = palette.palette_type {
            assert_eq!(value, 42);
        } else {
            panic!("Expected PaletteType::Single");
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

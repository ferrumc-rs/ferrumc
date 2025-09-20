use std::cmp::max;

mod get;
mod set;
mod resize;

pub struct Palette<T> {
    pub palette_type: PaletteType<T>,
    pub length: usize,
}

pub enum PaletteType<T> {
    Single(T),
    Indirect {
        bits_per_entry: u8,
        data: Vec<u64>,
        palette: Vec<T>,
    },
    Direct(Vec<T>),
}

impl<T> Palette<T> {
    pub fn new(size: usize, value: T) -> Self {
        Self {
            palette_type: PaletteType::Single(value),
            length: size,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }
}

impl<T: Eq + std::hash::Hash + Clone> From<Vec<T>> for Palette<T> {
    fn from(values: Vec<T>) -> Self {
        let length = values.len();
        if length == 1 {
            Self {
                palette_type: PaletteType::Single(values.into_iter().next().unwrap()),
                length,
            }
        } else {
            let unique_values = calculate_unique_values(&values);
            if calculate_bits_per_entry(unique_values) <= 15 {
                let bits_per_entry = calculate_bits_per_entry(unique_values);
                let palette: Vec<T> = {
                    use std::collections::HashSet;
                    let unique_values: HashSet<T> = values.iter().cloned().collect();
                    unique_values.into_iter().collect()
                };
                let mut data: Vec<u64> = vec![0; (length * bits_per_entry as usize).div_ceil(64)];
                for (i, value) in values.iter().enumerate() {
                    let palette_index = palette
                        .iter()
                        .position(|v| v == value)
                        .expect("Value not found in palette") as u64;
                    let bit_index = i * bits_per_entry as usize;
                    let array_index = bit_index / 64;
                    let bit_offset = bit_index % 64;

                    if bit_offset + bits_per_entry as usize <= 64 {
                        // Value fits within a single u64
                        data[array_index] |= palette_index << bit_offset;
                    } else {
                        // Value spans two u64s
                        let low_bits = 64 - bit_offset;
                        let high_bits = bits_per_entry as usize - low_bits;
                        data[array_index] |= (palette_index & ((1 << low_bits) - 1)) << bit_offset;
                        data[array_index + 1] |= palette_index >> low_bits;
                    }
                }
                Self {
                    palette_type: PaletteType::Indirect {
                        bits_per_entry,
                        data,
                        palette,
                    },
                    length,
                }
            } else {
                Self {
                    palette_type: PaletteType::Direct(values),
                    length,
                }
            }
        }
    }
}

#[inline]
pub(crate) fn calculate_bits_per_entry(palette_size: usize) -> u8 {
    max(4, (palette_size as f64).log2().ceil() as u8)
}

pub(crate) fn calculate_unique_values<T: Eq + std::hash::Hash>(values: &[T]) -> usize {
    use std::collections::HashSet;
    let unique_values: HashSet<&T> = values.iter().collect();
    unique_values.len()
}

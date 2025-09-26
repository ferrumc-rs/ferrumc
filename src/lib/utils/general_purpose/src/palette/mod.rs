use bitcode::{Decode, Encode};
use deepsize::DeepSizeOf;

mod count;
mod get;
mod optimise;
mod resize;
mod set;
mod utils;

const MIN_BITS_PER_ENTRY: u8 = 4;

#[derive(Encode, Decode, Clone, DeepSizeOf, Eq, PartialEq, Debug)]
pub struct Palette<T> {
    pub palette_type: PaletteType<T>,
    pub length: usize,
}
#[derive(Encode, Decode, Clone, DeepSizeOf, Eq, PartialEq, Debug)]
pub enum PaletteType<T> {
    Single(T),
    Indirect {
        bits_per_entry: u8,
        data: Vec<u64>,
        palette: Vec<(u16, T)>,
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

    pub fn is_empty(&self) -> bool {
        self.length == 0
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
            let unique_values = utils::calculate_unique_values(&values);
            if utils::calculate_bits_per_entry(unique_values) <= 15 {
                let bits_per_entry = utils::calculate_bits_per_entry(unique_values);
                let palette: Vec<(u16, T)> = {
                    use std::collections::HashMap;
                    let mut unique_values: HashMap<&T, u16> = HashMap::new();
                    for value in &values {
                        *unique_values.entry(value).or_default() += 1;
                    }
                    unique_values
                        .into_iter()
                        .map(|(v, c)| (c, v.clone()))
                        .collect()
                };
                let mut data: Vec<u64> = vec![0; (length * bits_per_entry as usize).div_ceil(64)];
                for (i, value) in values.iter().enumerate() {
                    let palette_index = palette
                        .iter()
                        .position(|v| v.1 == *value)
                        .expect("Value not found in palette")
                        as u64;
                    let u64_index = (i * bits_per_entry as usize) / 64;
                    let bit_offset = (i * bits_per_entry as usize) % 64;
                    data[u64_index] |= palette_index << bit_offset;
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

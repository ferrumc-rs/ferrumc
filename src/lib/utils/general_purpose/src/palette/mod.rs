use bitcode::{Decode, Encode};
use deepsize::DeepSizeOf;

mod count;
mod direct;
mod get;
mod indirect;
mod optimise;
mod resize;
mod set;
mod single;
mod utils;

const MIN_BITS_PER_ENTRY: u8 = 4;
const INDIRECT_THRESHOLD: u8 = 15;

#[derive(Encode, Decode, Clone, DeepSizeOf, Eq, PartialEq, Debug)]
pub struct Palette<T>
where
    T: Default + PartialEq + Clone,
{
    pub palette_type: PaletteType<T>,
    pub length: usize,
    pub indirect_threshold: u8,
}
#[derive(Encode, Decode, Clone, DeepSizeOf, Eq, PartialEq, Debug)]
pub enum PaletteType<T> {
    Single(T),
    Indirect {
        bits_per_entry: u8,
        data: Vec<u64>,
        palette: Vec<(u32, T)>,
    },
    Direct(Vec<T>),
}

impl<T> Palette<T>
where
    T: Clone + Default + PartialEq,
{
    pub fn new(size: usize, value: T, indirect_threshold: u8) -> Self {
        Self {
            palette_type: PaletteType::Single(value),
            length: size,
            indirect_threshold,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

impl<T: Eq + std::hash::Hash + Clone> From<Vec<T>> for Palette<T>
where
    T: Clone + Default + PartialEq,
{
    fn from(values: Vec<T>) -> Self {
        let length = values.len();
        if length == 1 {
            Self {
                palette_type: PaletteType::Single(values.into_iter().next().unwrap()),
                length,
                indirect_threshold: INDIRECT_THRESHOLD,
            }
        } else {
            let unique_values = utils::calculate_unique_values(&values);
            if utils::calculate_bits_per_entry(unique_values) <= 15 {
                let bits_per_entry = utils::calculate_bits_per_entry(unique_values);
                use std::collections::HashMap;
                let mut freq: HashMap<&T, u32> = HashMap::new();
                for v in &values {
                    *freq.entry(v).or_default() += 1;
                }
                let palette: Vec<(u32, T)> =
                    freq.into_iter().map(|(v, c)| (c, v.clone())).collect();
                let entries_per_u64 = 64 / bits_per_entry as usize;
                let data_len = (length + entries_per_u64 - 1) / entries_per_u64;
                let mut data = vec![0u64; data_len];
                for (i, value) in values.iter().enumerate() {
                    let palette_index = palette
                        .iter()
                        .position(|p| p.1 == *value)
                        .expect("Value not found in palette")
                        as u64;
                    utils::write_index(&mut data, bits_per_entry, i, palette_index);
                }
                Self {
                    palette_type: PaletteType::Indirect {
                        bits_per_entry,
                        data,
                        palette,
                    },
                    length,
                    indirect_threshold: INDIRECT_THRESHOLD,
                }
            } else {
                Self {
                    palette_type: PaletteType::Direct(values),
                    length,
                    indirect_threshold: INDIRECT_THRESHOLD,
                }
            }
        }
    }
}

impl<T: Default> Default for PaletteType<T> {
    fn default() -> Self {
        PaletteType::Single(T::default())
    }
}

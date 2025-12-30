use deepsize::DeepSizeOf;
use crate::pos::SectionBlockPos;

#[derive(Clone, DeepSizeOf)]
pub enum BiomeData {
    Uniform(u8),
    Mixed(Box<[u8]>)
}

impl BiomeData {
    pub fn new_uniform(value: u8) -> Self {
        BiomeData::Uniform(value)
    }

    pub fn new_mixed() -> Self {
        BiomeData::Mixed(vec![0u8; 64].into_boxed_slice())
    }

    pub fn fill_biome(&mut self, value: u8) {
        *self = BiomeData::new_uniform(value);
    }

    pub fn set_biome(&mut self, value: u8, pos: SectionBlockPos) {
        let idx = Self::get_idx(pos);

        match self {
            BiomeData::Uniform(data) => if *data != value {
                let mut new_data = vec![*data; 64].into_boxed_slice();
                new_data[idx] = value;
                *self = BiomeData::Mixed(new_data);
            },
            BiomeData::Mixed(data) => data[idx] = value,
        }
    }

    pub fn get_biome(&self, pos: SectionBlockPos) -> u8 {
        let idx = Self::get_idx(pos);

        match self {
            BiomeData::Uniform(data) => *data,
            BiomeData::Mixed(data) => data[idx],
        }
    }

    fn get_idx(pos: SectionBlockPos) -> usize {
        let x = pos.x >> 2;
        let y = pos.y >> 2;
        let z = pos.z >> 2;

        ((y << 4) | (z << 2) | x) as usize
    }
}
use crate::pos::SectionBlockPos;
use bitcode_derive::{Decode, Encode};
use bytemuck::{Pod, Zeroable};
use deepsize::DeepSizeOf;

#[repr(transparent)]
#[derive(Copy, Clone, Encode, Decode, Default, PartialEq, DeepSizeOf, Pod, Zeroable)]
pub struct BiomeType(pub u8);

#[derive(Clone, DeepSizeOf, Encode, Decode)]
pub enum BiomeData {
    Uniform(BiomeType),
    Mixed(Box<[BiomeType]>),
}

impl BiomeData {
    pub fn new_uniform(value: BiomeType) -> Self {
        BiomeData::Uniform(value)
    }

    pub fn new_mixed() -> Self {
        BiomeData::Mixed(vec![BiomeType::default(); 64].into_boxed_slice())
    }

    pub fn fill_biome(&mut self, value: BiomeType) {
        *self = BiomeData::new_uniform(value);
    }

    pub fn set_biome(&mut self, value: BiomeType, pos: SectionBlockPos) {
        let idx = Self::get_idx(pos);

        match self {
            BiomeData::Uniform(data) => {
                if *data != value {
                    let mut new_data = vec![*data; 64].into_boxed_slice();
                    new_data[idx] = value;
                    *self = BiomeData::Mixed(new_data);
                }
            }
            BiomeData::Mixed(data) => data[idx] = value,
        }
    }

    pub fn get_biome(&self, pos: SectionBlockPos) -> BiomeType {
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

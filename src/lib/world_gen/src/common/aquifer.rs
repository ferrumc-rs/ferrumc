use std::collections::BTreeMap;

use ferrumc_world::vanilla_chunk_format::BlockData;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FluidPicker(pub i32, pub FluidType);

impl FluidPicker {
    pub const EMPTY: FluidPicker = Self::new(-10000, FluidType::Air);
    pub const fn new(level: i32, fluid_type: FluidType) -> Self {
        Self(level, fluid_type)
    }
    pub const fn at(&self, y: i32) -> FluidType {
        if y < self.0 { self.1 } else { FluidType::Air }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FluidType {
    Air,
    Water,
    Lava,
}

impl From<FluidType> for BlockData {
    fn from(value: FluidType) -> Self {
        match value {
            FluidType::Air => BlockData::default(),
            FluidType::Water => BlockData {
                name: "minecraft:water".to_string(),
                properties: Some(BTreeMap::from([("level".to_string(), "0".to_string())])),
            },
            FluidType::Lava => BlockData {
                name: "minecraft:lava".to_string(),
                properties: Some(BTreeMap::from([("level".to_string(), "0".to_string())])),
            },
        }
    }
}

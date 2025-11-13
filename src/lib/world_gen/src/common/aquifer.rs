use ferrumc_macros::block;
use ferrumc_world::block_state_id::BlockStateId;

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

impl From<FluidType> for BlockStateId {
    fn from(value: FluidType) -> Self {
        match value {
            FluidType::Air => block!("air"),
            FluidType::Water => block!("water", {level: 0}),
            FluidType::Lava => block!("lava", {level: 0}),
        }
    }
}

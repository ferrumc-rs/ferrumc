#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum LargeMushroomBlockType {
    BrownMushroomBlock,
    ChorusPlant,
    MushroomStem,
    RedMushroomBlock,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct LargeMushroomBlock {
    pub block_type: LargeMushroomBlockType,
    pub down: bool,
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub up: bool,
    pub west: bool,
}

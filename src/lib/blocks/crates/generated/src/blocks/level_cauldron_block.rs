#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum LevelCauldronBlockType {
    PowderSnowCauldron,
    WaterCauldron,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct LevelCauldronBlock {
    pub block_type: LevelCauldronBlockType,
    pub level: i32,
}

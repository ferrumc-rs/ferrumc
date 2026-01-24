#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum WeightedPressurePlateBlockType {
    HeavyWeightedPressurePlate,
    LightWeightedPressurePlate,
    Target,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct WeightedPressurePlateBlock {
    pub block_type: WeightedPressurePlateBlockType,
    pub power: i32,
}

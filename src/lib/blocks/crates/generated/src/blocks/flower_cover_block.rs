#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum FlowerCoverBlockType {
    PinkPetals,
    Wildflowers,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct FlowerCoverBlock {
    pub block_type: FlowerCoverBlockType,
    pub facing: Direction,
    pub flower_amount: i32,
}

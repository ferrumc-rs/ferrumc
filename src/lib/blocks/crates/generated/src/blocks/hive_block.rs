#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum HiveBlockType {
    BeeNest,
    Beehive,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct HiveBlock {
    pub block_type: HiveBlockType,
    pub facing: Direction,
    pub honey_level: i32,
}

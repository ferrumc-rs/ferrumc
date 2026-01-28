#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum LiquidBlockType {
    Lava,
    Water,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct LiquidBlock {
    pub block_type: LiquidBlockType,
    pub level: i32,
}

#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum FurnaceBlockType {
    BlastFurnace,
    Furnace,
    RedstoneWallTorch,
    Smoker,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct FurnaceBlock {
    pub block_type: FurnaceBlockType,
    pub facing: Direction,
    pub lit: bool,
}

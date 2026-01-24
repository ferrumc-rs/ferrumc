#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum LeavesBlockType {
    AcaciaLeaves,
    AzaleaLeaves,
    BirchLeaves,
    CherryLeaves,
    DarkOakLeaves,
    FloweringAzaleaLeaves,
    JungleLeaves,
    MangroveLeaves,
    OakLeaves,
    PaleOakLeaves,
    SpruceLeaves,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct LeavesBlock {
    pub block_type: LeavesBlockType,
    pub distance: i32,
    pub persistent: bool,
    pub waterlogged: bool,
}

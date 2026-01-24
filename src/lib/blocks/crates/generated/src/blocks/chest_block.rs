#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum ChestBlockType {
    Chest,
    TrappedChest,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ChestBlock {
    pub block_type: ChestBlockType,
    pub facing: Direction,
    pub ty: ChestType,
    pub waterlogged: bool,
}

#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct RedstoneWireBlock {
    pub east: RedstoneSide,
    pub north: RedstoneSide,
    pub power: i32,
    pub south: RedstoneSide,
    pub west: RedstoneSide,
}

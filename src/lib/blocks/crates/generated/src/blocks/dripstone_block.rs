#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct DripstoneBlock {
    pub thickness: DripstoneThickness,
    pub vertical_direction: Direction,
    pub waterlogged: bool,
}

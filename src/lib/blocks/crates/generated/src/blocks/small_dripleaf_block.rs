#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SmallDripleafBlock {
    pub facing: Direction,
    pub half: DoubleBlockHalf,
    pub waterlogged: bool,
}

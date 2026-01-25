#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct LightningRodBlock {
    pub facing: Direction,
    pub powered: bool,
    pub waterlogged: bool,
}

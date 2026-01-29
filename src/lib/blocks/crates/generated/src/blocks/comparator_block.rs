#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ComparatorBlock {
    pub facing: Direction,
    pub mode: ComparatorMode,
    pub powered: bool,
}

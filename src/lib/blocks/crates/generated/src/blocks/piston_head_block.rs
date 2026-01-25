#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct PistonHeadBlock {
    pub facing: Direction,
    pub short: bool,
    pub ty: PistonType,
}

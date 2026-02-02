#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BellBlock {
    pub attachment: BellAttachType,
    pub facing: Direction,
    pub powered: bool,
}

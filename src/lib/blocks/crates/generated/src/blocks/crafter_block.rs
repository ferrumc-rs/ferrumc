#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CrafterBlock {
    pub crafting: bool,
    pub orientation: FrontAndTop,
    pub triggered: bool,
}

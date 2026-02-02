#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct RepeaterBlock {
    pub delay: i32,
    pub facing: Direction,
    pub locked: bool,
    pub powered: bool,
}

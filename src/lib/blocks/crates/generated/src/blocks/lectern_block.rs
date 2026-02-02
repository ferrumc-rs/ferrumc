#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct LecternBlock {
    pub facing: Direction,
    pub has_book: bool,
    pub powered: bool,
}

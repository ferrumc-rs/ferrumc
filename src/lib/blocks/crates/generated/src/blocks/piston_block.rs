#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum PistonBlockType {
    Piston,
    StickyPiston,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct PistonBlock {
    pub block_type: PistonBlockType,
    pub extended: bool,
    pub facing: Direction,
}

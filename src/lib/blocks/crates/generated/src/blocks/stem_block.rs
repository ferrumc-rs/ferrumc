#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum StemBlockType {
    AttachedMelonStem,
    AttachedPumpkinStem,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct StemBlock {
    pub block_type: StemBlockType,
    pub facing: Direction,
}

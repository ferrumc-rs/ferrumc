#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum SnowyBlockType {
    GrassBlock,
    Mycelium,
    Podzol,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SnowyBlock {
    pub block_type: SnowyBlockType,
    pub snowy: bool,
}

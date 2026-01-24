#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum RedstoneOreBlockType {
    DeepslateRedstoneOre,
    RedstoneOre,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct RedstoneOreBlock {
    pub block_type: RedstoneOreBlockType,
    pub lit: bool,
}

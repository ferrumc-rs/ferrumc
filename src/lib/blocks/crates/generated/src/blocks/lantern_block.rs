#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum LanternBlockType {
    Lantern,
    SoulLantern,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct LanternBlock {
    pub block_type: LanternBlockType,
    pub hanging: bool,
    pub waterlogged: bool,
}

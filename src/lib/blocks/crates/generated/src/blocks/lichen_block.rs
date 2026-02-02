#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum LichenBlockType {
    GlowLichen,
    ResinClump,
    SculkVein,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct LichenBlock {
    pub block_type: LichenBlockType,
    pub down: bool,
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub up: bool,
    pub waterlogged: bool,
    pub west: bool,
}

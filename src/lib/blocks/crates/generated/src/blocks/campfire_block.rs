#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum CampfireBlockType {
    Campfire,
    SoulCampfire,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CampfireBlock {
    pub block_type: CampfireBlockType,
    pub facing: Direction,
    pub lit: bool,
    pub signal_fire: bool,
    pub waterlogged: bool,
}

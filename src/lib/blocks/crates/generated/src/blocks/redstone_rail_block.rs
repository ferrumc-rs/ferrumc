#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum RedstoneRailBlockType {
    ActivatorRail,
    DetectorRail,
    PoweredRail,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct RedstoneRailBlock {
    pub block_type: RedstoneRailBlockType,
    pub powered: bool,
    pub shape: RailShape,
    pub waterlogged: bool,
}

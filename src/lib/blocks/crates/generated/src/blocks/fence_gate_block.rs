#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum FenceGateBlockType {
    AcaciaFenceGate,
    BambooFenceGate,
    BirchFenceGate,
    CherryFenceGate,
    CrimsonFenceGate,
    DarkOakFenceGate,
    JungleFenceGate,
    MangroveFenceGate,
    OakFenceGate,
    PaleOakFenceGate,
    SpruceFenceGate,
    WarpedFenceGate,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct FenceGateBlock {
    pub block_type: FenceGateBlockType,
    pub facing: Direction,
    pub in_wall: bool,
    pub open: bool,
    pub powered: bool,
}

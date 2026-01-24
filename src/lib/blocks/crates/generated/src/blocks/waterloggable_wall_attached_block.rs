#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum WaterloggableWallAttachedBlockType {
    AcaciaWallHangingSign,
    AcaciaWallSign,
    AmethystCluster,
    BambooWallHangingSign,
    BambooWallSign,
    BigDripleafStem,
    BirchWallHangingSign,
    BirchWallSign,
    BrainCoralWallFan,
    BubbleCoralWallFan,
    CherryWallHangingSign,
    CherryWallSign,
    CrimsonWallHangingSign,
    CrimsonWallSign,
    DarkOakWallHangingSign,
    DarkOakWallSign,
    DeadBrainCoralWallFan,
    DeadBubbleCoralWallFan,
    DeadFireCoralWallFan,
    DeadHornCoralWallFan,
    DeadTubeCoralWallFan,
    EnderChest,
    FireCoralWallFan,
    HornCoralWallFan,
    JungleWallHangingSign,
    JungleWallSign,
    Ladder,
    LargeAmethystBud,
    MangroveWallHangingSign,
    MangroveWallSign,
    MediumAmethystBud,
    OakWallHangingSign,
    OakWallSign,
    PaleOakWallHangingSign,
    PaleOakWallSign,
    SmallAmethystBud,
    SpruceWallHangingSign,
    SpruceWallSign,
    TubeCoralWallFan,
    WarpedWallHangingSign,
    WarpedWallSign,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct WaterloggableWallAttachedBlock {
    pub block_type: WaterloggableWallAttachedBlockType,
    pub facing: Direction,
    pub waterlogged: bool,
}

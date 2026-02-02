#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum WaterloggableBlockType {
    Barrier,
    BrainCoral,
    BrainCoralFan,
    BubbleCoral,
    BubbleCoralFan,
    Conduit,
    CopperGrate,
    DeadBrainCoral,
    DeadBrainCoralFan,
    DeadBubbleCoral,
    DeadBubbleCoralFan,
    DeadFireCoral,
    DeadFireCoralFan,
    DeadHornCoral,
    DeadHornCoralFan,
    DeadTubeCoral,
    DeadTubeCoralFan,
    ExposedCopperGrate,
    FireCoral,
    FireCoralFan,
    HangingRoots,
    HeavyCore,
    HornCoral,
    HornCoralFan,
    MangroveRoots,
    OxidizedCopperGrate,
    TubeCoral,
    TubeCoralFan,
    WaxedCopperGrate,
    WaxedExposedCopperGrate,
    WaxedOxidizedCopperGrate,
    WaxedWeatheredCopperGrate,
    WeatheredCopperGrate,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct WaterloggableBlock {
    pub block_type: WaterloggableBlockType,
    pub waterlogged: bool,
}

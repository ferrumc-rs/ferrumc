#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum CandleCakeBlockType {
    BlackCandleCake,
    BlueCandleCake,
    BrownCandleCake,
    CandleCake,
    CyanCandleCake,
    DeepslateRedstoneOre,
    GrayCandleCake,
    GreenCandleCake,
    LightBlueCandleCake,
    LightGrayCandleCake,
    LimeCandleCake,
    MagentaCandleCake,
    OrangeCandleCake,
    PinkCandleCake,
    PurpleCandleCake,
    RedCandleCake,
    RedstoneLamp,
    RedstoneOre,
    RedstoneTorch,
    WhiteCandleCake,
    YellowCandleCake,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CandleCakeBlock {
    pub block_type: CandleCakeBlockType,
    pub lit: bool,
}

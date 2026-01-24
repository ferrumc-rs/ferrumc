#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum FenceAndPaneBlockType {
    AcaciaFence,
    BambooFence,
    BirchFence,
    BlackStainedGlassPane,
    BlueStainedGlassPane,
    BrownStainedGlassPane,
    CherryFence,
    CrimsonFence,
    CyanStainedGlassPane,
    DarkOakFence,
    GlassPane,
    GrayStainedGlassPane,
    GreenStainedGlassPane,
    IronBars,
    JungleFence,
    LightBlueStainedGlassPane,
    LightGrayStainedGlassPane,
    LimeStainedGlassPane,
    MagentaStainedGlassPane,
    MangroveFence,
    NetherBrickFence,
    OakFence,
    OrangeStainedGlassPane,
    PaleOakFence,
    PinkStainedGlassPane,
    PurpleStainedGlassPane,
    RedStainedGlassPane,
    SpruceFence,
    WarpedFence,
    WhiteStainedGlassPane,
    YellowStainedGlassPane,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct FenceAndPaneBlock {
    pub block_type: FenceAndPaneBlockType,
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

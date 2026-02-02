#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum HangingSignBlockType {
    AcaciaHangingSign,
    BambooHangingSign,
    BirchHangingSign,
    CherryHangingSign,
    CrimsonHangingSign,
    DarkOakHangingSign,
    JungleHangingSign,
    MangroveHangingSign,
    OakHangingSign,
    PaleOakHangingSign,
    SpruceHangingSign,
    WarpedHangingSign,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct HangingSignBlock {
    pub block_type: HangingSignBlockType,
    pub attached: bool,
    pub rotation: i32,
    pub waterlogged: bool,
}

#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum TrapdoorBlockType {
    AcaciaTrapdoor,
    BambooTrapdoor,
    BirchTrapdoor,
    CherryTrapdoor,
    CopperTrapdoor,
    CrimsonTrapdoor,
    DarkOakTrapdoor,
    ExposedCopperTrapdoor,
    IronTrapdoor,
    JungleTrapdoor,
    MangroveTrapdoor,
    OakTrapdoor,
    OxidizedCopperTrapdoor,
    PaleOakTrapdoor,
    SpruceTrapdoor,
    WarpedTrapdoor,
    WaxedCopperTrapdoor,
    WaxedExposedCopperTrapdoor,
    WaxedOxidizedCopperTrapdoor,
    WaxedWeatheredCopperTrapdoor,
    WeatheredCopperTrapdoor,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct TrapdoorBlock {
    pub block_type: TrapdoorBlockType,
    pub facing: Direction,
    pub half: Half,
    pub open: bool,
    pub powered: bool,
    pub waterlogged: bool,
}

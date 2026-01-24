#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum PressurePlateBlockType {
    AcaciaPressurePlate,
    BambooPressurePlate,
    BirchPressurePlate,
    CherryPressurePlate,
    CrimsonPressurePlate,
    DarkOakPressurePlate,
    JunglePressurePlate,
    MangrovePressurePlate,
    OakPressurePlate,
    PaleOakPressurePlate,
    PolishedBlackstonePressurePlate,
    SprucePressurePlate,
    StonePressurePlate,
    WarpedPressurePlate,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct PressurePlateBlock {
    pub block_type: PressurePlateBlockType,
    pub powered: bool,
}

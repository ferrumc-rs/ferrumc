#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum DoorBlockType {
    AcaciaDoor,
    BambooDoor,
    BirchDoor,
    CherryDoor,
    CopperDoor,
    CrimsonDoor,
    DarkOakDoor,
    ExposedCopperDoor,
    IronDoor,
    JungleDoor,
    MangroveDoor,
    OakDoor,
    OxidizedCopperDoor,
    PaleOakDoor,
    SpruceDoor,
    WarpedDoor,
    WaxedCopperDoor,
    WaxedExposedCopperDoor,
    WaxedOxidizedCopperDoor,
    WaxedWeatheredCopperDoor,
    WeatheredCopperDoor,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct DoorBlock {
    pub block_type: DoorBlockType,
    pub facing: Direction,
    pub half: DoubleBlockHalf,
    pub hinge: DoorHingeSide,
    pub open: bool,
    pub powered: bool,
}

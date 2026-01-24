#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum ButtonBlockType {
    AcaciaButton,
    BambooButton,
    BirchButton,
    CherryButton,
    CrimsonButton,
    DarkOakButton,
    JungleButton,
    Lever,
    MangroveButton,
    OakButton,
    PaleOakButton,
    PolishedBlackstoneButton,
    SpruceButton,
    StoneButton,
    WarpedButton,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ButtonBlock {
    pub block_type: ButtonBlockType,
    pub face: AttachFace,
    pub facing: Direction,
    pub powered: bool,
}

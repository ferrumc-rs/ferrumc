#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum SignBlockType {
    AcaciaSign,
    BambooSign,
    BirchSign,
    CherrySign,
    CrimsonSign,
    DarkOakSign,
    JungleSign,
    MangroveSign,
    OakSign,
    PaleOakSign,
    SpruceSign,
    WarpedSign,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SignBlock {
    pub block_type: SignBlockType,
    pub rotation: i32,
    pub waterlogged: bool,
}

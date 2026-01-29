#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum SaplingBlockType {
    AcaciaSapling,
    BirchSapling,
    CherrySapling,
    DarkOakSapling,
    JungleSapling,
    OakSapling,
    PaleOakSapling,
    SpruceSapling,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SaplingBlock {
    pub block_type: SaplingBlockType,
    pub stage: i32,
}

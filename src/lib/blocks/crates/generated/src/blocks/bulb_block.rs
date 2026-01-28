#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum BulbBlockType {
    CopperBulb,
    ExposedCopperBulb,
    OxidizedCopperBulb,
    WaxedCopperBulb,
    WaxedExposedCopperBulb,
    WaxedOxidizedCopperBulb,
    WaxedWeatheredCopperBulb,
    WeatheredCopperBulb,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BulbBlock {
    pub block_type: BulbBlockType,
    pub lit: bool,
    pub powered: bool,
}

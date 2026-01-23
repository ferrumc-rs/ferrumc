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
impl TryInto<u32> for BulbBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            BulbBlock {
                block_type: BulbBlockType::CopperBulb,
                lit: true,
                powered: true,
            } => Ok(25720u32),
            BulbBlock {
                block_type: BulbBlockType::CopperBulb,
                lit: true,
                powered: false,
            } => Ok(25721u32),
            BulbBlock {
                block_type: BulbBlockType::CopperBulb,
                lit: false,
                powered: true,
            } => Ok(25722u32),
            BulbBlock {
                block_type: BulbBlockType::CopperBulb,
                lit: false,
                powered: false,
            } => Ok(25723u32),
            BulbBlock {
                block_type: BulbBlockType::ExposedCopperBulb,
                lit: true,
                powered: true,
            } => Ok(25724u32),
            BulbBlock {
                block_type: BulbBlockType::ExposedCopperBulb,
                lit: true,
                powered: false,
            } => Ok(25725u32),
            BulbBlock {
                block_type: BulbBlockType::ExposedCopperBulb,
                lit: false,
                powered: true,
            } => Ok(25726u32),
            BulbBlock {
                block_type: BulbBlockType::ExposedCopperBulb,
                lit: false,
                powered: false,
            } => Ok(25727u32),
            BulbBlock {
                block_type: BulbBlockType::OxidizedCopperBulb,
                lit: true,
                powered: true,
            } => Ok(25732u32),
            BulbBlock {
                block_type: BulbBlockType::OxidizedCopperBulb,
                lit: true,
                powered: false,
            } => Ok(25733u32),
            BulbBlock {
                block_type: BulbBlockType::OxidizedCopperBulb,
                lit: false,
                powered: true,
            } => Ok(25734u32),
            BulbBlock {
                block_type: BulbBlockType::OxidizedCopperBulb,
                lit: false,
                powered: false,
            } => Ok(25735u32),
            BulbBlock {
                block_type: BulbBlockType::WaxedCopperBulb,
                lit: true,
                powered: true,
            } => Ok(25736u32),
            BulbBlock {
                block_type: BulbBlockType::WaxedCopperBulb,
                lit: true,
                powered: false,
            } => Ok(25737u32),
            BulbBlock {
                block_type: BulbBlockType::WaxedCopperBulb,
                lit: false,
                powered: true,
            } => Ok(25738u32),
            BulbBlock {
                block_type: BulbBlockType::WaxedCopperBulb,
                lit: false,
                powered: false,
            } => Ok(25739u32),
            BulbBlock {
                block_type: BulbBlockType::WaxedExposedCopperBulb,
                lit: true,
                powered: true,
            } => Ok(25740u32),
            BulbBlock {
                block_type: BulbBlockType::WaxedExposedCopperBulb,
                lit: true,
                powered: false,
            } => Ok(25741u32),
            BulbBlock {
                block_type: BulbBlockType::WaxedExposedCopperBulb,
                lit: false,
                powered: true,
            } => Ok(25742u32),
            BulbBlock {
                block_type: BulbBlockType::WaxedExposedCopperBulb,
                lit: false,
                powered: false,
            } => Ok(25743u32),
            BulbBlock {
                block_type: BulbBlockType::WaxedOxidizedCopperBulb,
                lit: true,
                powered: true,
            } => Ok(25748u32),
            BulbBlock {
                block_type: BulbBlockType::WaxedOxidizedCopperBulb,
                lit: true,
                powered: false,
            } => Ok(25749u32),
            BulbBlock {
                block_type: BulbBlockType::WaxedOxidizedCopperBulb,
                lit: false,
                powered: true,
            } => Ok(25750u32),
            BulbBlock {
                block_type: BulbBlockType::WaxedOxidizedCopperBulb,
                lit: false,
                powered: false,
            } => Ok(25751u32),
            BulbBlock {
                block_type: BulbBlockType::WaxedWeatheredCopperBulb,
                lit: true,
                powered: true,
            } => Ok(25744u32),
            BulbBlock {
                block_type: BulbBlockType::WaxedWeatheredCopperBulb,
                lit: true,
                powered: false,
            } => Ok(25745u32),
            BulbBlock {
                block_type: BulbBlockType::WaxedWeatheredCopperBulb,
                lit: false,
                powered: true,
            } => Ok(25746u32),
            BulbBlock {
                block_type: BulbBlockType::WaxedWeatheredCopperBulb,
                lit: false,
                powered: false,
            } => Ok(25747u32),
            BulbBlock {
                block_type: BulbBlockType::WeatheredCopperBulb,
                lit: true,
                powered: true,
            } => Ok(25728u32),
            BulbBlock {
                block_type: BulbBlockType::WeatheredCopperBulb,
                lit: true,
                powered: false,
            } => Ok(25729u32),
            BulbBlock {
                block_type: BulbBlockType::WeatheredCopperBulb,
                lit: false,
                powered: true,
            } => Ok(25730u32),
            BulbBlock {
                block_type: BulbBlockType::WeatheredCopperBulb,
                lit: false,
                powered: false,
            } => Ok(25731u32),
            _ => Err(()),
        }
    }
}

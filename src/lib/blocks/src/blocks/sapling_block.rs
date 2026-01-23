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
impl TryInto<u32> for SaplingBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            SaplingBlock {
                block_type: SaplingBlockType::AcaciaSapling,
                stage: 0i32,
            } => Ok(37u32),
            SaplingBlock {
                block_type: SaplingBlockType::AcaciaSapling,
                stage: 1i32,
            } => Ok(38u32),
            SaplingBlock {
                block_type: SaplingBlockType::BirchSapling,
                stage: 0i32,
            } => Ok(33u32),
            SaplingBlock {
                block_type: SaplingBlockType::BirchSapling,
                stage: 1i32,
            } => Ok(34u32),
            SaplingBlock {
                block_type: SaplingBlockType::CherrySapling,
                stage: 0i32,
            } => Ok(39u32),
            SaplingBlock {
                block_type: SaplingBlockType::CherrySapling,
                stage: 1i32,
            } => Ok(40u32),
            SaplingBlock {
                block_type: SaplingBlockType::DarkOakSapling,
                stage: 0i32,
            } => Ok(41u32),
            SaplingBlock {
                block_type: SaplingBlockType::DarkOakSapling,
                stage: 1i32,
            } => Ok(42u32),
            SaplingBlock {
                block_type: SaplingBlockType::JungleSapling,
                stage: 0i32,
            } => Ok(35u32),
            SaplingBlock {
                block_type: SaplingBlockType::JungleSapling,
                stage: 1i32,
            } => Ok(36u32),
            SaplingBlock {
                block_type: SaplingBlockType::OakSapling,
                stage: 0i32,
            } => Ok(29u32),
            SaplingBlock {
                block_type: SaplingBlockType::OakSapling,
                stage: 1i32,
            } => Ok(30u32),
            SaplingBlock {
                block_type: SaplingBlockType::PaleOakSapling,
                stage: 0i32,
            } => Ok(43u32),
            SaplingBlock {
                block_type: SaplingBlockType::PaleOakSapling,
                stage: 1i32,
            } => Ok(44u32),
            SaplingBlock {
                block_type: SaplingBlockType::SpruceSapling,
                stage: 0i32,
            } => Ok(31u32),
            SaplingBlock {
                block_type: SaplingBlockType::SpruceSapling,
                stage: 1i32,
            } => Ok(32u32),
            _ => Err(()),
        }
    }
}

#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
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
pub struct SaplingBlock {
    pub block_type: SaplingBlockType,
    pub stage: i32,
}
impl TryFrom<u32> for SaplingBlock {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            37u32 => Ok(SaplingBlock {
                block_type: SaplingBlockType::AcaciaSapling,
                stage: 0i32,
            }),
            38u32 => Ok(SaplingBlock {
                block_type: SaplingBlockType::AcaciaSapling,
                stage: 1i32,
            }),
            33u32 => Ok(SaplingBlock {
                block_type: SaplingBlockType::BirchSapling,
                stage: 0i32,
            }),
            34u32 => Ok(SaplingBlock {
                block_type: SaplingBlockType::BirchSapling,
                stage: 1i32,
            }),
            39u32 => Ok(SaplingBlock {
                block_type: SaplingBlockType::CherrySapling,
                stage: 0i32,
            }),
            40u32 => Ok(SaplingBlock {
                block_type: SaplingBlockType::CherrySapling,
                stage: 1i32,
            }),
            41u32 => Ok(SaplingBlock {
                block_type: SaplingBlockType::DarkOakSapling,
                stage: 0i32,
            }),
            42u32 => Ok(SaplingBlock {
                block_type: SaplingBlockType::DarkOakSapling,
                stage: 1i32,
            }),
            35u32 => Ok(SaplingBlock {
                block_type: SaplingBlockType::JungleSapling,
                stage: 0i32,
            }),
            36u32 => Ok(SaplingBlock {
                block_type: SaplingBlockType::JungleSapling,
                stage: 1i32,
            }),
            29u32 => Ok(SaplingBlock {
                block_type: SaplingBlockType::OakSapling,
                stage: 0i32,
            }),
            30u32 => Ok(SaplingBlock {
                block_type: SaplingBlockType::OakSapling,
                stage: 1i32,
            }),
            43u32 => Ok(SaplingBlock {
                block_type: SaplingBlockType::PaleOakSapling,
                stage: 0i32,
            }),
            44u32 => Ok(SaplingBlock {
                block_type: SaplingBlockType::PaleOakSapling,
                stage: 1i32,
            }),
            31u32 => Ok(SaplingBlock {
                block_type: SaplingBlockType::SpruceSapling,
                stage: 0i32,
            }),
            32u32 => Ok(SaplingBlock {
                block_type: SaplingBlockType::SpruceSapling,
                stage: 1i32,
            }),
            _ => Err(()),
        }
    }
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

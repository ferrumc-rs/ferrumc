#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum SnowyBlockType {
    GrassBlock,
    Mycelium,
    Podzol,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SnowyBlock {
    pub block_type: SnowyBlockType,
    pub snowy: bool,
}
impl SnowyBlock {
    pub(crate) const VTABLE: crate::BlockBehaviorTable =
        crate::BlockBehaviorTable::from::<SnowyBlock>();
}
impl TryFrom<u32> for SnowyBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            8u32 => Ok(SnowyBlock {
                block_type: SnowyBlockType::GrassBlock,
                snowy: true,
            }),
            9u32 => Ok(SnowyBlock {
                block_type: SnowyBlockType::GrassBlock,
                snowy: false,
            }),
            7640u32 => Ok(SnowyBlock {
                block_type: SnowyBlockType::Mycelium,
                snowy: true,
            }),
            7641u32 => Ok(SnowyBlock {
                block_type: SnowyBlockType::Mycelium,
                snowy: false,
            }),
            12u32 => Ok(SnowyBlock {
                block_type: SnowyBlockType::Podzol,
                snowy: true,
            }),
            13u32 => Ok(SnowyBlock {
                block_type: SnowyBlockType::Podzol,
                snowy: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for SnowyBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            SnowyBlock {
                block_type: SnowyBlockType::GrassBlock,
                snowy: true,
            } => Ok(8u32),
            SnowyBlock {
                block_type: SnowyBlockType::GrassBlock,
                snowy: false,
            } => Ok(9u32),
            SnowyBlock {
                block_type: SnowyBlockType::Mycelium,
                snowy: true,
            } => Ok(7640u32),
            SnowyBlock {
                block_type: SnowyBlockType::Mycelium,
                snowy: false,
            } => Ok(7641u32),
            SnowyBlock {
                block_type: SnowyBlockType::Podzol,
                snowy: true,
            } => Ok(12u32),
            SnowyBlock {
                block_type: SnowyBlockType::Podzol,
                snowy: false,
            } => Ok(13u32),
            _ => Err(()),
        }
    }
}

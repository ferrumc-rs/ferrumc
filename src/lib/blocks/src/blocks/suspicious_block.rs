#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum SuspiciousBlockType {
    SuspiciousGravel,
    SuspiciousSand,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SuspiciousBlock {
    pub block_type: SuspiciousBlockType,
    pub dusted: i32,
}
impl SuspiciousBlock {
    pub(crate) const VTABLE: crate::BlockBehaviorTable =
        crate::BlockBehaviorTable::from::<SuspiciousBlock>();
}
impl TryFrom<u32> for SuspiciousBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            125u32 => Ok(SuspiciousBlock {
                block_type: SuspiciousBlockType::SuspiciousGravel,
                dusted: 0i32,
            }),
            126u32 => Ok(SuspiciousBlock {
                block_type: SuspiciousBlockType::SuspiciousGravel,
                dusted: 1i32,
            }),
            127u32 => Ok(SuspiciousBlock {
                block_type: SuspiciousBlockType::SuspiciousGravel,
                dusted: 2i32,
            }),
            128u32 => Ok(SuspiciousBlock {
                block_type: SuspiciousBlockType::SuspiciousGravel,
                dusted: 3i32,
            }),
            119u32 => Ok(SuspiciousBlock {
                block_type: SuspiciousBlockType::SuspiciousSand,
                dusted: 0i32,
            }),
            120u32 => Ok(SuspiciousBlock {
                block_type: SuspiciousBlockType::SuspiciousSand,
                dusted: 1i32,
            }),
            121u32 => Ok(SuspiciousBlock {
                block_type: SuspiciousBlockType::SuspiciousSand,
                dusted: 2i32,
            }),
            122u32 => Ok(SuspiciousBlock {
                block_type: SuspiciousBlockType::SuspiciousSand,
                dusted: 3i32,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for SuspiciousBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            SuspiciousBlock {
                block_type: SuspiciousBlockType::SuspiciousGravel,
                dusted: 0i32,
            } => Ok(125u32),
            SuspiciousBlock {
                block_type: SuspiciousBlockType::SuspiciousGravel,
                dusted: 1i32,
            } => Ok(126u32),
            SuspiciousBlock {
                block_type: SuspiciousBlockType::SuspiciousGravel,
                dusted: 2i32,
            } => Ok(127u32),
            SuspiciousBlock {
                block_type: SuspiciousBlockType::SuspiciousGravel,
                dusted: 3i32,
            } => Ok(128u32),
            SuspiciousBlock {
                block_type: SuspiciousBlockType::SuspiciousSand,
                dusted: 0i32,
            } => Ok(119u32),
            SuspiciousBlock {
                block_type: SuspiciousBlockType::SuspiciousSand,
                dusted: 1i32,
            } => Ok(120u32),
            SuspiciousBlock {
                block_type: SuspiciousBlockType::SuspiciousSand,
                dusted: 2i32,
            } => Ok(121u32),
            SuspiciousBlock {
                block_type: SuspiciousBlockType::SuspiciousSand,
                dusted: 3i32,
            } => Ok(122u32),
            _ => Err(()),
        }
    }
}

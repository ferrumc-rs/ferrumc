#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum LanternBlockType {
    Lantern,
    SoulLantern,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct LanternBlock {
    pub block_type: LanternBlockType,
    pub hanging: bool,
    pub waterlogged: bool,
}
impl LanternBlock {
    pub(crate) const VTABLE: crate::BlockBehaviorTable =
        crate::BlockBehaviorTable::from::<LanternBlock>();
}
impl TryFrom<u32> for LanternBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            19526u32 => Ok(LanternBlock {
                block_type: LanternBlockType::Lantern,
                hanging: true,
                waterlogged: true,
            }),
            19527u32 => Ok(LanternBlock {
                block_type: LanternBlockType::Lantern,
                hanging: true,
                waterlogged: false,
            }),
            19528u32 => Ok(LanternBlock {
                block_type: LanternBlockType::Lantern,
                hanging: false,
                waterlogged: true,
            }),
            19529u32 => Ok(LanternBlock {
                block_type: LanternBlockType::Lantern,
                hanging: false,
                waterlogged: false,
            }),
            19530u32 => Ok(LanternBlock {
                block_type: LanternBlockType::SoulLantern,
                hanging: true,
                waterlogged: true,
            }),
            19531u32 => Ok(LanternBlock {
                block_type: LanternBlockType::SoulLantern,
                hanging: true,
                waterlogged: false,
            }),
            19532u32 => Ok(LanternBlock {
                block_type: LanternBlockType::SoulLantern,
                hanging: false,
                waterlogged: true,
            }),
            19533u32 => Ok(LanternBlock {
                block_type: LanternBlockType::SoulLantern,
                hanging: false,
                waterlogged: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for LanternBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            LanternBlock {
                block_type: LanternBlockType::Lantern,
                hanging: true,
                waterlogged: true,
            } => Ok(19526u32),
            LanternBlock {
                block_type: LanternBlockType::Lantern,
                hanging: true,
                waterlogged: false,
            } => Ok(19527u32),
            LanternBlock {
                block_type: LanternBlockType::Lantern,
                hanging: false,
                waterlogged: true,
            } => Ok(19528u32),
            LanternBlock {
                block_type: LanternBlockType::Lantern,
                hanging: false,
                waterlogged: false,
            } => Ok(19529u32),
            LanternBlock {
                block_type: LanternBlockType::SoulLantern,
                hanging: true,
                waterlogged: true,
            } => Ok(19530u32),
            LanternBlock {
                block_type: LanternBlockType::SoulLantern,
                hanging: true,
                waterlogged: false,
            } => Ok(19531u32),
            LanternBlock {
                block_type: LanternBlockType::SoulLantern,
                hanging: false,
                waterlogged: true,
            } => Ok(19532u32),
            LanternBlock {
                block_type: LanternBlockType::SoulLantern,
                hanging: false,
                waterlogged: false,
            } => Ok(19533u32),
            _ => Err(()),
        }
    }
}

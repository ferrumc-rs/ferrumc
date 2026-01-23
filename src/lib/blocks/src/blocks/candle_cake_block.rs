#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum CandleCakeBlockType {
    BlackCandleCake,
    BlueCandleCake,
    BrownCandleCake,
    CandleCake,
    CyanCandleCake,
    DeepslateRedstoneOre,
    GrayCandleCake,
    GreenCandleCake,
    LightBlueCandleCake,
    LightGrayCandleCake,
    LimeCandleCake,
    MagentaCandleCake,
    OrangeCandleCake,
    PinkCandleCake,
    PurpleCandleCake,
    RedCandleCake,
    RedstoneLamp,
    RedstoneOre,
    RedstoneTorch,
    WhiteCandleCake,
    YellowCandleCake,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CandleCakeBlock {
    pub block_type: CandleCakeBlockType,
    pub lit: bool,
}
impl CandleCakeBlock {
    pub(crate) const VTABLE: crate::BlockBehaviorTable =
        crate::BlockBehaviorTable::from::<CandleCakeBlock>();
}
impl TryFrom<u32> for CandleCakeBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            22057u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::BlackCandleCake,
                lit: true,
            }),
            22058u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::BlackCandleCake,
                lit: false,
            }),
            22049u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::BlueCandleCake,
                lit: true,
            }),
            22050u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::BlueCandleCake,
                lit: false,
            }),
            22051u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::BrownCandleCake,
                lit: true,
            }),
            22052u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::BrownCandleCake,
                lit: false,
            }),
            22025u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::CandleCake,
                lit: true,
            }),
            22026u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::CandleCake,
                lit: false,
            }),
            22045u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::CyanCandleCake,
                lit: true,
            }),
            22046u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::CyanCandleCake,
                lit: false,
            }),
            5914u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::DeepslateRedstoneOre,
                lit: true,
            }),
            5915u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::DeepslateRedstoneOre,
                lit: false,
            }),
            22041u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::GrayCandleCake,
                lit: true,
            }),
            22042u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::GrayCandleCake,
                lit: false,
            }),
            22053u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::GreenCandleCake,
                lit: true,
            }),
            22054u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::GreenCandleCake,
                lit: false,
            }),
            22033u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::LightBlueCandleCake,
                lit: true,
            }),
            22034u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::LightBlueCandleCake,
                lit: false,
            }),
            22043u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::LightGrayCandleCake,
                lit: true,
            }),
            22044u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::LightGrayCandleCake,
                lit: false,
            }),
            22037u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::LimeCandleCake,
                lit: true,
            }),
            22038u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::LimeCandleCake,
                lit: false,
            }),
            22031u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::MagentaCandleCake,
                lit: true,
            }),
            22032u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::MagentaCandleCake,
                lit: false,
            }),
            22029u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::OrangeCandleCake,
                lit: true,
            }),
            22030u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::OrangeCandleCake,
                lit: false,
            }),
            22039u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::PinkCandleCake,
                lit: true,
            }),
            22040u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::PinkCandleCake,
                lit: false,
            }),
            22047u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::PurpleCandleCake,
                lit: true,
            }),
            22048u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::PurpleCandleCake,
                lit: false,
            }),
            22055u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::RedCandleCake,
                lit: true,
            }),
            22056u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::RedCandleCake,
                lit: false,
            }),
            8201u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::RedstoneLamp,
                lit: true,
            }),
            8202u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::RedstoneLamp,
                lit: false,
            }),
            5912u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::RedstoneOre,
                lit: true,
            }),
            5913u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::RedstoneOre,
                lit: false,
            }),
            5916u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::RedstoneTorch,
                lit: true,
            }),
            5917u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::RedstoneTorch,
                lit: false,
            }),
            22027u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::WhiteCandleCake,
                lit: true,
            }),
            22028u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::WhiteCandleCake,
                lit: false,
            }),
            22035u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::YellowCandleCake,
                lit: true,
            }),
            22036u32 => Ok(CandleCakeBlock {
                block_type: CandleCakeBlockType::YellowCandleCake,
                lit: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for CandleCakeBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            CandleCakeBlock {
                block_type: CandleCakeBlockType::BlackCandleCake,
                lit: true,
            } => Ok(22057u32),
            CandleCakeBlock {
                block_type: CandleCakeBlockType::BlackCandleCake,
                lit: false,
            } => Ok(22058u32),
            CandleCakeBlock {
                block_type: CandleCakeBlockType::BlueCandleCake,
                lit: true,
            } => Ok(22049u32),
            CandleCakeBlock {
                block_type: CandleCakeBlockType::BlueCandleCake,
                lit: false,
            } => Ok(22050u32),
            CandleCakeBlock {
                block_type: CandleCakeBlockType::BrownCandleCake,
                lit: true,
            } => Ok(22051u32),
            CandleCakeBlock {
                block_type: CandleCakeBlockType::BrownCandleCake,
                lit: false,
            } => Ok(22052u32),
            CandleCakeBlock {
                block_type: CandleCakeBlockType::CandleCake,
                lit: true,
            } => Ok(22025u32),
            CandleCakeBlock {
                block_type: CandleCakeBlockType::CandleCake,
                lit: false,
            } => Ok(22026u32),
            CandleCakeBlock {
                block_type: CandleCakeBlockType::CyanCandleCake,
                lit: true,
            } => Ok(22045u32),
            CandleCakeBlock {
                block_type: CandleCakeBlockType::CyanCandleCake,
                lit: false,
            } => Ok(22046u32),
            CandleCakeBlock {
                block_type: CandleCakeBlockType::DeepslateRedstoneOre,
                lit: true,
            } => Ok(5914u32),
            CandleCakeBlock {
                block_type: CandleCakeBlockType::DeepslateRedstoneOre,
                lit: false,
            } => Ok(5915u32),
            CandleCakeBlock {
                block_type: CandleCakeBlockType::GrayCandleCake,
                lit: true,
            } => Ok(22041u32),
            CandleCakeBlock {
                block_type: CandleCakeBlockType::GrayCandleCake,
                lit: false,
            } => Ok(22042u32),
            CandleCakeBlock {
                block_type: CandleCakeBlockType::GreenCandleCake,
                lit: true,
            } => Ok(22053u32),
            CandleCakeBlock {
                block_type: CandleCakeBlockType::GreenCandleCake,
                lit: false,
            } => Ok(22054u32),
            CandleCakeBlock {
                block_type: CandleCakeBlockType::LightBlueCandleCake,
                lit: true,
            } => Ok(22033u32),
            CandleCakeBlock {
                block_type: CandleCakeBlockType::LightBlueCandleCake,
                lit: false,
            } => Ok(22034u32),
            CandleCakeBlock {
                block_type: CandleCakeBlockType::LightGrayCandleCake,
                lit: true,
            } => Ok(22043u32),
            CandleCakeBlock {
                block_type: CandleCakeBlockType::LightGrayCandleCake,
                lit: false,
            } => Ok(22044u32),
            CandleCakeBlock {
                block_type: CandleCakeBlockType::LimeCandleCake,
                lit: true,
            } => Ok(22037u32),
            CandleCakeBlock {
                block_type: CandleCakeBlockType::LimeCandleCake,
                lit: false,
            } => Ok(22038u32),
            CandleCakeBlock {
                block_type: CandleCakeBlockType::MagentaCandleCake,
                lit: true,
            } => Ok(22031u32),
            CandleCakeBlock {
                block_type: CandleCakeBlockType::MagentaCandleCake,
                lit: false,
            } => Ok(22032u32),
            CandleCakeBlock {
                block_type: CandleCakeBlockType::OrangeCandleCake,
                lit: true,
            } => Ok(22029u32),
            CandleCakeBlock {
                block_type: CandleCakeBlockType::OrangeCandleCake,
                lit: false,
            } => Ok(22030u32),
            CandleCakeBlock {
                block_type: CandleCakeBlockType::PinkCandleCake,
                lit: true,
            } => Ok(22039u32),
            CandleCakeBlock {
                block_type: CandleCakeBlockType::PinkCandleCake,
                lit: false,
            } => Ok(22040u32),
            CandleCakeBlock {
                block_type: CandleCakeBlockType::PurpleCandleCake,
                lit: true,
            } => Ok(22047u32),
            CandleCakeBlock {
                block_type: CandleCakeBlockType::PurpleCandleCake,
                lit: false,
            } => Ok(22048u32),
            CandleCakeBlock {
                block_type: CandleCakeBlockType::RedCandleCake,
                lit: true,
            } => Ok(22055u32),
            CandleCakeBlock {
                block_type: CandleCakeBlockType::RedCandleCake,
                lit: false,
            } => Ok(22056u32),
            CandleCakeBlock {
                block_type: CandleCakeBlockType::RedstoneLamp,
                lit: true,
            } => Ok(8201u32),
            CandleCakeBlock {
                block_type: CandleCakeBlockType::RedstoneLamp,
                lit: false,
            } => Ok(8202u32),
            CandleCakeBlock {
                block_type: CandleCakeBlockType::RedstoneOre,
                lit: true,
            } => Ok(5912u32),
            CandleCakeBlock {
                block_type: CandleCakeBlockType::RedstoneOre,
                lit: false,
            } => Ok(5913u32),
            CandleCakeBlock {
                block_type: CandleCakeBlockType::RedstoneTorch,
                lit: true,
            } => Ok(5916u32),
            CandleCakeBlock {
                block_type: CandleCakeBlockType::RedstoneTorch,
                lit: false,
            } => Ok(5917u32),
            CandleCakeBlock {
                block_type: CandleCakeBlockType::WhiteCandleCake,
                lit: true,
            } => Ok(22027u32),
            CandleCakeBlock {
                block_type: CandleCakeBlockType::WhiteCandleCake,
                lit: false,
            } => Ok(22028u32),
            CandleCakeBlock {
                block_type: CandleCakeBlockType::YellowCandleCake,
                lit: true,
            } => Ok(22035u32),
            CandleCakeBlock {
                block_type: CandleCakeBlockType::YellowCandleCake,
                lit: false,
            } => Ok(22036u32),
            _ => Err(()),
        }
    }
}

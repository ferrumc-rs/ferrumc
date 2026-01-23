#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub enum GeneratedStruct77Type {
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
pub struct GeneratedStruct77 {
    pub block_type: GeneratedStruct77Type,
    pub lit: bool,
}
impl TryFrom<u32> for GeneratedStruct77 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            22057u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::BlackCandleCake,
                lit: true,
            }),
            22058u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::BlackCandleCake,
                lit: false,
            }),
            22049u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::BlueCandleCake,
                lit: true,
            }),
            22050u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::BlueCandleCake,
                lit: false,
            }),
            22051u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::BrownCandleCake,
                lit: true,
            }),
            22052u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::BrownCandleCake,
                lit: false,
            }),
            22025u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::CandleCake,
                lit: true,
            }),
            22026u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::CandleCake,
                lit: false,
            }),
            22045u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::CyanCandleCake,
                lit: true,
            }),
            22046u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::CyanCandleCake,
                lit: false,
            }),
            5914u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::DeepslateRedstoneOre,
                lit: true,
            }),
            5915u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::DeepslateRedstoneOre,
                lit: false,
            }),
            22041u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::GrayCandleCake,
                lit: true,
            }),
            22042u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::GrayCandleCake,
                lit: false,
            }),
            22053u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::GreenCandleCake,
                lit: true,
            }),
            22054u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::GreenCandleCake,
                lit: false,
            }),
            22033u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::LightBlueCandleCake,
                lit: true,
            }),
            22034u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::LightBlueCandleCake,
                lit: false,
            }),
            22043u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::LightGrayCandleCake,
                lit: true,
            }),
            22044u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::LightGrayCandleCake,
                lit: false,
            }),
            22037u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::LimeCandleCake,
                lit: true,
            }),
            22038u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::LimeCandleCake,
                lit: false,
            }),
            22031u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::MagentaCandleCake,
                lit: true,
            }),
            22032u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::MagentaCandleCake,
                lit: false,
            }),
            22029u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::OrangeCandleCake,
                lit: true,
            }),
            22030u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::OrangeCandleCake,
                lit: false,
            }),
            22039u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::PinkCandleCake,
                lit: true,
            }),
            22040u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::PinkCandleCake,
                lit: false,
            }),
            22047u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::PurpleCandleCake,
                lit: true,
            }),
            22048u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::PurpleCandleCake,
                lit: false,
            }),
            22055u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::RedCandleCake,
                lit: true,
            }),
            22056u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::RedCandleCake,
                lit: false,
            }),
            8201u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::RedstoneLamp,
                lit: true,
            }),
            8202u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::RedstoneLamp,
                lit: false,
            }),
            5912u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::RedstoneOre,
                lit: true,
            }),
            5913u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::RedstoneOre,
                lit: false,
            }),
            5916u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::RedstoneTorch,
                lit: true,
            }),
            5917u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::RedstoneTorch,
                lit: false,
            }),
            22027u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::WhiteCandleCake,
                lit: true,
            }),
            22028u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::WhiteCandleCake,
                lit: false,
            }),
            22035u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::YellowCandleCake,
                lit: true,
            }),
            22036u32 => Ok(GeneratedStruct77 {
                block_type: GeneratedStruct77Type::YellowCandleCake,
                lit: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct77 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::BlackCandleCake,
                lit: true,
            } => Ok(22057u32),
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::BlackCandleCake,
                lit: false,
            } => Ok(22058u32),
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::BlueCandleCake,
                lit: true,
            } => Ok(22049u32),
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::BlueCandleCake,
                lit: false,
            } => Ok(22050u32),
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::BrownCandleCake,
                lit: true,
            } => Ok(22051u32),
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::BrownCandleCake,
                lit: false,
            } => Ok(22052u32),
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::CandleCake,
                lit: true,
            } => Ok(22025u32),
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::CandleCake,
                lit: false,
            } => Ok(22026u32),
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::CyanCandleCake,
                lit: true,
            } => Ok(22045u32),
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::CyanCandleCake,
                lit: false,
            } => Ok(22046u32),
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::DeepslateRedstoneOre,
                lit: true,
            } => Ok(5914u32),
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::DeepslateRedstoneOre,
                lit: false,
            } => Ok(5915u32),
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::GrayCandleCake,
                lit: true,
            } => Ok(22041u32),
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::GrayCandleCake,
                lit: false,
            } => Ok(22042u32),
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::GreenCandleCake,
                lit: true,
            } => Ok(22053u32),
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::GreenCandleCake,
                lit: false,
            } => Ok(22054u32),
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::LightBlueCandleCake,
                lit: true,
            } => Ok(22033u32),
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::LightBlueCandleCake,
                lit: false,
            } => Ok(22034u32),
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::LightGrayCandleCake,
                lit: true,
            } => Ok(22043u32),
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::LightGrayCandleCake,
                lit: false,
            } => Ok(22044u32),
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::LimeCandleCake,
                lit: true,
            } => Ok(22037u32),
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::LimeCandleCake,
                lit: false,
            } => Ok(22038u32),
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::MagentaCandleCake,
                lit: true,
            } => Ok(22031u32),
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::MagentaCandleCake,
                lit: false,
            } => Ok(22032u32),
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::OrangeCandleCake,
                lit: true,
            } => Ok(22029u32),
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::OrangeCandleCake,
                lit: false,
            } => Ok(22030u32),
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::PinkCandleCake,
                lit: true,
            } => Ok(22039u32),
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::PinkCandleCake,
                lit: false,
            } => Ok(22040u32),
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::PurpleCandleCake,
                lit: true,
            } => Ok(22047u32),
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::PurpleCandleCake,
                lit: false,
            } => Ok(22048u32),
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::RedCandleCake,
                lit: true,
            } => Ok(22055u32),
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::RedCandleCake,
                lit: false,
            } => Ok(22056u32),
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::RedstoneLamp,
                lit: true,
            } => Ok(8201u32),
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::RedstoneLamp,
                lit: false,
            } => Ok(8202u32),
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::RedstoneOre,
                lit: true,
            } => Ok(5912u32),
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::RedstoneOre,
                lit: false,
            } => Ok(5913u32),
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::RedstoneTorch,
                lit: true,
            } => Ok(5916u32),
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::RedstoneTorch,
                lit: false,
            } => Ok(5917u32),
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::WhiteCandleCake,
                lit: true,
            } => Ok(22027u32),
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::WhiteCandleCake,
                lit: false,
            } => Ok(22028u32),
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::YellowCandleCake,
                lit: true,
            } => Ok(22035u32),
            GeneratedStruct77 {
                block_type: GeneratedStruct77Type::YellowCandleCake,
                lit: false,
            } => Ok(22036u32),
            _ => Err(()),
        }
    }
}

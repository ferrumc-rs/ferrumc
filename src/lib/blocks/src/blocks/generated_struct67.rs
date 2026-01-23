#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub enum GeneratedStruct67Type {
    LargeFern,
    Lilac,
    Peony,
    PitcherPlant,
    RoseBush,
    Sunflower,
    TallGrass,
    TallSeagrass,
}
#[allow(dead_code)]
pub struct GeneratedStruct67 {
    pub block_type: GeneratedStruct67Type,
    pub half: DoubleBlockHalf,
}
impl TryFrom<u32> for GeneratedStruct67 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            11646u32 => Ok(GeneratedStruct67 {
                block_type: GeneratedStruct67Type::LargeFern,
                half: DoubleBlockHalf::Upper,
            }),
            11647u32 => Ok(GeneratedStruct67 {
                block_type: GeneratedStruct67Type::LargeFern,
                half: DoubleBlockHalf::Lower,
            }),
            11638u32 => Ok(GeneratedStruct67 {
                block_type: GeneratedStruct67Type::Lilac,
                half: DoubleBlockHalf::Upper,
            }),
            11639u32 => Ok(GeneratedStruct67 {
                block_type: GeneratedStruct67Type::Lilac,
                half: DoubleBlockHalf::Lower,
            }),
            11642u32 => Ok(GeneratedStruct67 {
                block_type: GeneratedStruct67Type::Peony,
                half: DoubleBlockHalf::Upper,
            }),
            11643u32 => Ok(GeneratedStruct67 {
                block_type: GeneratedStruct67Type::Peony,
                half: DoubleBlockHalf::Lower,
            }),
            13530u32 => Ok(GeneratedStruct67 {
                block_type: GeneratedStruct67Type::PitcherPlant,
                half: DoubleBlockHalf::Upper,
            }),
            13531u32 => Ok(GeneratedStruct67 {
                block_type: GeneratedStruct67Type::PitcherPlant,
                half: DoubleBlockHalf::Lower,
            }),
            11640u32 => Ok(GeneratedStruct67 {
                block_type: GeneratedStruct67Type::RoseBush,
                half: DoubleBlockHalf::Upper,
            }),
            11641u32 => Ok(GeneratedStruct67 {
                block_type: GeneratedStruct67Type::RoseBush,
                half: DoubleBlockHalf::Lower,
            }),
            11636u32 => Ok(GeneratedStruct67 {
                block_type: GeneratedStruct67Type::Sunflower,
                half: DoubleBlockHalf::Upper,
            }),
            11637u32 => Ok(GeneratedStruct67 {
                block_type: GeneratedStruct67Type::Sunflower,
                half: DoubleBlockHalf::Lower,
            }),
            11644u32 => Ok(GeneratedStruct67 {
                block_type: GeneratedStruct67Type::TallGrass,
                half: DoubleBlockHalf::Upper,
            }),
            11645u32 => Ok(GeneratedStruct67 {
                block_type: GeneratedStruct67Type::TallGrass,
                half: DoubleBlockHalf::Lower,
            }),
            2055u32 => Ok(GeneratedStruct67 {
                block_type: GeneratedStruct67Type::TallSeagrass,
                half: DoubleBlockHalf::Upper,
            }),
            2056u32 => Ok(GeneratedStruct67 {
                block_type: GeneratedStruct67Type::TallSeagrass,
                half: DoubleBlockHalf::Lower,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct67 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct67 {
                block_type: GeneratedStruct67Type::LargeFern,
                half: DoubleBlockHalf::Upper,
            } => Ok(11646u32),
            GeneratedStruct67 {
                block_type: GeneratedStruct67Type::LargeFern,
                half: DoubleBlockHalf::Lower,
            } => Ok(11647u32),
            GeneratedStruct67 {
                block_type: GeneratedStruct67Type::Lilac,
                half: DoubleBlockHalf::Upper,
            } => Ok(11638u32),
            GeneratedStruct67 {
                block_type: GeneratedStruct67Type::Lilac,
                half: DoubleBlockHalf::Lower,
            } => Ok(11639u32),
            GeneratedStruct67 {
                block_type: GeneratedStruct67Type::Peony,
                half: DoubleBlockHalf::Upper,
            } => Ok(11642u32),
            GeneratedStruct67 {
                block_type: GeneratedStruct67Type::Peony,
                half: DoubleBlockHalf::Lower,
            } => Ok(11643u32),
            GeneratedStruct67 {
                block_type: GeneratedStruct67Type::PitcherPlant,
                half: DoubleBlockHalf::Upper,
            } => Ok(13530u32),
            GeneratedStruct67 {
                block_type: GeneratedStruct67Type::PitcherPlant,
                half: DoubleBlockHalf::Lower,
            } => Ok(13531u32),
            GeneratedStruct67 {
                block_type: GeneratedStruct67Type::RoseBush,
                half: DoubleBlockHalf::Upper,
            } => Ok(11640u32),
            GeneratedStruct67 {
                block_type: GeneratedStruct67Type::RoseBush,
                half: DoubleBlockHalf::Lower,
            } => Ok(11641u32),
            GeneratedStruct67 {
                block_type: GeneratedStruct67Type::Sunflower,
                half: DoubleBlockHalf::Upper,
            } => Ok(11636u32),
            GeneratedStruct67 {
                block_type: GeneratedStruct67Type::Sunflower,
                half: DoubleBlockHalf::Lower,
            } => Ok(11637u32),
            GeneratedStruct67 {
                block_type: GeneratedStruct67Type::TallGrass,
                half: DoubleBlockHalf::Upper,
            } => Ok(11644u32),
            GeneratedStruct67 {
                block_type: GeneratedStruct67Type::TallGrass,
                half: DoubleBlockHalf::Lower,
            } => Ok(11645u32),
            GeneratedStruct67 {
                block_type: GeneratedStruct67Type::TallSeagrass,
                half: DoubleBlockHalf::Upper,
            } => Ok(2055u32),
            GeneratedStruct67 {
                block_type: GeneratedStruct67Type::TallSeagrass,
                half: DoubleBlockHalf::Lower,
            } => Ok(2056u32),
            _ => Err(()),
        }
    }
}

#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum DoublePlantBlockType {
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
#[derive(Clone, Debug)]
pub struct DoublePlantBlock {
    pub block_type: DoublePlantBlockType,
    pub half: DoubleBlockHalf,
}
impl DoublePlantBlock {
    pub(crate) const VTABLE: crate::BlockBehaviorTable =
        crate::BlockBehaviorTable::from::<DoublePlantBlock>();
}
impl TryFrom<u32> for DoublePlantBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            11646u32 => Ok(DoublePlantBlock {
                block_type: DoublePlantBlockType::LargeFern,
                half: DoubleBlockHalf::Upper,
            }),
            11647u32 => Ok(DoublePlantBlock {
                block_type: DoublePlantBlockType::LargeFern,
                half: DoubleBlockHalf::Lower,
            }),
            11638u32 => Ok(DoublePlantBlock {
                block_type: DoublePlantBlockType::Lilac,
                half: DoubleBlockHalf::Upper,
            }),
            11639u32 => Ok(DoublePlantBlock {
                block_type: DoublePlantBlockType::Lilac,
                half: DoubleBlockHalf::Lower,
            }),
            11642u32 => Ok(DoublePlantBlock {
                block_type: DoublePlantBlockType::Peony,
                half: DoubleBlockHalf::Upper,
            }),
            11643u32 => Ok(DoublePlantBlock {
                block_type: DoublePlantBlockType::Peony,
                half: DoubleBlockHalf::Lower,
            }),
            13530u32 => Ok(DoublePlantBlock {
                block_type: DoublePlantBlockType::PitcherPlant,
                half: DoubleBlockHalf::Upper,
            }),
            13531u32 => Ok(DoublePlantBlock {
                block_type: DoublePlantBlockType::PitcherPlant,
                half: DoubleBlockHalf::Lower,
            }),
            11640u32 => Ok(DoublePlantBlock {
                block_type: DoublePlantBlockType::RoseBush,
                half: DoubleBlockHalf::Upper,
            }),
            11641u32 => Ok(DoublePlantBlock {
                block_type: DoublePlantBlockType::RoseBush,
                half: DoubleBlockHalf::Lower,
            }),
            11636u32 => Ok(DoublePlantBlock {
                block_type: DoublePlantBlockType::Sunflower,
                half: DoubleBlockHalf::Upper,
            }),
            11637u32 => Ok(DoublePlantBlock {
                block_type: DoublePlantBlockType::Sunflower,
                half: DoubleBlockHalf::Lower,
            }),
            11644u32 => Ok(DoublePlantBlock {
                block_type: DoublePlantBlockType::TallGrass,
                half: DoubleBlockHalf::Upper,
            }),
            11645u32 => Ok(DoublePlantBlock {
                block_type: DoublePlantBlockType::TallGrass,
                half: DoubleBlockHalf::Lower,
            }),
            2055u32 => Ok(DoublePlantBlock {
                block_type: DoublePlantBlockType::TallSeagrass,
                half: DoubleBlockHalf::Upper,
            }),
            2056u32 => Ok(DoublePlantBlock {
                block_type: DoublePlantBlockType::TallSeagrass,
                half: DoubleBlockHalf::Lower,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for DoublePlantBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            DoublePlantBlock {
                block_type: DoublePlantBlockType::LargeFern,
                half: DoubleBlockHalf::Upper,
            } => Ok(11646u32),
            DoublePlantBlock {
                block_type: DoublePlantBlockType::LargeFern,
                half: DoubleBlockHalf::Lower,
            } => Ok(11647u32),
            DoublePlantBlock {
                block_type: DoublePlantBlockType::Lilac,
                half: DoubleBlockHalf::Upper,
            } => Ok(11638u32),
            DoublePlantBlock {
                block_type: DoublePlantBlockType::Lilac,
                half: DoubleBlockHalf::Lower,
            } => Ok(11639u32),
            DoublePlantBlock {
                block_type: DoublePlantBlockType::Peony,
                half: DoubleBlockHalf::Upper,
            } => Ok(11642u32),
            DoublePlantBlock {
                block_type: DoublePlantBlockType::Peony,
                half: DoubleBlockHalf::Lower,
            } => Ok(11643u32),
            DoublePlantBlock {
                block_type: DoublePlantBlockType::PitcherPlant,
                half: DoubleBlockHalf::Upper,
            } => Ok(13530u32),
            DoublePlantBlock {
                block_type: DoublePlantBlockType::PitcherPlant,
                half: DoubleBlockHalf::Lower,
            } => Ok(13531u32),
            DoublePlantBlock {
                block_type: DoublePlantBlockType::RoseBush,
                half: DoubleBlockHalf::Upper,
            } => Ok(11640u32),
            DoublePlantBlock {
                block_type: DoublePlantBlockType::RoseBush,
                half: DoubleBlockHalf::Lower,
            } => Ok(11641u32),
            DoublePlantBlock {
                block_type: DoublePlantBlockType::Sunflower,
                half: DoubleBlockHalf::Upper,
            } => Ok(11636u32),
            DoublePlantBlock {
                block_type: DoublePlantBlockType::Sunflower,
                half: DoubleBlockHalf::Lower,
            } => Ok(11637u32),
            DoublePlantBlock {
                block_type: DoublePlantBlockType::TallGrass,
                half: DoubleBlockHalf::Upper,
            } => Ok(11644u32),
            DoublePlantBlock {
                block_type: DoublePlantBlockType::TallGrass,
                half: DoubleBlockHalf::Lower,
            } => Ok(11645u32),
            DoublePlantBlock {
                block_type: DoublePlantBlockType::TallSeagrass,
                half: DoubleBlockHalf::Upper,
            } => Ok(2055u32),
            DoublePlantBlock {
                block_type: DoublePlantBlockType::TallSeagrass,
                half: DoubleBlockHalf::Lower,
            } => Ok(2056u32),
            _ => Err(()),
        }
    }
}

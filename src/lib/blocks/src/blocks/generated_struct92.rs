#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub struct GeneratedStruct92 {
    pub shape: RailShape,
    pub waterlogged: bool,
}
impl TryFrom<u32> for GeneratedStruct92 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            4758u32 => Ok(GeneratedStruct92 {
                shape: RailShape::NorthSouth,
                waterlogged: true,
            }),
            4759u32 => Ok(GeneratedStruct92 {
                shape: RailShape::NorthSouth,
                waterlogged: false,
            }),
            4760u32 => Ok(GeneratedStruct92 {
                shape: RailShape::EastWest,
                waterlogged: true,
            }),
            4761u32 => Ok(GeneratedStruct92 {
                waterlogged: false,
                shape: RailShape::EastWest,
            }),
            4762u32 => Ok(GeneratedStruct92 {
                waterlogged: true,
                shape: RailShape::AscendingEast,
            }),
            4763u32 => Ok(GeneratedStruct92 {
                shape: RailShape::AscendingEast,
                waterlogged: false,
            }),
            4764u32 => Ok(GeneratedStruct92 {
                waterlogged: true,
                shape: RailShape::AscendingWest,
            }),
            4765u32 => Ok(GeneratedStruct92 {
                waterlogged: false,
                shape: RailShape::AscendingWest,
            }),
            4766u32 => Ok(GeneratedStruct92 {
                waterlogged: true,
                shape: RailShape::AscendingNorth,
            }),
            4767u32 => Ok(GeneratedStruct92 {
                waterlogged: false,
                shape: RailShape::AscendingNorth,
            }),
            4768u32 => Ok(GeneratedStruct92 {
                waterlogged: true,
                shape: RailShape::AscendingSouth,
            }),
            4769u32 => Ok(GeneratedStruct92 {
                shape: RailShape::AscendingSouth,
                waterlogged: false,
            }),
            4770u32 => Ok(GeneratedStruct92 {
                waterlogged: true,
                shape: RailShape::SouthEast,
            }),
            4771u32 => Ok(GeneratedStruct92 {
                shape: RailShape::SouthEast,
                waterlogged: false,
            }),
            4772u32 => Ok(GeneratedStruct92 {
                shape: RailShape::SouthWest,
                waterlogged: true,
            }),
            4773u32 => Ok(GeneratedStruct92 {
                shape: RailShape::SouthWest,
                waterlogged: false,
            }),
            4774u32 => Ok(GeneratedStruct92 {
                shape: RailShape::NorthWest,
                waterlogged: true,
            }),
            4775u32 => Ok(GeneratedStruct92 {
                waterlogged: false,
                shape: RailShape::NorthWest,
            }),
            4776u32 => Ok(GeneratedStruct92 {
                shape: RailShape::NorthEast,
                waterlogged: true,
            }),
            4777u32 => Ok(GeneratedStruct92 {
                waterlogged: false,
                shape: RailShape::NorthEast,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct92 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct92 {
                shape: RailShape::NorthSouth,
                waterlogged: true,
            } => Ok(4758u32),
            GeneratedStruct92 {
                shape: RailShape::NorthSouth,
                waterlogged: false,
            } => Ok(4759u32),
            GeneratedStruct92 {
                shape: RailShape::EastWest,
                waterlogged: true,
            } => Ok(4760u32),
            GeneratedStruct92 {
                waterlogged: false,
                shape: RailShape::EastWest,
            } => Ok(4761u32),
            GeneratedStruct92 {
                waterlogged: true,
                shape: RailShape::AscendingEast,
            } => Ok(4762u32),
            GeneratedStruct92 {
                shape: RailShape::AscendingEast,
                waterlogged: false,
            } => Ok(4763u32),
            GeneratedStruct92 {
                waterlogged: true,
                shape: RailShape::AscendingWest,
            } => Ok(4764u32),
            GeneratedStruct92 {
                waterlogged: false,
                shape: RailShape::AscendingWest,
            } => Ok(4765u32),
            GeneratedStruct92 {
                waterlogged: true,
                shape: RailShape::AscendingNorth,
            } => Ok(4766u32),
            GeneratedStruct92 {
                waterlogged: false,
                shape: RailShape::AscendingNorth,
            } => Ok(4767u32),
            GeneratedStruct92 {
                waterlogged: true,
                shape: RailShape::AscendingSouth,
            } => Ok(4768u32),
            GeneratedStruct92 {
                shape: RailShape::AscendingSouth,
                waterlogged: false,
            } => Ok(4769u32),
            GeneratedStruct92 {
                waterlogged: true,
                shape: RailShape::SouthEast,
            } => Ok(4770u32),
            GeneratedStruct92 {
                shape: RailShape::SouthEast,
                waterlogged: false,
            } => Ok(4771u32),
            GeneratedStruct92 {
                shape: RailShape::SouthWest,
                waterlogged: true,
            } => Ok(4772u32),
            GeneratedStruct92 {
                shape: RailShape::SouthWest,
                waterlogged: false,
            } => Ok(4773u32),
            GeneratedStruct92 {
                shape: RailShape::NorthWest,
                waterlogged: true,
            } => Ok(4774u32),
            GeneratedStruct92 {
                waterlogged: false,
                shape: RailShape::NorthWest,
            } => Ok(4775u32),
            GeneratedStruct92 {
                shape: RailShape::NorthEast,
                waterlogged: true,
            } => Ok(4776u32),
            GeneratedStruct92 {
                waterlogged: false,
                shape: RailShape::NorthEast,
            } => Ok(4777u32),
            _ => Err(()),
        }
    }
}

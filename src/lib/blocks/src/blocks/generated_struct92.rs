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
                waterlogged: true,
                shape: RailShape::NorthSouth,
            }),
            4759u32 => Ok(GeneratedStruct92 {
                waterlogged: false,
                shape: RailShape::NorthSouth,
            }),
            4760u32 => Ok(GeneratedStruct92 {
                waterlogged: true,
                shape: RailShape::EastWest,
            }),
            4761u32 => Ok(GeneratedStruct92 {
                shape: RailShape::EastWest,
                waterlogged: false,
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
                shape: RailShape::AscendingWest,
                waterlogged: true,
            }),
            4765u32 => Ok(GeneratedStruct92 {
                waterlogged: false,
                shape: RailShape::AscendingWest,
            }),
            4766u32 => Ok(GeneratedStruct92 {
                shape: RailShape::AscendingNorth,
                waterlogged: true,
            }),
            4767u32 => Ok(GeneratedStruct92 {
                shape: RailShape::AscendingNorth,
                waterlogged: false,
            }),
            4768u32 => Ok(GeneratedStruct92 {
                shape: RailShape::AscendingSouth,
                waterlogged: true,
            }),
            4769u32 => Ok(GeneratedStruct92 {
                waterlogged: false,
                shape: RailShape::AscendingSouth,
            }),
            4770u32 => Ok(GeneratedStruct92 {
                waterlogged: true,
                shape: RailShape::SouthEast,
            }),
            4771u32 => Ok(GeneratedStruct92 {
                waterlogged: false,
                shape: RailShape::SouthEast,
            }),
            4772u32 => Ok(GeneratedStruct92 {
                shape: RailShape::SouthWest,
                waterlogged: true,
            }),
            4773u32 => Ok(GeneratedStruct92 {
                waterlogged: false,
                shape: RailShape::SouthWest,
            }),
            4774u32 => Ok(GeneratedStruct92 {
                waterlogged: true,
                shape: RailShape::NorthWest,
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
                shape: RailShape::NorthEast,
                waterlogged: false,
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
                waterlogged: true,
                shape: RailShape::NorthSouth,
            } => Ok(4758u32),
            GeneratedStruct92 {
                waterlogged: false,
                shape: RailShape::NorthSouth,
            } => Ok(4759u32),
            GeneratedStruct92 {
                waterlogged: true,
                shape: RailShape::EastWest,
            } => Ok(4760u32),
            GeneratedStruct92 {
                shape: RailShape::EastWest,
                waterlogged: false,
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
                shape: RailShape::AscendingWest,
                waterlogged: true,
            } => Ok(4764u32),
            GeneratedStruct92 {
                waterlogged: false,
                shape: RailShape::AscendingWest,
            } => Ok(4765u32),
            GeneratedStruct92 {
                shape: RailShape::AscendingNorth,
                waterlogged: true,
            } => Ok(4766u32),
            GeneratedStruct92 {
                shape: RailShape::AscendingNorth,
                waterlogged: false,
            } => Ok(4767u32),
            GeneratedStruct92 {
                shape: RailShape::AscendingSouth,
                waterlogged: true,
            } => Ok(4768u32),
            GeneratedStruct92 {
                waterlogged: false,
                shape: RailShape::AscendingSouth,
            } => Ok(4769u32),
            GeneratedStruct92 {
                waterlogged: true,
                shape: RailShape::SouthEast,
            } => Ok(4770u32),
            GeneratedStruct92 {
                waterlogged: false,
                shape: RailShape::SouthEast,
            } => Ok(4771u32),
            GeneratedStruct92 {
                shape: RailShape::SouthWest,
                waterlogged: true,
            } => Ok(4772u32),
            GeneratedStruct92 {
                waterlogged: false,
                shape: RailShape::SouthWest,
            } => Ok(4773u32),
            GeneratedStruct92 {
                waterlogged: true,
                shape: RailShape::NorthWest,
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
                shape: RailShape::NorthEast,
                waterlogged: false,
            } => Ok(4777u32),
            _ => Err(()),
        }
    }
}

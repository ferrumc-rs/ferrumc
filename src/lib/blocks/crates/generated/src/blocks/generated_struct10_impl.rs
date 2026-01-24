use crate::GeneratedStruct10;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for GeneratedStruct10 {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            19494u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::Floor,
                facing: Direction::North,
                powered: true,
            }),
            19495u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::Floor,
                facing: Direction::North,
                powered: false,
            }),
            19496u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::Floor,
                facing: Direction::South,
                powered: true,
            }),
            19497u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::Floor,
                facing: Direction::South,
                powered: false,
            }),
            19498u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::Floor,
                facing: Direction::West,
                powered: true,
            }),
            19499u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::Floor,
                facing: Direction::West,
                powered: false,
            }),
            19500u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::Floor,
                facing: Direction::East,
                powered: true,
            }),
            19501u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::Floor,
                facing: Direction::East,
                powered: false,
            }),
            19502u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::Ceiling,
                facing: Direction::North,
                powered: true,
            }),
            19503u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::Ceiling,
                facing: Direction::North,
                powered: false,
            }),
            19504u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::Ceiling,
                facing: Direction::South,
                powered: true,
            }),
            19505u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::Ceiling,
                facing: Direction::South,
                powered: false,
            }),
            19506u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::Ceiling,
                facing: Direction::West,
                powered: true,
            }),
            19507u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::Ceiling,
                facing: Direction::West,
                powered: false,
            }),
            19508u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::Ceiling,
                facing: Direction::East,
                powered: true,
            }),
            19509u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::Ceiling,
                facing: Direction::East,
                powered: false,
            }),
            19510u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::SingleWall,
                facing: Direction::North,
                powered: true,
            }),
            19511u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::SingleWall,
                facing: Direction::North,
                powered: false,
            }),
            19512u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::SingleWall,
                facing: Direction::South,
                powered: true,
            }),
            19513u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::SingleWall,
                facing: Direction::South,
                powered: false,
            }),
            19514u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::SingleWall,
                facing: Direction::West,
                powered: true,
            }),
            19515u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::SingleWall,
                facing: Direction::West,
                powered: false,
            }),
            19516u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::SingleWall,
                facing: Direction::East,
                powered: true,
            }),
            19517u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::SingleWall,
                facing: Direction::East,
                powered: false,
            }),
            19518u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::DoubleWall,
                facing: Direction::North,
                powered: true,
            }),
            19519u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::DoubleWall,
                facing: Direction::North,
                powered: false,
            }),
            19520u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::DoubleWall,
                facing: Direction::South,
                powered: true,
            }),
            19521u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::DoubleWall,
                facing: Direction::South,
                powered: false,
            }),
            19522u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::DoubleWall,
                facing: Direction::West,
                powered: true,
            }),
            19523u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::DoubleWall,
                facing: Direction::West,
                powered: false,
            }),
            19524u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::DoubleWall,
                facing: Direction::East,
                powered: true,
            }),
            19525u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::DoubleWall,
                facing: Direction::East,
                powered: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct10 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct10 {
                attachment: BellAttachType::Floor,
                facing: Direction::North,
                powered: true,
            } => Ok(19494u32),
            GeneratedStruct10 {
                attachment: BellAttachType::Floor,
                facing: Direction::North,
                powered: false,
            } => Ok(19495u32),
            GeneratedStruct10 {
                attachment: BellAttachType::Floor,
                facing: Direction::South,
                powered: true,
            } => Ok(19496u32),
            GeneratedStruct10 {
                attachment: BellAttachType::Floor,
                facing: Direction::South,
                powered: false,
            } => Ok(19497u32),
            GeneratedStruct10 {
                attachment: BellAttachType::Floor,
                facing: Direction::West,
                powered: true,
            } => Ok(19498u32),
            GeneratedStruct10 {
                attachment: BellAttachType::Floor,
                facing: Direction::West,
                powered: false,
            } => Ok(19499u32),
            GeneratedStruct10 {
                attachment: BellAttachType::Floor,
                facing: Direction::East,
                powered: true,
            } => Ok(19500u32),
            GeneratedStruct10 {
                attachment: BellAttachType::Floor,
                facing: Direction::East,
                powered: false,
            } => Ok(19501u32),
            GeneratedStruct10 {
                attachment: BellAttachType::Ceiling,
                facing: Direction::North,
                powered: true,
            } => Ok(19502u32),
            GeneratedStruct10 {
                attachment: BellAttachType::Ceiling,
                facing: Direction::North,
                powered: false,
            } => Ok(19503u32),
            GeneratedStruct10 {
                attachment: BellAttachType::Ceiling,
                facing: Direction::South,
                powered: true,
            } => Ok(19504u32),
            GeneratedStruct10 {
                attachment: BellAttachType::Ceiling,
                facing: Direction::South,
                powered: false,
            } => Ok(19505u32),
            GeneratedStruct10 {
                attachment: BellAttachType::Ceiling,
                facing: Direction::West,
                powered: true,
            } => Ok(19506u32),
            GeneratedStruct10 {
                attachment: BellAttachType::Ceiling,
                facing: Direction::West,
                powered: false,
            } => Ok(19507u32),
            GeneratedStruct10 {
                attachment: BellAttachType::Ceiling,
                facing: Direction::East,
                powered: true,
            } => Ok(19508u32),
            GeneratedStruct10 {
                attachment: BellAttachType::Ceiling,
                facing: Direction::East,
                powered: false,
            } => Ok(19509u32),
            GeneratedStruct10 {
                attachment: BellAttachType::SingleWall,
                facing: Direction::North,
                powered: true,
            } => Ok(19510u32),
            GeneratedStruct10 {
                attachment: BellAttachType::SingleWall,
                facing: Direction::North,
                powered: false,
            } => Ok(19511u32),
            GeneratedStruct10 {
                attachment: BellAttachType::SingleWall,
                facing: Direction::South,
                powered: true,
            } => Ok(19512u32),
            GeneratedStruct10 {
                attachment: BellAttachType::SingleWall,
                facing: Direction::South,
                powered: false,
            } => Ok(19513u32),
            GeneratedStruct10 {
                attachment: BellAttachType::SingleWall,
                facing: Direction::West,
                powered: true,
            } => Ok(19514u32),
            GeneratedStruct10 {
                attachment: BellAttachType::SingleWall,
                facing: Direction::West,
                powered: false,
            } => Ok(19515u32),
            GeneratedStruct10 {
                attachment: BellAttachType::SingleWall,
                facing: Direction::East,
                powered: true,
            } => Ok(19516u32),
            GeneratedStruct10 {
                attachment: BellAttachType::SingleWall,
                facing: Direction::East,
                powered: false,
            } => Ok(19517u32),
            GeneratedStruct10 {
                attachment: BellAttachType::DoubleWall,
                facing: Direction::North,
                powered: true,
            } => Ok(19518u32),
            GeneratedStruct10 {
                attachment: BellAttachType::DoubleWall,
                facing: Direction::North,
                powered: false,
            } => Ok(19519u32),
            GeneratedStruct10 {
                attachment: BellAttachType::DoubleWall,
                facing: Direction::South,
                powered: true,
            } => Ok(19520u32),
            GeneratedStruct10 {
                attachment: BellAttachType::DoubleWall,
                facing: Direction::South,
                powered: false,
            } => Ok(19521u32),
            GeneratedStruct10 {
                attachment: BellAttachType::DoubleWall,
                facing: Direction::West,
                powered: true,
            } => Ok(19522u32),
            GeneratedStruct10 {
                attachment: BellAttachType::DoubleWall,
                facing: Direction::West,
                powered: false,
            } => Ok(19523u32),
            GeneratedStruct10 {
                attachment: BellAttachType::DoubleWall,
                facing: Direction::East,
                powered: true,
            } => Ok(19524u32),
            GeneratedStruct10 {
                attachment: BellAttachType::DoubleWall,
                facing: Direction::East,
                powered: false,
            } => Ok(19525u32),
            _ => Err(()),
        }
    }
}

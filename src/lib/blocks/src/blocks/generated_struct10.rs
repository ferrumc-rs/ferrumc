#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub struct GeneratedStruct10 {
    pub attachment: BellAttachType,
    pub facing: Direction,
    pub powered: bool,
}
impl TryFrom<u32> for GeneratedStruct10 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            19494u32 => Ok(GeneratedStruct10 {
                facing: Direction::North,
                powered: true,
                attachment: BellAttachType::Floor,
            }),
            19495u32 => Ok(GeneratedStruct10 {
                facing: Direction::North,
                powered: false,
                attachment: BellAttachType::Floor,
            }),
            19496u32 => Ok(GeneratedStruct10 {
                facing: Direction::South,
                powered: true,
                attachment: BellAttachType::Floor,
            }),
            19497u32 => Ok(GeneratedStruct10 {
                facing: Direction::South,
                powered: false,
                attachment: BellAttachType::Floor,
            }),
            19498u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::Floor,
                powered: true,
                facing: Direction::West,
            }),
            19499u32 => Ok(GeneratedStruct10 {
                facing: Direction::West,
                attachment: BellAttachType::Floor,
                powered: false,
            }),
            19500u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::Floor,
                facing: Direction::East,
                powered: true,
            }),
            19501u32 => Ok(GeneratedStruct10 {
                facing: Direction::East,
                attachment: BellAttachType::Floor,
                powered: false,
            }),
            19502u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::Ceiling,
                facing: Direction::North,
                powered: true,
            }),
            19503u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::Ceiling,
                powered: false,
                facing: Direction::North,
            }),
            19504u32 => Ok(GeneratedStruct10 {
                powered: true,
                facing: Direction::South,
                attachment: BellAttachType::Ceiling,
            }),
            19505u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::Ceiling,
                powered: false,
                facing: Direction::South,
            }),
            19506u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::Ceiling,
                facing: Direction::West,
                powered: true,
            }),
            19507u32 => Ok(GeneratedStruct10 {
                facing: Direction::West,
                attachment: BellAttachType::Ceiling,
                powered: false,
            }),
            19508u32 => Ok(GeneratedStruct10 {
                facing: Direction::East,
                attachment: BellAttachType::Ceiling,
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
                powered: false,
                facing: Direction::North,
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
                powered: true,
                facing: Direction::West,
                attachment: BellAttachType::SingleWall,
            }),
            19515u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::SingleWall,
                facing: Direction::West,
                powered: false,
            }),
            19516u32 => Ok(GeneratedStruct10 {
                powered: true,
                attachment: BellAttachType::SingleWall,
                facing: Direction::East,
            }),
            19517u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::SingleWall,
                facing: Direction::East,
                powered: false,
            }),
            19518u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::DoubleWall,
                powered: true,
                facing: Direction::North,
            }),
            19519u32 => Ok(GeneratedStruct10 {
                facing: Direction::North,
                attachment: BellAttachType::DoubleWall,
                powered: false,
            }),
            19520u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::DoubleWall,
                facing: Direction::South,
                powered: true,
            }),
            19521u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::DoubleWall,
                powered: false,
                facing: Direction::South,
            }),
            19522u32 => Ok(GeneratedStruct10 {
                powered: true,
                facing: Direction::West,
                attachment: BellAttachType::DoubleWall,
            }),
            19523u32 => Ok(GeneratedStruct10 {
                powered: false,
                facing: Direction::West,
                attachment: BellAttachType::DoubleWall,
            }),
            19524u32 => Ok(GeneratedStruct10 {
                powered: true,
                attachment: BellAttachType::DoubleWall,
                facing: Direction::East,
            }),
            19525u32 => Ok(GeneratedStruct10 {
                powered: false,
                facing: Direction::East,
                attachment: BellAttachType::DoubleWall,
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
                facing: Direction::North,
                powered: true,
                attachment: BellAttachType::Floor,
            } => Ok(19494u32),
            GeneratedStruct10 {
                facing: Direction::North,
                powered: false,
                attachment: BellAttachType::Floor,
            } => Ok(19495u32),
            GeneratedStruct10 {
                facing: Direction::South,
                powered: true,
                attachment: BellAttachType::Floor,
            } => Ok(19496u32),
            GeneratedStruct10 {
                facing: Direction::South,
                powered: false,
                attachment: BellAttachType::Floor,
            } => Ok(19497u32),
            GeneratedStruct10 {
                attachment: BellAttachType::Floor,
                powered: true,
                facing: Direction::West,
            } => Ok(19498u32),
            GeneratedStruct10 {
                facing: Direction::West,
                attachment: BellAttachType::Floor,
                powered: false,
            } => Ok(19499u32),
            GeneratedStruct10 {
                attachment: BellAttachType::Floor,
                facing: Direction::East,
                powered: true,
            } => Ok(19500u32),
            GeneratedStruct10 {
                facing: Direction::East,
                attachment: BellAttachType::Floor,
                powered: false,
            } => Ok(19501u32),
            GeneratedStruct10 {
                attachment: BellAttachType::Ceiling,
                facing: Direction::North,
                powered: true,
            } => Ok(19502u32),
            GeneratedStruct10 {
                attachment: BellAttachType::Ceiling,
                powered: false,
                facing: Direction::North,
            } => Ok(19503u32),
            GeneratedStruct10 {
                powered: true,
                facing: Direction::South,
                attachment: BellAttachType::Ceiling,
            } => Ok(19504u32),
            GeneratedStruct10 {
                attachment: BellAttachType::Ceiling,
                powered: false,
                facing: Direction::South,
            } => Ok(19505u32),
            GeneratedStruct10 {
                attachment: BellAttachType::Ceiling,
                facing: Direction::West,
                powered: true,
            } => Ok(19506u32),
            GeneratedStruct10 {
                facing: Direction::West,
                attachment: BellAttachType::Ceiling,
                powered: false,
            } => Ok(19507u32),
            GeneratedStruct10 {
                facing: Direction::East,
                attachment: BellAttachType::Ceiling,
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
                powered: false,
                facing: Direction::North,
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
                powered: true,
                facing: Direction::West,
                attachment: BellAttachType::SingleWall,
            } => Ok(19514u32),
            GeneratedStruct10 {
                attachment: BellAttachType::SingleWall,
                facing: Direction::West,
                powered: false,
            } => Ok(19515u32),
            GeneratedStruct10 {
                powered: true,
                attachment: BellAttachType::SingleWall,
                facing: Direction::East,
            } => Ok(19516u32),
            GeneratedStruct10 {
                attachment: BellAttachType::SingleWall,
                facing: Direction::East,
                powered: false,
            } => Ok(19517u32),
            GeneratedStruct10 {
                attachment: BellAttachType::DoubleWall,
                powered: true,
                facing: Direction::North,
            } => Ok(19518u32),
            GeneratedStruct10 {
                facing: Direction::North,
                attachment: BellAttachType::DoubleWall,
                powered: false,
            } => Ok(19519u32),
            GeneratedStruct10 {
                attachment: BellAttachType::DoubleWall,
                facing: Direction::South,
                powered: true,
            } => Ok(19520u32),
            GeneratedStruct10 {
                attachment: BellAttachType::DoubleWall,
                powered: false,
                facing: Direction::South,
            } => Ok(19521u32),
            GeneratedStruct10 {
                powered: true,
                facing: Direction::West,
                attachment: BellAttachType::DoubleWall,
            } => Ok(19522u32),
            GeneratedStruct10 {
                powered: false,
                facing: Direction::West,
                attachment: BellAttachType::DoubleWall,
            } => Ok(19523u32),
            GeneratedStruct10 {
                powered: true,
                attachment: BellAttachType::DoubleWall,
                facing: Direction::East,
            } => Ok(19524u32),
            GeneratedStruct10 {
                powered: false,
                facing: Direction::East,
                attachment: BellAttachType::DoubleWall,
            } => Ok(19525u32),
            _ => Err(()),
        }
    }
}

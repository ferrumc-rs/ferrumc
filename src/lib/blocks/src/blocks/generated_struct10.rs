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
                attachment: BellAttachType::Floor,
                powered: false,
                facing: Direction::North,
            }),
            19496u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::Floor,
                powered: true,
                facing: Direction::South,
            }),
            19497u32 => Ok(GeneratedStruct10 {
                facing: Direction::South,
                powered: false,
                attachment: BellAttachType::Floor,
            }),
            19498u32 => Ok(GeneratedStruct10 {
                facing: Direction::West,
                powered: true,
                attachment: BellAttachType::Floor,
            }),
            19499u32 => Ok(GeneratedStruct10 {
                facing: Direction::West,
                powered: false,
                attachment: BellAttachType::Floor,
            }),
            19500u32 => Ok(GeneratedStruct10 {
                powered: true,
                attachment: BellAttachType::Floor,
                facing: Direction::East,
            }),
            19501u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::Floor,
                facing: Direction::East,
                powered: false,
            }),
            19502u32 => Ok(GeneratedStruct10 {
                facing: Direction::North,
                powered: true,
                attachment: BellAttachType::Ceiling,
            }),
            19503u32 => Ok(GeneratedStruct10 {
                facing: Direction::North,
                attachment: BellAttachType::Ceiling,
                powered: false,
            }),
            19504u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::Ceiling,
                facing: Direction::South,
                powered: true,
            }),
            19505u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::Ceiling,
                powered: false,
                facing: Direction::South,
            }),
            19506u32 => Ok(GeneratedStruct10 {
                powered: true,
                facing: Direction::West,
                attachment: BellAttachType::Ceiling,
            }),
            19507u32 => Ok(GeneratedStruct10 {
                facing: Direction::West,
                attachment: BellAttachType::Ceiling,
                powered: false,
            }),
            19508u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::Ceiling,
                powered: true,
                facing: Direction::East,
            }),
            19509u32 => Ok(GeneratedStruct10 {
                attachment: BellAttachType::Ceiling,
                facing: Direction::East,
                powered: false,
            }),
            19510u32 => Ok(GeneratedStruct10 {
                facing: Direction::North,
                powered: true,
                attachment: BellAttachType::SingleWall,
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
                facing: Direction::South,
                attachment: BellAttachType::SingleWall,
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
                facing: Direction::East,
                powered: false,
                attachment: BellAttachType::SingleWall,
            }),
            19518u32 => Ok(GeneratedStruct10 {
                facing: Direction::North,
                powered: true,
                attachment: BellAttachType::DoubleWall,
            }),
            19519u32 => Ok(GeneratedStruct10 {
                facing: Direction::North,
                powered: false,
                attachment: BellAttachType::DoubleWall,
            }),
            19520u32 => Ok(GeneratedStruct10 {
                powered: true,
                attachment: BellAttachType::DoubleWall,
                facing: Direction::South,
            }),
            19521u32 => Ok(GeneratedStruct10 {
                powered: false,
                attachment: BellAttachType::DoubleWall,
                facing: Direction::South,
            }),
            19522u32 => Ok(GeneratedStruct10 {
                powered: true,
                attachment: BellAttachType::DoubleWall,
                facing: Direction::West,
            }),
            19523u32 => Ok(GeneratedStruct10 {
                powered: false,
                attachment: BellAttachType::DoubleWall,
                facing: Direction::West,
            }),
            19524u32 => Ok(GeneratedStruct10 {
                powered: true,
                facing: Direction::East,
                attachment: BellAttachType::DoubleWall,
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
                attachment: BellAttachType::Floor,
                powered: false,
                facing: Direction::North,
            } => Ok(19495u32),
            GeneratedStruct10 {
                attachment: BellAttachType::Floor,
                powered: true,
                facing: Direction::South,
            } => Ok(19496u32),
            GeneratedStruct10 {
                facing: Direction::South,
                powered: false,
                attachment: BellAttachType::Floor,
            } => Ok(19497u32),
            GeneratedStruct10 {
                facing: Direction::West,
                powered: true,
                attachment: BellAttachType::Floor,
            } => Ok(19498u32),
            GeneratedStruct10 {
                facing: Direction::West,
                powered: false,
                attachment: BellAttachType::Floor,
            } => Ok(19499u32),
            GeneratedStruct10 {
                powered: true,
                attachment: BellAttachType::Floor,
                facing: Direction::East,
            } => Ok(19500u32),
            GeneratedStruct10 {
                attachment: BellAttachType::Floor,
                facing: Direction::East,
                powered: false,
            } => Ok(19501u32),
            GeneratedStruct10 {
                facing: Direction::North,
                powered: true,
                attachment: BellAttachType::Ceiling,
            } => Ok(19502u32),
            GeneratedStruct10 {
                facing: Direction::North,
                attachment: BellAttachType::Ceiling,
                powered: false,
            } => Ok(19503u32),
            GeneratedStruct10 {
                attachment: BellAttachType::Ceiling,
                facing: Direction::South,
                powered: true,
            } => Ok(19504u32),
            GeneratedStruct10 {
                attachment: BellAttachType::Ceiling,
                powered: false,
                facing: Direction::South,
            } => Ok(19505u32),
            GeneratedStruct10 {
                powered: true,
                facing: Direction::West,
                attachment: BellAttachType::Ceiling,
            } => Ok(19506u32),
            GeneratedStruct10 {
                facing: Direction::West,
                attachment: BellAttachType::Ceiling,
                powered: false,
            } => Ok(19507u32),
            GeneratedStruct10 {
                attachment: BellAttachType::Ceiling,
                powered: true,
                facing: Direction::East,
            } => Ok(19508u32),
            GeneratedStruct10 {
                attachment: BellAttachType::Ceiling,
                facing: Direction::East,
                powered: false,
            } => Ok(19509u32),
            GeneratedStruct10 {
                facing: Direction::North,
                powered: true,
                attachment: BellAttachType::SingleWall,
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
                facing: Direction::South,
                attachment: BellAttachType::SingleWall,
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
                facing: Direction::East,
                powered: false,
                attachment: BellAttachType::SingleWall,
            } => Ok(19517u32),
            GeneratedStruct10 {
                facing: Direction::North,
                powered: true,
                attachment: BellAttachType::DoubleWall,
            } => Ok(19518u32),
            GeneratedStruct10 {
                facing: Direction::North,
                powered: false,
                attachment: BellAttachType::DoubleWall,
            } => Ok(19519u32),
            GeneratedStruct10 {
                powered: true,
                attachment: BellAttachType::DoubleWall,
                facing: Direction::South,
            } => Ok(19520u32),
            GeneratedStruct10 {
                powered: false,
                attachment: BellAttachType::DoubleWall,
                facing: Direction::South,
            } => Ok(19521u32),
            GeneratedStruct10 {
                powered: true,
                attachment: BellAttachType::DoubleWall,
                facing: Direction::West,
            } => Ok(19522u32),
            GeneratedStruct10 {
                powered: false,
                attachment: BellAttachType::DoubleWall,
                facing: Direction::West,
            } => Ok(19523u32),
            GeneratedStruct10 {
                powered: true,
                facing: Direction::East,
                attachment: BellAttachType::DoubleWall,
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

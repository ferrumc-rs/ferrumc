#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub enum GeneratedStruct63Type {
    Dispenser,
    Dropper,
}
#[allow(dead_code)]
pub struct GeneratedStruct63 {
    pub block_type: GeneratedStruct63Type,
    pub facing: Direction,
    pub triggered: bool,
}
impl TryFrom<u32> for GeneratedStruct63 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            566u32 => Ok(GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dispenser,
                facing: Direction::North,
                triggered: true,
            }),
            567u32 => Ok(GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dispenser,
                facing: Direction::North,
                triggered: false,
            }),
            568u32 => Ok(GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dispenser,
                facing: Direction::East,
                triggered: true,
            }),
            569u32 => Ok(GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dispenser,
                facing: Direction::East,
                triggered: false,
            }),
            570u32 => Ok(GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dispenser,
                facing: Direction::South,
                triggered: true,
            }),
            571u32 => Ok(GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dispenser,
                facing: Direction::South,
                triggered: false,
            }),
            572u32 => Ok(GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dispenser,
                facing: Direction::West,
                triggered: true,
            }),
            573u32 => Ok(GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dispenser,
                facing: Direction::West,
                triggered: false,
            }),
            574u32 => Ok(GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dispenser,
                facing: Direction::Up,
                triggered: true,
            }),
            575u32 => Ok(GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dispenser,
                facing: Direction::Up,
                triggered: false,
            }),
            576u32 => Ok(GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dispenser,
                facing: Direction::Down,
                triggered: true,
            }),
            577u32 => Ok(GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dispenser,
                facing: Direction::Down,
                triggered: false,
            }),
            10153u32 => Ok(GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dropper,
                facing: Direction::North,
                triggered: true,
            }),
            10154u32 => Ok(GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dropper,
                facing: Direction::North,
                triggered: false,
            }),
            10155u32 => Ok(GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dropper,
                facing: Direction::East,
                triggered: true,
            }),
            10156u32 => Ok(GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dropper,
                facing: Direction::East,
                triggered: false,
            }),
            10157u32 => Ok(GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dropper,
                facing: Direction::South,
                triggered: true,
            }),
            10158u32 => Ok(GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dropper,
                facing: Direction::South,
                triggered: false,
            }),
            10159u32 => Ok(GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dropper,
                facing: Direction::West,
                triggered: true,
            }),
            10160u32 => Ok(GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dropper,
                facing: Direction::West,
                triggered: false,
            }),
            10161u32 => Ok(GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dropper,
                facing: Direction::Up,
                triggered: true,
            }),
            10162u32 => Ok(GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dropper,
                facing: Direction::Up,
                triggered: false,
            }),
            10163u32 => Ok(GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dropper,
                facing: Direction::Down,
                triggered: true,
            }),
            10164u32 => Ok(GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dropper,
                facing: Direction::Down,
                triggered: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct63 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dispenser,
                facing: Direction::North,
                triggered: true,
            } => Ok(566u32),
            GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dispenser,
                facing: Direction::North,
                triggered: false,
            } => Ok(567u32),
            GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dispenser,
                facing: Direction::East,
                triggered: true,
            } => Ok(568u32),
            GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dispenser,
                facing: Direction::East,
                triggered: false,
            } => Ok(569u32),
            GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dispenser,
                facing: Direction::South,
                triggered: true,
            } => Ok(570u32),
            GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dispenser,
                facing: Direction::South,
                triggered: false,
            } => Ok(571u32),
            GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dispenser,
                facing: Direction::West,
                triggered: true,
            } => Ok(572u32),
            GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dispenser,
                facing: Direction::West,
                triggered: false,
            } => Ok(573u32),
            GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dispenser,
                facing: Direction::Up,
                triggered: true,
            } => Ok(574u32),
            GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dispenser,
                facing: Direction::Up,
                triggered: false,
            } => Ok(575u32),
            GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dispenser,
                facing: Direction::Down,
                triggered: true,
            } => Ok(576u32),
            GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dispenser,
                facing: Direction::Down,
                triggered: false,
            } => Ok(577u32),
            GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dropper,
                facing: Direction::North,
                triggered: true,
            } => Ok(10153u32),
            GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dropper,
                facing: Direction::North,
                triggered: false,
            } => Ok(10154u32),
            GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dropper,
                facing: Direction::East,
                triggered: true,
            } => Ok(10155u32),
            GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dropper,
                facing: Direction::East,
                triggered: false,
            } => Ok(10156u32),
            GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dropper,
                facing: Direction::South,
                triggered: true,
            } => Ok(10157u32),
            GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dropper,
                facing: Direction::South,
                triggered: false,
            } => Ok(10158u32),
            GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dropper,
                facing: Direction::West,
                triggered: true,
            } => Ok(10159u32),
            GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dropper,
                facing: Direction::West,
                triggered: false,
            } => Ok(10160u32),
            GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dropper,
                facing: Direction::Up,
                triggered: true,
            } => Ok(10161u32),
            GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dropper,
                facing: Direction::Up,
                triggered: false,
            } => Ok(10162u32),
            GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dropper,
                facing: Direction::Down,
                triggered: true,
            } => Ok(10163u32),
            GeneratedStruct63 {
                block_type: GeneratedStruct63Type::Dropper,
                facing: Direction::Down,
                triggered: false,
            } => Ok(10164u32),
            _ => Err(()),
        }
    }
}

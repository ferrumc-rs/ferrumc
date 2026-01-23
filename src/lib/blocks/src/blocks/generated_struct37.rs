#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub enum GeneratedStruct37Type {
    Piston,
    StickyPiston,
}
#[allow(dead_code)]
pub struct GeneratedStruct37 {
    pub block_type: GeneratedStruct37Type,
    pub extended: bool,
    pub facing: Direction,
}
impl TryFrom<u32> for GeneratedStruct37 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            2057u32 => Ok(GeneratedStruct37 {
                block_type: GeneratedStruct37Type::Piston,
                extended: true,
                facing: Direction::North,
            }),
            2058u32 => Ok(GeneratedStruct37 {
                block_type: GeneratedStruct37Type::Piston,
                extended: true,
                facing: Direction::East,
            }),
            2059u32 => Ok(GeneratedStruct37 {
                block_type: GeneratedStruct37Type::Piston,
                extended: true,
                facing: Direction::South,
            }),
            2060u32 => Ok(GeneratedStruct37 {
                block_type: GeneratedStruct37Type::Piston,
                extended: true,
                facing: Direction::West,
            }),
            2061u32 => Ok(GeneratedStruct37 {
                block_type: GeneratedStruct37Type::Piston,
                extended: true,
                facing: Direction::Up,
            }),
            2062u32 => Ok(GeneratedStruct37 {
                block_type: GeneratedStruct37Type::Piston,
                facing: Direction::Down,
                extended: true,
            }),
            2063u32 => Ok(GeneratedStruct37 {
                block_type: GeneratedStruct37Type::Piston,
                extended: false,
                facing: Direction::North,
            }),
            2064u32 => Ok(GeneratedStruct37 {
                block_type: GeneratedStruct37Type::Piston,
                facing: Direction::East,
                extended: false,
            }),
            2065u32 => Ok(GeneratedStruct37 {
                block_type: GeneratedStruct37Type::Piston,
                facing: Direction::South,
                extended: false,
            }),
            2066u32 => Ok(GeneratedStruct37 {
                block_type: GeneratedStruct37Type::Piston,
                extended: false,
                facing: Direction::West,
            }),
            2067u32 => Ok(GeneratedStruct37 {
                block_type: GeneratedStruct37Type::Piston,
                extended: false,
                facing: Direction::Up,
            }),
            2068u32 => Ok(GeneratedStruct37 {
                block_type: GeneratedStruct37Type::Piston,
                extended: false,
                facing: Direction::Down,
            }),
            2035u32 => Ok(GeneratedStruct37 {
                block_type: GeneratedStruct37Type::StickyPiston,
                facing: Direction::North,
                extended: true,
            }),
            2036u32 => Ok(GeneratedStruct37 {
                block_type: GeneratedStruct37Type::StickyPiston,
                extended: true,
                facing: Direction::East,
            }),
            2037u32 => Ok(GeneratedStruct37 {
                block_type: GeneratedStruct37Type::StickyPiston,
                facing: Direction::South,
                extended: true,
            }),
            2038u32 => Ok(GeneratedStruct37 {
                block_type: GeneratedStruct37Type::StickyPiston,
                extended: true,
                facing: Direction::West,
            }),
            2039u32 => Ok(GeneratedStruct37 {
                block_type: GeneratedStruct37Type::StickyPiston,
                facing: Direction::Up,
                extended: true,
            }),
            2040u32 => Ok(GeneratedStruct37 {
                block_type: GeneratedStruct37Type::StickyPiston,
                facing: Direction::Down,
                extended: true,
            }),
            2041u32 => Ok(GeneratedStruct37 {
                block_type: GeneratedStruct37Type::StickyPiston,
                extended: false,
                facing: Direction::North,
            }),
            2042u32 => Ok(GeneratedStruct37 {
                block_type: GeneratedStruct37Type::StickyPiston,
                facing: Direction::East,
                extended: false,
            }),
            2043u32 => Ok(GeneratedStruct37 {
                block_type: GeneratedStruct37Type::StickyPiston,
                facing: Direction::South,
                extended: false,
            }),
            2044u32 => Ok(GeneratedStruct37 {
                block_type: GeneratedStruct37Type::StickyPiston,
                facing: Direction::West,
                extended: false,
            }),
            2045u32 => Ok(GeneratedStruct37 {
                block_type: GeneratedStruct37Type::StickyPiston,
                extended: false,
                facing: Direction::Up,
            }),
            2046u32 => Ok(GeneratedStruct37 {
                block_type: GeneratedStruct37Type::StickyPiston,
                facing: Direction::Down,
                extended: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct37 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct37 {
                block_type: GeneratedStruct37Type::Piston,
                extended: true,
                facing: Direction::North,
            } => Ok(2057u32),
            GeneratedStruct37 {
                block_type: GeneratedStruct37Type::Piston,
                extended: true,
                facing: Direction::East,
            } => Ok(2058u32),
            GeneratedStruct37 {
                block_type: GeneratedStruct37Type::Piston,
                extended: true,
                facing: Direction::South,
            } => Ok(2059u32),
            GeneratedStruct37 {
                block_type: GeneratedStruct37Type::Piston,
                extended: true,
                facing: Direction::West,
            } => Ok(2060u32),
            GeneratedStruct37 {
                block_type: GeneratedStruct37Type::Piston,
                extended: true,
                facing: Direction::Up,
            } => Ok(2061u32),
            GeneratedStruct37 {
                block_type: GeneratedStruct37Type::Piston,
                facing: Direction::Down,
                extended: true,
            } => Ok(2062u32),
            GeneratedStruct37 {
                block_type: GeneratedStruct37Type::Piston,
                extended: false,
                facing: Direction::North,
            } => Ok(2063u32),
            GeneratedStruct37 {
                block_type: GeneratedStruct37Type::Piston,
                facing: Direction::East,
                extended: false,
            } => Ok(2064u32),
            GeneratedStruct37 {
                block_type: GeneratedStruct37Type::Piston,
                facing: Direction::South,
                extended: false,
            } => Ok(2065u32),
            GeneratedStruct37 {
                block_type: GeneratedStruct37Type::Piston,
                extended: false,
                facing: Direction::West,
            } => Ok(2066u32),
            GeneratedStruct37 {
                block_type: GeneratedStruct37Type::Piston,
                extended: false,
                facing: Direction::Up,
            } => Ok(2067u32),
            GeneratedStruct37 {
                block_type: GeneratedStruct37Type::Piston,
                extended: false,
                facing: Direction::Down,
            } => Ok(2068u32),
            GeneratedStruct37 {
                block_type: GeneratedStruct37Type::StickyPiston,
                facing: Direction::North,
                extended: true,
            } => Ok(2035u32),
            GeneratedStruct37 {
                block_type: GeneratedStruct37Type::StickyPiston,
                extended: true,
                facing: Direction::East,
            } => Ok(2036u32),
            GeneratedStruct37 {
                block_type: GeneratedStruct37Type::StickyPiston,
                facing: Direction::South,
                extended: true,
            } => Ok(2037u32),
            GeneratedStruct37 {
                block_type: GeneratedStruct37Type::StickyPiston,
                extended: true,
                facing: Direction::West,
            } => Ok(2038u32),
            GeneratedStruct37 {
                block_type: GeneratedStruct37Type::StickyPiston,
                facing: Direction::Up,
                extended: true,
            } => Ok(2039u32),
            GeneratedStruct37 {
                block_type: GeneratedStruct37Type::StickyPiston,
                facing: Direction::Down,
                extended: true,
            } => Ok(2040u32),
            GeneratedStruct37 {
                block_type: GeneratedStruct37Type::StickyPiston,
                extended: false,
                facing: Direction::North,
            } => Ok(2041u32),
            GeneratedStruct37 {
                block_type: GeneratedStruct37Type::StickyPiston,
                facing: Direction::East,
                extended: false,
            } => Ok(2042u32),
            GeneratedStruct37 {
                block_type: GeneratedStruct37Type::StickyPiston,
                facing: Direction::South,
                extended: false,
            } => Ok(2043u32),
            GeneratedStruct37 {
                block_type: GeneratedStruct37Type::StickyPiston,
                facing: Direction::West,
                extended: false,
            } => Ok(2044u32),
            GeneratedStruct37 {
                block_type: GeneratedStruct37Type::StickyPiston,
                extended: false,
                facing: Direction::Up,
            } => Ok(2045u32),
            GeneratedStruct37 {
                block_type: GeneratedStruct37Type::StickyPiston,
                facing: Direction::Down,
                extended: false,
            } => Ok(2046u32),
            _ => Err(()),
        }
    }
}

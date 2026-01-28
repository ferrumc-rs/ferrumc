use crate::PistonBlock;
use crate::PistonBlockType;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for PistonBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            2057u32 => Ok(PistonBlock {
                block_type: PistonBlockType::Piston,
                extended: true,
                facing: Direction::North,
            }),
            2058u32 => Ok(PistonBlock {
                block_type: PistonBlockType::Piston,
                extended: true,
                facing: Direction::East,
            }),
            2059u32 => Ok(PistonBlock {
                block_type: PistonBlockType::Piston,
                extended: true,
                facing: Direction::South,
            }),
            2060u32 => Ok(PistonBlock {
                block_type: PistonBlockType::Piston,
                extended: true,
                facing: Direction::West,
            }),
            2061u32 => Ok(PistonBlock {
                block_type: PistonBlockType::Piston,
                extended: true,
                facing: Direction::Up,
            }),
            2062u32 => Ok(PistonBlock {
                block_type: PistonBlockType::Piston,
                extended: true,
                facing: Direction::Down,
            }),
            2063u32 => Ok(PistonBlock {
                block_type: PistonBlockType::Piston,
                extended: false,
                facing: Direction::North,
            }),
            2064u32 => Ok(PistonBlock {
                block_type: PistonBlockType::Piston,
                extended: false,
                facing: Direction::East,
            }),
            2065u32 => Ok(PistonBlock {
                block_type: PistonBlockType::Piston,
                extended: false,
                facing: Direction::South,
            }),
            2066u32 => Ok(PistonBlock {
                block_type: PistonBlockType::Piston,
                extended: false,
                facing: Direction::West,
            }),
            2067u32 => Ok(PistonBlock {
                block_type: PistonBlockType::Piston,
                extended: false,
                facing: Direction::Up,
            }),
            2068u32 => Ok(PistonBlock {
                block_type: PistonBlockType::Piston,
                extended: false,
                facing: Direction::Down,
            }),
            2035u32 => Ok(PistonBlock {
                block_type: PistonBlockType::StickyPiston,
                extended: true,
                facing: Direction::North,
            }),
            2036u32 => Ok(PistonBlock {
                block_type: PistonBlockType::StickyPiston,
                extended: true,
                facing: Direction::East,
            }),
            2037u32 => Ok(PistonBlock {
                block_type: PistonBlockType::StickyPiston,
                extended: true,
                facing: Direction::South,
            }),
            2038u32 => Ok(PistonBlock {
                block_type: PistonBlockType::StickyPiston,
                extended: true,
                facing: Direction::West,
            }),
            2039u32 => Ok(PistonBlock {
                block_type: PistonBlockType::StickyPiston,
                extended: true,
                facing: Direction::Up,
            }),
            2040u32 => Ok(PistonBlock {
                block_type: PistonBlockType::StickyPiston,
                extended: true,
                facing: Direction::Down,
            }),
            2041u32 => Ok(PistonBlock {
                block_type: PistonBlockType::StickyPiston,
                extended: false,
                facing: Direction::North,
            }),
            2042u32 => Ok(PistonBlock {
                block_type: PistonBlockType::StickyPiston,
                extended: false,
                facing: Direction::East,
            }),
            2043u32 => Ok(PistonBlock {
                block_type: PistonBlockType::StickyPiston,
                extended: false,
                facing: Direction::South,
            }),
            2044u32 => Ok(PistonBlock {
                block_type: PistonBlockType::StickyPiston,
                extended: false,
                facing: Direction::West,
            }),
            2045u32 => Ok(PistonBlock {
                block_type: PistonBlockType::StickyPiston,
                extended: false,
                facing: Direction::Up,
            }),
            2046u32 => Ok(PistonBlock {
                block_type: PistonBlockType::StickyPiston,
                extended: false,
                facing: Direction::Down,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for PistonBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            PistonBlock {
                block_type: PistonBlockType::Piston,
                extended: true,
                facing: Direction::North,
            } => Ok(2057u32),
            PistonBlock {
                block_type: PistonBlockType::Piston,
                extended: true,
                facing: Direction::East,
            } => Ok(2058u32),
            PistonBlock {
                block_type: PistonBlockType::Piston,
                extended: true,
                facing: Direction::South,
            } => Ok(2059u32),
            PistonBlock {
                block_type: PistonBlockType::Piston,
                extended: true,
                facing: Direction::West,
            } => Ok(2060u32),
            PistonBlock {
                block_type: PistonBlockType::Piston,
                extended: true,
                facing: Direction::Up,
            } => Ok(2061u32),
            PistonBlock {
                block_type: PistonBlockType::Piston,
                extended: true,
                facing: Direction::Down,
            } => Ok(2062u32),
            PistonBlock {
                block_type: PistonBlockType::Piston,
                extended: false,
                facing: Direction::North,
            } => Ok(2063u32),
            PistonBlock {
                block_type: PistonBlockType::Piston,
                extended: false,
                facing: Direction::East,
            } => Ok(2064u32),
            PistonBlock {
                block_type: PistonBlockType::Piston,
                extended: false,
                facing: Direction::South,
            } => Ok(2065u32),
            PistonBlock {
                block_type: PistonBlockType::Piston,
                extended: false,
                facing: Direction::West,
            } => Ok(2066u32),
            PistonBlock {
                block_type: PistonBlockType::Piston,
                extended: false,
                facing: Direction::Up,
            } => Ok(2067u32),
            PistonBlock {
                block_type: PistonBlockType::Piston,
                extended: false,
                facing: Direction::Down,
            } => Ok(2068u32),
            PistonBlock {
                block_type: PistonBlockType::StickyPiston,
                extended: true,
                facing: Direction::North,
            } => Ok(2035u32),
            PistonBlock {
                block_type: PistonBlockType::StickyPiston,
                extended: true,
                facing: Direction::East,
            } => Ok(2036u32),
            PistonBlock {
                block_type: PistonBlockType::StickyPiston,
                extended: true,
                facing: Direction::South,
            } => Ok(2037u32),
            PistonBlock {
                block_type: PistonBlockType::StickyPiston,
                extended: true,
                facing: Direction::West,
            } => Ok(2038u32),
            PistonBlock {
                block_type: PistonBlockType::StickyPiston,
                extended: true,
                facing: Direction::Up,
            } => Ok(2039u32),
            PistonBlock {
                block_type: PistonBlockType::StickyPiston,
                extended: true,
                facing: Direction::Down,
            } => Ok(2040u32),
            PistonBlock {
                block_type: PistonBlockType::StickyPiston,
                extended: false,
                facing: Direction::North,
            } => Ok(2041u32),
            PistonBlock {
                block_type: PistonBlockType::StickyPiston,
                extended: false,
                facing: Direction::East,
            } => Ok(2042u32),
            PistonBlock {
                block_type: PistonBlockType::StickyPiston,
                extended: false,
                facing: Direction::South,
            } => Ok(2043u32),
            PistonBlock {
                block_type: PistonBlockType::StickyPiston,
                extended: false,
                facing: Direction::West,
            } => Ok(2044u32),
            PistonBlock {
                block_type: PistonBlockType::StickyPiston,
                extended: false,
                facing: Direction::Up,
            } => Ok(2045u32),
            PistonBlock {
                block_type: PistonBlockType::StickyPiston,
                extended: false,
                facing: Direction::Down,
            } => Ok(2046u32),
            _ => Err(()),
        }
    }
}

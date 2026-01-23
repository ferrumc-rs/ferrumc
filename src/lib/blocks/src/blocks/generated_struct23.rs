#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub struct GeneratedStruct23 {
    pub cracked: bool,
    pub facing: Direction,
    pub waterlogged: bool,
}
impl TryFrom<u32> for GeneratedStruct23 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            27634u32 => Ok(GeneratedStruct23 {
                facing: Direction::North,
                cracked: true,
                waterlogged: true,
            }),
            27635u32 => Ok(GeneratedStruct23 {
                cracked: true,
                waterlogged: false,
                facing: Direction::North,
            }),
            27636u32 => Ok(GeneratedStruct23 {
                facing: Direction::South,
                waterlogged: true,
                cracked: true,
            }),
            27637u32 => Ok(GeneratedStruct23 {
                waterlogged: false,
                facing: Direction::South,
                cracked: true,
            }),
            27638u32 => Ok(GeneratedStruct23 {
                facing: Direction::West,
                cracked: true,
                waterlogged: true,
            }),
            27639u32 => Ok(GeneratedStruct23 {
                waterlogged: false,
                facing: Direction::West,
                cracked: true,
            }),
            27640u32 => Ok(GeneratedStruct23 {
                cracked: true,
                facing: Direction::East,
                waterlogged: true,
            }),
            27641u32 => Ok(GeneratedStruct23 {
                cracked: true,
                waterlogged: false,
                facing: Direction::East,
            }),
            27642u32 => Ok(GeneratedStruct23 {
                facing: Direction::North,
                waterlogged: true,
                cracked: false,
            }),
            27643u32 => Ok(GeneratedStruct23 {
                waterlogged: false,
                cracked: false,
                facing: Direction::North,
            }),
            27644u32 => Ok(GeneratedStruct23 {
                facing: Direction::South,
                waterlogged: true,
                cracked: false,
            }),
            27645u32 => Ok(GeneratedStruct23 {
                waterlogged: false,
                facing: Direction::South,
                cracked: false,
            }),
            27646u32 => Ok(GeneratedStruct23 {
                facing: Direction::West,
                waterlogged: true,
                cracked: false,
            }),
            27647u32 => Ok(GeneratedStruct23 {
                waterlogged: false,
                facing: Direction::West,
                cracked: false,
            }),
            27648u32 => Ok(GeneratedStruct23 {
                cracked: false,
                facing: Direction::East,
                waterlogged: true,
            }),
            27649u32 => Ok(GeneratedStruct23 {
                cracked: false,
                waterlogged: false,
                facing: Direction::East,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct23 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct23 {
                facing: Direction::North,
                cracked: true,
                waterlogged: true,
            } => Ok(27634u32),
            GeneratedStruct23 {
                cracked: true,
                waterlogged: false,
                facing: Direction::North,
            } => Ok(27635u32),
            GeneratedStruct23 {
                facing: Direction::South,
                waterlogged: true,
                cracked: true,
            } => Ok(27636u32),
            GeneratedStruct23 {
                waterlogged: false,
                facing: Direction::South,
                cracked: true,
            } => Ok(27637u32),
            GeneratedStruct23 {
                facing: Direction::West,
                cracked: true,
                waterlogged: true,
            } => Ok(27638u32),
            GeneratedStruct23 {
                waterlogged: false,
                facing: Direction::West,
                cracked: true,
            } => Ok(27639u32),
            GeneratedStruct23 {
                cracked: true,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(27640u32),
            GeneratedStruct23 {
                cracked: true,
                waterlogged: false,
                facing: Direction::East,
            } => Ok(27641u32),
            GeneratedStruct23 {
                facing: Direction::North,
                waterlogged: true,
                cracked: false,
            } => Ok(27642u32),
            GeneratedStruct23 {
                waterlogged: false,
                cracked: false,
                facing: Direction::North,
            } => Ok(27643u32),
            GeneratedStruct23 {
                facing: Direction::South,
                waterlogged: true,
                cracked: false,
            } => Ok(27644u32),
            GeneratedStruct23 {
                waterlogged: false,
                facing: Direction::South,
                cracked: false,
            } => Ok(27645u32),
            GeneratedStruct23 {
                facing: Direction::West,
                waterlogged: true,
                cracked: false,
            } => Ok(27646u32),
            GeneratedStruct23 {
                waterlogged: false,
                facing: Direction::West,
                cracked: false,
            } => Ok(27647u32),
            GeneratedStruct23 {
                cracked: false,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(27648u32),
            GeneratedStruct23 {
                cracked: false,
                waterlogged: false,
                facing: Direction::East,
            } => Ok(27649u32),
            _ => Err(()),
        }
    }
}

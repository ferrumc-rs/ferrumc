#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub struct GeneratedStruct44 {
    pub facing: Direction,
    pub half: DoubleBlockHalf,
    pub waterlogged: bool,
}
impl TryFrom<u32> for GeneratedStruct44 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            25944u32 => Ok(GeneratedStruct44 {
                facing: Direction::North,
                waterlogged: true,
                half: DoubleBlockHalf::Upper,
            }),
            25945u32 => Ok(GeneratedStruct44 {
                facing: Direction::North,
                waterlogged: false,
                half: DoubleBlockHalf::Upper,
            }),
            25946u32 => Ok(GeneratedStruct44 {
                half: DoubleBlockHalf::Lower,
                facing: Direction::North,
                waterlogged: true,
            }),
            25947u32 => Ok(GeneratedStruct44 {
                waterlogged: false,
                half: DoubleBlockHalf::Lower,
                facing: Direction::North,
            }),
            25948u32 => Ok(GeneratedStruct44 {
                facing: Direction::South,
                half: DoubleBlockHalf::Upper,
                waterlogged: true,
            }),
            25949u32 => Ok(GeneratedStruct44 {
                facing: Direction::South,
                waterlogged: false,
                half: DoubleBlockHalf::Upper,
            }),
            25950u32 => Ok(GeneratedStruct44 {
                facing: Direction::South,
                waterlogged: true,
                half: DoubleBlockHalf::Lower,
            }),
            25951u32 => Ok(GeneratedStruct44 {
                facing: Direction::South,
                half: DoubleBlockHalf::Lower,
                waterlogged: false,
            }),
            25952u32 => Ok(GeneratedStruct44 {
                half: DoubleBlockHalf::Upper,
                facing: Direction::West,
                waterlogged: true,
            }),
            25953u32 => Ok(GeneratedStruct44 {
                facing: Direction::West,
                half: DoubleBlockHalf::Upper,
                waterlogged: false,
            }),
            25954u32 => Ok(GeneratedStruct44 {
                facing: Direction::West,
                half: DoubleBlockHalf::Lower,
                waterlogged: true,
            }),
            25955u32 => Ok(GeneratedStruct44 {
                half: DoubleBlockHalf::Lower,
                facing: Direction::West,
                waterlogged: false,
            }),
            25956u32 => Ok(GeneratedStruct44 {
                half: DoubleBlockHalf::Upper,
                facing: Direction::East,
                waterlogged: true,
            }),
            25957u32 => Ok(GeneratedStruct44 {
                half: DoubleBlockHalf::Upper,
                waterlogged: false,
                facing: Direction::East,
            }),
            25958u32 => Ok(GeneratedStruct44 {
                half: DoubleBlockHalf::Lower,
                waterlogged: true,
                facing: Direction::East,
            }),
            25959u32 => Ok(GeneratedStruct44 {
                half: DoubleBlockHalf::Lower,
                facing: Direction::East,
                waterlogged: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct44 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct44 {
                facing: Direction::North,
                waterlogged: true,
                half: DoubleBlockHalf::Upper,
            } => Ok(25944u32),
            GeneratedStruct44 {
                facing: Direction::North,
                waterlogged: false,
                half: DoubleBlockHalf::Upper,
            } => Ok(25945u32),
            GeneratedStruct44 {
                half: DoubleBlockHalf::Lower,
                facing: Direction::North,
                waterlogged: true,
            } => Ok(25946u32),
            GeneratedStruct44 {
                waterlogged: false,
                half: DoubleBlockHalf::Lower,
                facing: Direction::North,
            } => Ok(25947u32),
            GeneratedStruct44 {
                facing: Direction::South,
                half: DoubleBlockHalf::Upper,
                waterlogged: true,
            } => Ok(25948u32),
            GeneratedStruct44 {
                facing: Direction::South,
                waterlogged: false,
                half: DoubleBlockHalf::Upper,
            } => Ok(25949u32),
            GeneratedStruct44 {
                facing: Direction::South,
                waterlogged: true,
                half: DoubleBlockHalf::Lower,
            } => Ok(25950u32),
            GeneratedStruct44 {
                facing: Direction::South,
                half: DoubleBlockHalf::Lower,
                waterlogged: false,
            } => Ok(25951u32),
            GeneratedStruct44 {
                half: DoubleBlockHalf::Upper,
                facing: Direction::West,
                waterlogged: true,
            } => Ok(25952u32),
            GeneratedStruct44 {
                facing: Direction::West,
                half: DoubleBlockHalf::Upper,
                waterlogged: false,
            } => Ok(25953u32),
            GeneratedStruct44 {
                facing: Direction::West,
                half: DoubleBlockHalf::Lower,
                waterlogged: true,
            } => Ok(25954u32),
            GeneratedStruct44 {
                half: DoubleBlockHalf::Lower,
                facing: Direction::West,
                waterlogged: false,
            } => Ok(25955u32),
            GeneratedStruct44 {
                half: DoubleBlockHalf::Upper,
                facing: Direction::East,
                waterlogged: true,
            } => Ok(25956u32),
            GeneratedStruct44 {
                half: DoubleBlockHalf::Upper,
                waterlogged: false,
                facing: Direction::East,
            } => Ok(25957u32),
            GeneratedStruct44 {
                half: DoubleBlockHalf::Lower,
                waterlogged: true,
                facing: Direction::East,
            } => Ok(25958u32),
            GeneratedStruct44 {
                half: DoubleBlockHalf::Lower,
                facing: Direction::East,
                waterlogged: false,
            } => Ok(25959u32),
            _ => Err(()),
        }
    }
}

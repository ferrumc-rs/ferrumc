use crate::GeneratedStruct44;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for GeneratedStruct44 {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            25944u32 => Ok(GeneratedStruct44 {
                facing: Direction::North,
                half: DoubleBlockHalf::Upper,
                waterlogged: true,
            }),
            25945u32 => Ok(GeneratedStruct44 {
                facing: Direction::North,
                half: DoubleBlockHalf::Upper,
                waterlogged: false,
            }),
            25946u32 => Ok(GeneratedStruct44 {
                facing: Direction::North,
                half: DoubleBlockHalf::Lower,
                waterlogged: true,
            }),
            25947u32 => Ok(GeneratedStruct44 {
                facing: Direction::North,
                half: DoubleBlockHalf::Lower,
                waterlogged: false,
            }),
            25948u32 => Ok(GeneratedStruct44 {
                facing: Direction::South,
                half: DoubleBlockHalf::Upper,
                waterlogged: true,
            }),
            25949u32 => Ok(GeneratedStruct44 {
                facing: Direction::South,
                half: DoubleBlockHalf::Upper,
                waterlogged: false,
            }),
            25950u32 => Ok(GeneratedStruct44 {
                facing: Direction::South,
                half: DoubleBlockHalf::Lower,
                waterlogged: true,
            }),
            25951u32 => Ok(GeneratedStruct44 {
                facing: Direction::South,
                half: DoubleBlockHalf::Lower,
                waterlogged: false,
            }),
            25952u32 => Ok(GeneratedStruct44 {
                facing: Direction::West,
                half: DoubleBlockHalf::Upper,
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
                facing: Direction::West,
                half: DoubleBlockHalf::Lower,
                waterlogged: false,
            }),
            25956u32 => Ok(GeneratedStruct44 {
                facing: Direction::East,
                half: DoubleBlockHalf::Upper,
                waterlogged: true,
            }),
            25957u32 => Ok(GeneratedStruct44 {
                facing: Direction::East,
                half: DoubleBlockHalf::Upper,
                waterlogged: false,
            }),
            25958u32 => Ok(GeneratedStruct44 {
                facing: Direction::East,
                half: DoubleBlockHalf::Lower,
                waterlogged: true,
            }),
            25959u32 => Ok(GeneratedStruct44 {
                facing: Direction::East,
                half: DoubleBlockHalf::Lower,
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
                half: DoubleBlockHalf::Upper,
                waterlogged: true,
            } => Ok(25944u32),
            GeneratedStruct44 {
                facing: Direction::North,
                half: DoubleBlockHalf::Upper,
                waterlogged: false,
            } => Ok(25945u32),
            GeneratedStruct44 {
                facing: Direction::North,
                half: DoubleBlockHalf::Lower,
                waterlogged: true,
            } => Ok(25946u32),
            GeneratedStruct44 {
                facing: Direction::North,
                half: DoubleBlockHalf::Lower,
                waterlogged: false,
            } => Ok(25947u32),
            GeneratedStruct44 {
                facing: Direction::South,
                half: DoubleBlockHalf::Upper,
                waterlogged: true,
            } => Ok(25948u32),
            GeneratedStruct44 {
                facing: Direction::South,
                half: DoubleBlockHalf::Upper,
                waterlogged: false,
            } => Ok(25949u32),
            GeneratedStruct44 {
                facing: Direction::South,
                half: DoubleBlockHalf::Lower,
                waterlogged: true,
            } => Ok(25950u32),
            GeneratedStruct44 {
                facing: Direction::South,
                half: DoubleBlockHalf::Lower,
                waterlogged: false,
            } => Ok(25951u32),
            GeneratedStruct44 {
                facing: Direction::West,
                half: DoubleBlockHalf::Upper,
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
                facing: Direction::West,
                half: DoubleBlockHalf::Lower,
                waterlogged: false,
            } => Ok(25955u32),
            GeneratedStruct44 {
                facing: Direction::East,
                half: DoubleBlockHalf::Upper,
                waterlogged: true,
            } => Ok(25956u32),
            GeneratedStruct44 {
                facing: Direction::East,
                half: DoubleBlockHalf::Upper,
                waterlogged: false,
            } => Ok(25957u32),
            GeneratedStruct44 {
                facing: Direction::East,
                half: DoubleBlockHalf::Lower,
                waterlogged: true,
            } => Ok(25958u32),
            GeneratedStruct44 {
                facing: Direction::East,
                half: DoubleBlockHalf::Lower,
                waterlogged: false,
            } => Ok(25959u32),
            _ => Err(()),
        }
    }
}

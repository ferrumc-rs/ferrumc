#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub struct GeneratedStruct59 {
    pub facing: Direction,
    pub segment_amount: i32,
}
impl TryFrom<u32> for GeneratedStruct59 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            25887u32 => Ok(GeneratedStruct59 {
                segment_amount: 1i32,
                facing: Direction::North,
            }),
            25888u32 => Ok(GeneratedStruct59 {
                facing: Direction::North,
                segment_amount: 2i32,
            }),
            25889u32 => Ok(GeneratedStruct59 {
                segment_amount: 3i32,
                facing: Direction::North,
            }),
            25890u32 => Ok(GeneratedStruct59 {
                segment_amount: 4i32,
                facing: Direction::North,
            }),
            25891u32 => Ok(GeneratedStruct59 {
                segment_amount: 1i32,
                facing: Direction::South,
            }),
            25892u32 => Ok(GeneratedStruct59 {
                segment_amount: 2i32,
                facing: Direction::South,
            }),
            25893u32 => Ok(GeneratedStruct59 {
                segment_amount: 3i32,
                facing: Direction::South,
            }),
            25894u32 => Ok(GeneratedStruct59 {
                segment_amount: 4i32,
                facing: Direction::South,
            }),
            25895u32 => Ok(GeneratedStruct59 {
                facing: Direction::West,
                segment_amount: 1i32,
            }),
            25896u32 => Ok(GeneratedStruct59 {
                segment_amount: 2i32,
                facing: Direction::West,
            }),
            25897u32 => Ok(GeneratedStruct59 {
                facing: Direction::West,
                segment_amount: 3i32,
            }),
            25898u32 => Ok(GeneratedStruct59 {
                segment_amount: 4i32,
                facing: Direction::West,
            }),
            25899u32 => Ok(GeneratedStruct59 {
                segment_amount: 1i32,
                facing: Direction::East,
            }),
            25900u32 => Ok(GeneratedStruct59 {
                facing: Direction::East,
                segment_amount: 2i32,
            }),
            25901u32 => Ok(GeneratedStruct59 {
                facing: Direction::East,
                segment_amount: 3i32,
            }),
            25902u32 => Ok(GeneratedStruct59 {
                facing: Direction::East,
                segment_amount: 4i32,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct59 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct59 {
                segment_amount: 1i32,
                facing: Direction::North,
            } => Ok(25887u32),
            GeneratedStruct59 {
                facing: Direction::North,
                segment_amount: 2i32,
            } => Ok(25888u32),
            GeneratedStruct59 {
                segment_amount: 3i32,
                facing: Direction::North,
            } => Ok(25889u32),
            GeneratedStruct59 {
                segment_amount: 4i32,
                facing: Direction::North,
            } => Ok(25890u32),
            GeneratedStruct59 {
                segment_amount: 1i32,
                facing: Direction::South,
            } => Ok(25891u32),
            GeneratedStruct59 {
                segment_amount: 2i32,
                facing: Direction::South,
            } => Ok(25892u32),
            GeneratedStruct59 {
                segment_amount: 3i32,
                facing: Direction::South,
            } => Ok(25893u32),
            GeneratedStruct59 {
                segment_amount: 4i32,
                facing: Direction::South,
            } => Ok(25894u32),
            GeneratedStruct59 {
                facing: Direction::West,
                segment_amount: 1i32,
            } => Ok(25895u32),
            GeneratedStruct59 {
                segment_amount: 2i32,
                facing: Direction::West,
            } => Ok(25896u32),
            GeneratedStruct59 {
                facing: Direction::West,
                segment_amount: 3i32,
            } => Ok(25897u32),
            GeneratedStruct59 {
                segment_amount: 4i32,
                facing: Direction::West,
            } => Ok(25898u32),
            GeneratedStruct59 {
                segment_amount: 1i32,
                facing: Direction::East,
            } => Ok(25899u32),
            GeneratedStruct59 {
                facing: Direction::East,
                segment_amount: 2i32,
            } => Ok(25900u32),
            GeneratedStruct59 {
                facing: Direction::East,
                segment_amount: 3i32,
            } => Ok(25901u32),
            GeneratedStruct59 {
                facing: Direction::East,
                segment_amount: 4i32,
            } => Ok(25902u32),
            _ => Err(()),
        }
    }
}

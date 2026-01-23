#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GeneratedStruct59 {
    pub facing: Direction,
    pub segment_amount: i32,
}
impl GeneratedStruct59 {
    pub(crate) const VTABLE: crate::BlockBehaviorTable =
        crate::BlockBehaviorTable::from::<GeneratedStruct59>();
}
impl TryFrom<u32> for GeneratedStruct59 {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            25887u32 => Ok(GeneratedStruct59 {
                facing: Direction::North,
                segment_amount: 1i32,
            }),
            25888u32 => Ok(GeneratedStruct59 {
                facing: Direction::North,
                segment_amount: 2i32,
            }),
            25889u32 => Ok(GeneratedStruct59 {
                facing: Direction::North,
                segment_amount: 3i32,
            }),
            25890u32 => Ok(GeneratedStruct59 {
                facing: Direction::North,
                segment_amount: 4i32,
            }),
            25891u32 => Ok(GeneratedStruct59 {
                facing: Direction::South,
                segment_amount: 1i32,
            }),
            25892u32 => Ok(GeneratedStruct59 {
                facing: Direction::South,
                segment_amount: 2i32,
            }),
            25893u32 => Ok(GeneratedStruct59 {
                facing: Direction::South,
                segment_amount: 3i32,
            }),
            25894u32 => Ok(GeneratedStruct59 {
                facing: Direction::South,
                segment_amount: 4i32,
            }),
            25895u32 => Ok(GeneratedStruct59 {
                facing: Direction::West,
                segment_amount: 1i32,
            }),
            25896u32 => Ok(GeneratedStruct59 {
                facing: Direction::West,
                segment_amount: 2i32,
            }),
            25897u32 => Ok(GeneratedStruct59 {
                facing: Direction::West,
                segment_amount: 3i32,
            }),
            25898u32 => Ok(GeneratedStruct59 {
                facing: Direction::West,
                segment_amount: 4i32,
            }),
            25899u32 => Ok(GeneratedStruct59 {
                facing: Direction::East,
                segment_amount: 1i32,
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
                facing: Direction::North,
                segment_amount: 1i32,
            } => Ok(25887u32),
            GeneratedStruct59 {
                facing: Direction::North,
                segment_amount: 2i32,
            } => Ok(25888u32),
            GeneratedStruct59 {
                facing: Direction::North,
                segment_amount: 3i32,
            } => Ok(25889u32),
            GeneratedStruct59 {
                facing: Direction::North,
                segment_amount: 4i32,
            } => Ok(25890u32),
            GeneratedStruct59 {
                facing: Direction::South,
                segment_amount: 1i32,
            } => Ok(25891u32),
            GeneratedStruct59 {
                facing: Direction::South,
                segment_amount: 2i32,
            } => Ok(25892u32),
            GeneratedStruct59 {
                facing: Direction::South,
                segment_amount: 3i32,
            } => Ok(25893u32),
            GeneratedStruct59 {
                facing: Direction::South,
                segment_amount: 4i32,
            } => Ok(25894u32),
            GeneratedStruct59 {
                facing: Direction::West,
                segment_amount: 1i32,
            } => Ok(25895u32),
            GeneratedStruct59 {
                facing: Direction::West,
                segment_amount: 2i32,
            } => Ok(25896u32),
            GeneratedStruct59 {
                facing: Direction::West,
                segment_amount: 3i32,
            } => Ok(25897u32),
            GeneratedStruct59 {
                facing: Direction::West,
                segment_amount: 4i32,
            } => Ok(25898u32),
            GeneratedStruct59 {
                facing: Direction::East,
                segment_amount: 1i32,
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

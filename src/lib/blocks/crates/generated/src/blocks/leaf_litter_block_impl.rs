use crate::LeafLitterBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for LeafLitterBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            25887u32 => Ok(LeafLitterBlock {
                facing: Direction::North,
                segment_amount: 1i32,
            }),
            25888u32 => Ok(LeafLitterBlock {
                facing: Direction::North,
                segment_amount: 2i32,
            }),
            25889u32 => Ok(LeafLitterBlock {
                facing: Direction::North,
                segment_amount: 3i32,
            }),
            25890u32 => Ok(LeafLitterBlock {
                facing: Direction::North,
                segment_amount: 4i32,
            }),
            25891u32 => Ok(LeafLitterBlock {
                facing: Direction::South,
                segment_amount: 1i32,
            }),
            25892u32 => Ok(LeafLitterBlock {
                facing: Direction::South,
                segment_amount: 2i32,
            }),
            25893u32 => Ok(LeafLitterBlock {
                facing: Direction::South,
                segment_amount: 3i32,
            }),
            25894u32 => Ok(LeafLitterBlock {
                facing: Direction::South,
                segment_amount: 4i32,
            }),
            25895u32 => Ok(LeafLitterBlock {
                facing: Direction::West,
                segment_amount: 1i32,
            }),
            25896u32 => Ok(LeafLitterBlock {
                facing: Direction::West,
                segment_amount: 2i32,
            }),
            25897u32 => Ok(LeafLitterBlock {
                facing: Direction::West,
                segment_amount: 3i32,
            }),
            25898u32 => Ok(LeafLitterBlock {
                facing: Direction::West,
                segment_amount: 4i32,
            }),
            25899u32 => Ok(LeafLitterBlock {
                facing: Direction::East,
                segment_amount: 1i32,
            }),
            25900u32 => Ok(LeafLitterBlock {
                facing: Direction::East,
                segment_amount: 2i32,
            }),
            25901u32 => Ok(LeafLitterBlock {
                facing: Direction::East,
                segment_amount: 3i32,
            }),
            25902u32 => Ok(LeafLitterBlock {
                facing: Direction::East,
                segment_amount: 4i32,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for LeafLitterBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            LeafLitterBlock {
                facing: Direction::North,
                segment_amount: 1i32,
            } => Ok(25887u32),
            LeafLitterBlock {
                facing: Direction::North,
                segment_amount: 2i32,
            } => Ok(25888u32),
            LeafLitterBlock {
                facing: Direction::North,
                segment_amount: 3i32,
            } => Ok(25889u32),
            LeafLitterBlock {
                facing: Direction::North,
                segment_amount: 4i32,
            } => Ok(25890u32),
            LeafLitterBlock {
                facing: Direction::South,
                segment_amount: 1i32,
            } => Ok(25891u32),
            LeafLitterBlock {
                facing: Direction::South,
                segment_amount: 2i32,
            } => Ok(25892u32),
            LeafLitterBlock {
                facing: Direction::South,
                segment_amount: 3i32,
            } => Ok(25893u32),
            LeafLitterBlock {
                facing: Direction::South,
                segment_amount: 4i32,
            } => Ok(25894u32),
            LeafLitterBlock {
                facing: Direction::West,
                segment_amount: 1i32,
            } => Ok(25895u32),
            LeafLitterBlock {
                facing: Direction::West,
                segment_amount: 2i32,
            } => Ok(25896u32),
            LeafLitterBlock {
                facing: Direction::West,
                segment_amount: 3i32,
            } => Ok(25897u32),
            LeafLitterBlock {
                facing: Direction::West,
                segment_amount: 4i32,
            } => Ok(25898u32),
            LeafLitterBlock {
                facing: Direction::East,
                segment_amount: 1i32,
            } => Ok(25899u32),
            LeafLitterBlock {
                facing: Direction::East,
                segment_amount: 2i32,
            } => Ok(25900u32),
            LeafLitterBlock {
                facing: Direction::East,
                segment_amount: 3i32,
            } => Ok(25901u32),
            LeafLitterBlock {
                facing: Direction::East,
                segment_amount: 4i32,
            } => Ok(25902u32),
            _ => Err(()),
        }
    }
}

use crate::FlowerCoverBlock;
use crate::FlowerCoverBlockType;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for FlowerCoverBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            25855u32 => Ok(FlowerCoverBlock {
                block_type: FlowerCoverBlockType::PinkPetals,
                facing: Direction::North,
                flower_amount: 1i32,
            }),
            25856u32 => Ok(FlowerCoverBlock {
                block_type: FlowerCoverBlockType::PinkPetals,
                facing: Direction::North,
                flower_amount: 2i32,
            }),
            25857u32 => Ok(FlowerCoverBlock {
                block_type: FlowerCoverBlockType::PinkPetals,
                facing: Direction::North,
                flower_amount: 3i32,
            }),
            25858u32 => Ok(FlowerCoverBlock {
                block_type: FlowerCoverBlockType::PinkPetals,
                facing: Direction::North,
                flower_amount: 4i32,
            }),
            25859u32 => Ok(FlowerCoverBlock {
                block_type: FlowerCoverBlockType::PinkPetals,
                facing: Direction::South,
                flower_amount: 1i32,
            }),
            25860u32 => Ok(FlowerCoverBlock {
                block_type: FlowerCoverBlockType::PinkPetals,
                facing: Direction::South,
                flower_amount: 2i32,
            }),
            25861u32 => Ok(FlowerCoverBlock {
                block_type: FlowerCoverBlockType::PinkPetals,
                facing: Direction::South,
                flower_amount: 3i32,
            }),
            25862u32 => Ok(FlowerCoverBlock {
                block_type: FlowerCoverBlockType::PinkPetals,
                facing: Direction::South,
                flower_amount: 4i32,
            }),
            25863u32 => Ok(FlowerCoverBlock {
                block_type: FlowerCoverBlockType::PinkPetals,
                facing: Direction::West,
                flower_amount: 1i32,
            }),
            25864u32 => Ok(FlowerCoverBlock {
                block_type: FlowerCoverBlockType::PinkPetals,
                facing: Direction::West,
                flower_amount: 2i32,
            }),
            25865u32 => Ok(FlowerCoverBlock {
                block_type: FlowerCoverBlockType::PinkPetals,
                facing: Direction::West,
                flower_amount: 3i32,
            }),
            25866u32 => Ok(FlowerCoverBlock {
                block_type: FlowerCoverBlockType::PinkPetals,
                facing: Direction::West,
                flower_amount: 4i32,
            }),
            25867u32 => Ok(FlowerCoverBlock {
                block_type: FlowerCoverBlockType::PinkPetals,
                facing: Direction::East,
                flower_amount: 1i32,
            }),
            25868u32 => Ok(FlowerCoverBlock {
                block_type: FlowerCoverBlockType::PinkPetals,
                facing: Direction::East,
                flower_amount: 2i32,
            }),
            25869u32 => Ok(FlowerCoverBlock {
                block_type: FlowerCoverBlockType::PinkPetals,
                facing: Direction::East,
                flower_amount: 3i32,
            }),
            25870u32 => Ok(FlowerCoverBlock {
                block_type: FlowerCoverBlockType::PinkPetals,
                facing: Direction::East,
                flower_amount: 4i32,
            }),
            25871u32 => Ok(FlowerCoverBlock {
                block_type: FlowerCoverBlockType::Wildflowers,
                facing: Direction::North,
                flower_amount: 1i32,
            }),
            25872u32 => Ok(FlowerCoverBlock {
                block_type: FlowerCoverBlockType::Wildflowers,
                facing: Direction::North,
                flower_amount: 2i32,
            }),
            25873u32 => Ok(FlowerCoverBlock {
                block_type: FlowerCoverBlockType::Wildflowers,
                facing: Direction::North,
                flower_amount: 3i32,
            }),
            25874u32 => Ok(FlowerCoverBlock {
                block_type: FlowerCoverBlockType::Wildflowers,
                facing: Direction::North,
                flower_amount: 4i32,
            }),
            25875u32 => Ok(FlowerCoverBlock {
                block_type: FlowerCoverBlockType::Wildflowers,
                facing: Direction::South,
                flower_amount: 1i32,
            }),
            25876u32 => Ok(FlowerCoverBlock {
                block_type: FlowerCoverBlockType::Wildflowers,
                facing: Direction::South,
                flower_amount: 2i32,
            }),
            25877u32 => Ok(FlowerCoverBlock {
                block_type: FlowerCoverBlockType::Wildflowers,
                facing: Direction::South,
                flower_amount: 3i32,
            }),
            25878u32 => Ok(FlowerCoverBlock {
                block_type: FlowerCoverBlockType::Wildflowers,
                facing: Direction::South,
                flower_amount: 4i32,
            }),
            25879u32 => Ok(FlowerCoverBlock {
                block_type: FlowerCoverBlockType::Wildflowers,
                facing: Direction::West,
                flower_amount: 1i32,
            }),
            25880u32 => Ok(FlowerCoverBlock {
                block_type: FlowerCoverBlockType::Wildflowers,
                facing: Direction::West,
                flower_amount: 2i32,
            }),
            25881u32 => Ok(FlowerCoverBlock {
                block_type: FlowerCoverBlockType::Wildflowers,
                facing: Direction::West,
                flower_amount: 3i32,
            }),
            25882u32 => Ok(FlowerCoverBlock {
                block_type: FlowerCoverBlockType::Wildflowers,
                facing: Direction::West,
                flower_amount: 4i32,
            }),
            25883u32 => Ok(FlowerCoverBlock {
                block_type: FlowerCoverBlockType::Wildflowers,
                facing: Direction::East,
                flower_amount: 1i32,
            }),
            25884u32 => Ok(FlowerCoverBlock {
                block_type: FlowerCoverBlockType::Wildflowers,
                facing: Direction::East,
                flower_amount: 2i32,
            }),
            25885u32 => Ok(FlowerCoverBlock {
                block_type: FlowerCoverBlockType::Wildflowers,
                facing: Direction::East,
                flower_amount: 3i32,
            }),
            25886u32 => Ok(FlowerCoverBlock {
                block_type: FlowerCoverBlockType::Wildflowers,
                facing: Direction::East,
                flower_amount: 4i32,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for FlowerCoverBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            FlowerCoverBlock {
                block_type: FlowerCoverBlockType::PinkPetals,
                facing: Direction::North,
                flower_amount: 1i32,
            } => Ok(25855u32),
            FlowerCoverBlock {
                block_type: FlowerCoverBlockType::PinkPetals,
                facing: Direction::North,
                flower_amount: 2i32,
            } => Ok(25856u32),
            FlowerCoverBlock {
                block_type: FlowerCoverBlockType::PinkPetals,
                facing: Direction::North,
                flower_amount: 3i32,
            } => Ok(25857u32),
            FlowerCoverBlock {
                block_type: FlowerCoverBlockType::PinkPetals,
                facing: Direction::North,
                flower_amount: 4i32,
            } => Ok(25858u32),
            FlowerCoverBlock {
                block_type: FlowerCoverBlockType::PinkPetals,
                facing: Direction::South,
                flower_amount: 1i32,
            } => Ok(25859u32),
            FlowerCoverBlock {
                block_type: FlowerCoverBlockType::PinkPetals,
                facing: Direction::South,
                flower_amount: 2i32,
            } => Ok(25860u32),
            FlowerCoverBlock {
                block_type: FlowerCoverBlockType::PinkPetals,
                facing: Direction::South,
                flower_amount: 3i32,
            } => Ok(25861u32),
            FlowerCoverBlock {
                block_type: FlowerCoverBlockType::PinkPetals,
                facing: Direction::South,
                flower_amount: 4i32,
            } => Ok(25862u32),
            FlowerCoverBlock {
                block_type: FlowerCoverBlockType::PinkPetals,
                facing: Direction::West,
                flower_amount: 1i32,
            } => Ok(25863u32),
            FlowerCoverBlock {
                block_type: FlowerCoverBlockType::PinkPetals,
                facing: Direction::West,
                flower_amount: 2i32,
            } => Ok(25864u32),
            FlowerCoverBlock {
                block_type: FlowerCoverBlockType::PinkPetals,
                facing: Direction::West,
                flower_amount: 3i32,
            } => Ok(25865u32),
            FlowerCoverBlock {
                block_type: FlowerCoverBlockType::PinkPetals,
                facing: Direction::West,
                flower_amount: 4i32,
            } => Ok(25866u32),
            FlowerCoverBlock {
                block_type: FlowerCoverBlockType::PinkPetals,
                facing: Direction::East,
                flower_amount: 1i32,
            } => Ok(25867u32),
            FlowerCoverBlock {
                block_type: FlowerCoverBlockType::PinkPetals,
                facing: Direction::East,
                flower_amount: 2i32,
            } => Ok(25868u32),
            FlowerCoverBlock {
                block_type: FlowerCoverBlockType::PinkPetals,
                facing: Direction::East,
                flower_amount: 3i32,
            } => Ok(25869u32),
            FlowerCoverBlock {
                block_type: FlowerCoverBlockType::PinkPetals,
                facing: Direction::East,
                flower_amount: 4i32,
            } => Ok(25870u32),
            FlowerCoverBlock {
                block_type: FlowerCoverBlockType::Wildflowers,
                facing: Direction::North,
                flower_amount: 1i32,
            } => Ok(25871u32),
            FlowerCoverBlock {
                block_type: FlowerCoverBlockType::Wildflowers,
                facing: Direction::North,
                flower_amount: 2i32,
            } => Ok(25872u32),
            FlowerCoverBlock {
                block_type: FlowerCoverBlockType::Wildflowers,
                facing: Direction::North,
                flower_amount: 3i32,
            } => Ok(25873u32),
            FlowerCoverBlock {
                block_type: FlowerCoverBlockType::Wildflowers,
                facing: Direction::North,
                flower_amount: 4i32,
            } => Ok(25874u32),
            FlowerCoverBlock {
                block_type: FlowerCoverBlockType::Wildflowers,
                facing: Direction::South,
                flower_amount: 1i32,
            } => Ok(25875u32),
            FlowerCoverBlock {
                block_type: FlowerCoverBlockType::Wildflowers,
                facing: Direction::South,
                flower_amount: 2i32,
            } => Ok(25876u32),
            FlowerCoverBlock {
                block_type: FlowerCoverBlockType::Wildflowers,
                facing: Direction::South,
                flower_amount: 3i32,
            } => Ok(25877u32),
            FlowerCoverBlock {
                block_type: FlowerCoverBlockType::Wildflowers,
                facing: Direction::South,
                flower_amount: 4i32,
            } => Ok(25878u32),
            FlowerCoverBlock {
                block_type: FlowerCoverBlockType::Wildflowers,
                facing: Direction::West,
                flower_amount: 1i32,
            } => Ok(25879u32),
            FlowerCoverBlock {
                block_type: FlowerCoverBlockType::Wildflowers,
                facing: Direction::West,
                flower_amount: 2i32,
            } => Ok(25880u32),
            FlowerCoverBlock {
                block_type: FlowerCoverBlockType::Wildflowers,
                facing: Direction::West,
                flower_amount: 3i32,
            } => Ok(25881u32),
            FlowerCoverBlock {
                block_type: FlowerCoverBlockType::Wildflowers,
                facing: Direction::West,
                flower_amount: 4i32,
            } => Ok(25882u32),
            FlowerCoverBlock {
                block_type: FlowerCoverBlockType::Wildflowers,
                facing: Direction::East,
                flower_amount: 1i32,
            } => Ok(25883u32),
            FlowerCoverBlock {
                block_type: FlowerCoverBlockType::Wildflowers,
                facing: Direction::East,
                flower_amount: 2i32,
            } => Ok(25884u32),
            FlowerCoverBlock {
                block_type: FlowerCoverBlockType::Wildflowers,
                facing: Direction::East,
                flower_amount: 3i32,
            } => Ok(25885u32),
            FlowerCoverBlock {
                block_type: FlowerCoverBlockType::Wildflowers,
                facing: Direction::East,
                flower_amount: 4i32,
            } => Ok(25886u32),
            _ => Err(()),
        }
    }
}

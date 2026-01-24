use crate::FurnaceBlock;
use crate::FurnaceBlockType;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for FurnaceBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            19451u32 => Ok(FurnaceBlock {
                block_type: FurnaceBlockType::BlastFurnace,
                facing: Direction::North,
                lit: true,
            }),
            19452u32 => Ok(FurnaceBlock {
                block_type: FurnaceBlockType::BlastFurnace,
                facing: Direction::North,
                lit: false,
            }),
            19453u32 => Ok(FurnaceBlock {
                block_type: FurnaceBlockType::BlastFurnace,
                facing: Direction::South,
                lit: true,
            }),
            19454u32 => Ok(FurnaceBlock {
                block_type: FurnaceBlockType::BlastFurnace,
                facing: Direction::South,
                lit: false,
            }),
            19455u32 => Ok(FurnaceBlock {
                block_type: FurnaceBlockType::BlastFurnace,
                facing: Direction::West,
                lit: true,
            }),
            19456u32 => Ok(FurnaceBlock {
                block_type: FurnaceBlockType::BlastFurnace,
                facing: Direction::West,
                lit: false,
            }),
            19457u32 => Ok(FurnaceBlock {
                block_type: FurnaceBlockType::BlastFurnace,
                facing: Direction::East,
                lit: true,
            }),
            19458u32 => Ok(FurnaceBlock {
                block_type: FurnaceBlockType::BlastFurnace,
                facing: Direction::East,
                lit: false,
            }),
            4358u32 => Ok(FurnaceBlock {
                block_type: FurnaceBlockType::Furnace,
                facing: Direction::North,
                lit: true,
            }),
            4359u32 => Ok(FurnaceBlock {
                block_type: FurnaceBlockType::Furnace,
                facing: Direction::North,
                lit: false,
            }),
            4360u32 => Ok(FurnaceBlock {
                block_type: FurnaceBlockType::Furnace,
                facing: Direction::South,
                lit: true,
            }),
            4361u32 => Ok(FurnaceBlock {
                block_type: FurnaceBlockType::Furnace,
                facing: Direction::South,
                lit: false,
            }),
            4362u32 => Ok(FurnaceBlock {
                block_type: FurnaceBlockType::Furnace,
                facing: Direction::West,
                lit: true,
            }),
            4363u32 => Ok(FurnaceBlock {
                block_type: FurnaceBlockType::Furnace,
                facing: Direction::West,
                lit: false,
            }),
            4364u32 => Ok(FurnaceBlock {
                block_type: FurnaceBlockType::Furnace,
                facing: Direction::East,
                lit: true,
            }),
            4365u32 => Ok(FurnaceBlock {
                block_type: FurnaceBlockType::Furnace,
                facing: Direction::East,
                lit: false,
            }),
            19443u32 => Ok(FurnaceBlock {
                block_type: FurnaceBlockType::Smoker,
                facing: Direction::North,
                lit: true,
            }),
            19444u32 => Ok(FurnaceBlock {
                block_type: FurnaceBlockType::Smoker,
                facing: Direction::North,
                lit: false,
            }),
            19445u32 => Ok(FurnaceBlock {
                block_type: FurnaceBlockType::Smoker,
                facing: Direction::South,
                lit: true,
            }),
            19446u32 => Ok(FurnaceBlock {
                block_type: FurnaceBlockType::Smoker,
                facing: Direction::South,
                lit: false,
            }),
            19447u32 => Ok(FurnaceBlock {
                block_type: FurnaceBlockType::Smoker,
                facing: Direction::West,
                lit: true,
            }),
            19448u32 => Ok(FurnaceBlock {
                block_type: FurnaceBlockType::Smoker,
                facing: Direction::West,
                lit: false,
            }),
            19449u32 => Ok(FurnaceBlock {
                block_type: FurnaceBlockType::Smoker,
                facing: Direction::East,
                lit: true,
            }),
            19450u32 => Ok(FurnaceBlock {
                block_type: FurnaceBlockType::Smoker,
                facing: Direction::East,
                lit: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for FurnaceBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            FurnaceBlock {
                block_type: FurnaceBlockType::BlastFurnace,
                facing: Direction::North,
                lit: true,
            } => Ok(19451u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::BlastFurnace,
                facing: Direction::North,
                lit: false,
            } => Ok(19452u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::BlastFurnace,
                facing: Direction::South,
                lit: true,
            } => Ok(19453u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::BlastFurnace,
                facing: Direction::South,
                lit: false,
            } => Ok(19454u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::BlastFurnace,
                facing: Direction::West,
                lit: true,
            } => Ok(19455u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::BlastFurnace,
                facing: Direction::West,
                lit: false,
            } => Ok(19456u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::BlastFurnace,
                facing: Direction::East,
                lit: true,
            } => Ok(19457u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::BlastFurnace,
                facing: Direction::East,
                lit: false,
            } => Ok(19458u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::Furnace,
                facing: Direction::North,
                lit: true,
            } => Ok(4358u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::Furnace,
                facing: Direction::North,
                lit: false,
            } => Ok(4359u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::Furnace,
                facing: Direction::South,
                lit: true,
            } => Ok(4360u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::Furnace,
                facing: Direction::South,
                lit: false,
            } => Ok(4361u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::Furnace,
                facing: Direction::West,
                lit: true,
            } => Ok(4362u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::Furnace,
                facing: Direction::West,
                lit: false,
            } => Ok(4363u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::Furnace,
                facing: Direction::East,
                lit: true,
            } => Ok(4364u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::Furnace,
                facing: Direction::East,
                lit: false,
            } => Ok(4365u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::Smoker,
                facing: Direction::North,
                lit: true,
            } => Ok(19443u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::Smoker,
                facing: Direction::North,
                lit: false,
            } => Ok(19444u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::Smoker,
                facing: Direction::South,
                lit: true,
            } => Ok(19445u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::Smoker,
                facing: Direction::South,
                lit: false,
            } => Ok(19446u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::Smoker,
                facing: Direction::West,
                lit: true,
            } => Ok(19447u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::Smoker,
                facing: Direction::West,
                lit: false,
            } => Ok(19448u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::Smoker,
                facing: Direction::East,
                lit: true,
            } => Ok(19449u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::Smoker,
                facing: Direction::East,
                lit: false,
            } => Ok(19450u32),
            _ => Err(()),
        }
    }
}

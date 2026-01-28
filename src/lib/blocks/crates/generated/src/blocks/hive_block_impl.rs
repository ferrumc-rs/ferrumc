use crate::HiveBlock;
use crate::HiveBlockType;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for HiveBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            20425u32 => Ok(HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::North,
                honey_level: 0i32,
            }),
            20426u32 => Ok(HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::North,
                honey_level: 1i32,
            }),
            20427u32 => Ok(HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::North,
                honey_level: 2i32,
            }),
            20428u32 => Ok(HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::North,
                honey_level: 3i32,
            }),
            20429u32 => Ok(HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::North,
                honey_level: 4i32,
            }),
            20430u32 => Ok(HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::North,
                honey_level: 5i32,
            }),
            20431u32 => Ok(HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::South,
                honey_level: 0i32,
            }),
            20432u32 => Ok(HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::South,
                honey_level: 1i32,
            }),
            20433u32 => Ok(HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::South,
                honey_level: 2i32,
            }),
            20434u32 => Ok(HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::South,
                honey_level: 3i32,
            }),
            20435u32 => Ok(HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::South,
                honey_level: 4i32,
            }),
            20436u32 => Ok(HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::South,
                honey_level: 5i32,
            }),
            20437u32 => Ok(HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::West,
                honey_level: 0i32,
            }),
            20438u32 => Ok(HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::West,
                honey_level: 1i32,
            }),
            20439u32 => Ok(HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::West,
                honey_level: 2i32,
            }),
            20440u32 => Ok(HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::West,
                honey_level: 3i32,
            }),
            20441u32 => Ok(HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::West,
                honey_level: 4i32,
            }),
            20442u32 => Ok(HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::West,
                honey_level: 5i32,
            }),
            20443u32 => Ok(HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::East,
                honey_level: 0i32,
            }),
            20444u32 => Ok(HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::East,
                honey_level: 1i32,
            }),
            20445u32 => Ok(HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::East,
                honey_level: 2i32,
            }),
            20446u32 => Ok(HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::East,
                honey_level: 3i32,
            }),
            20447u32 => Ok(HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::East,
                honey_level: 4i32,
            }),
            20448u32 => Ok(HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::East,
                honey_level: 5i32,
            }),
            20449u32 => Ok(HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::North,
                honey_level: 0i32,
            }),
            20450u32 => Ok(HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::North,
                honey_level: 1i32,
            }),
            20451u32 => Ok(HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::North,
                honey_level: 2i32,
            }),
            20452u32 => Ok(HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::North,
                honey_level: 3i32,
            }),
            20453u32 => Ok(HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::North,
                honey_level: 4i32,
            }),
            20454u32 => Ok(HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::North,
                honey_level: 5i32,
            }),
            20455u32 => Ok(HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::South,
                honey_level: 0i32,
            }),
            20456u32 => Ok(HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::South,
                honey_level: 1i32,
            }),
            20457u32 => Ok(HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::South,
                honey_level: 2i32,
            }),
            20458u32 => Ok(HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::South,
                honey_level: 3i32,
            }),
            20459u32 => Ok(HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::South,
                honey_level: 4i32,
            }),
            20460u32 => Ok(HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::South,
                honey_level: 5i32,
            }),
            20461u32 => Ok(HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::West,
                honey_level: 0i32,
            }),
            20462u32 => Ok(HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::West,
                honey_level: 1i32,
            }),
            20463u32 => Ok(HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::West,
                honey_level: 2i32,
            }),
            20464u32 => Ok(HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::West,
                honey_level: 3i32,
            }),
            20465u32 => Ok(HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::West,
                honey_level: 4i32,
            }),
            20466u32 => Ok(HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::West,
                honey_level: 5i32,
            }),
            20467u32 => Ok(HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::East,
                honey_level: 0i32,
            }),
            20468u32 => Ok(HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::East,
                honey_level: 1i32,
            }),
            20469u32 => Ok(HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::East,
                honey_level: 2i32,
            }),
            20470u32 => Ok(HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::East,
                honey_level: 3i32,
            }),
            20471u32 => Ok(HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::East,
                honey_level: 4i32,
            }),
            20472u32 => Ok(HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::East,
                honey_level: 5i32,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for HiveBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::North,
                honey_level: 0i32,
            } => Ok(20425u32),
            HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::North,
                honey_level: 1i32,
            } => Ok(20426u32),
            HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::North,
                honey_level: 2i32,
            } => Ok(20427u32),
            HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::North,
                honey_level: 3i32,
            } => Ok(20428u32),
            HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::North,
                honey_level: 4i32,
            } => Ok(20429u32),
            HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::North,
                honey_level: 5i32,
            } => Ok(20430u32),
            HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::South,
                honey_level: 0i32,
            } => Ok(20431u32),
            HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::South,
                honey_level: 1i32,
            } => Ok(20432u32),
            HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::South,
                honey_level: 2i32,
            } => Ok(20433u32),
            HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::South,
                honey_level: 3i32,
            } => Ok(20434u32),
            HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::South,
                honey_level: 4i32,
            } => Ok(20435u32),
            HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::South,
                honey_level: 5i32,
            } => Ok(20436u32),
            HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::West,
                honey_level: 0i32,
            } => Ok(20437u32),
            HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::West,
                honey_level: 1i32,
            } => Ok(20438u32),
            HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::West,
                honey_level: 2i32,
            } => Ok(20439u32),
            HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::West,
                honey_level: 3i32,
            } => Ok(20440u32),
            HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::West,
                honey_level: 4i32,
            } => Ok(20441u32),
            HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::West,
                honey_level: 5i32,
            } => Ok(20442u32),
            HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::East,
                honey_level: 0i32,
            } => Ok(20443u32),
            HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::East,
                honey_level: 1i32,
            } => Ok(20444u32),
            HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::East,
                honey_level: 2i32,
            } => Ok(20445u32),
            HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::East,
                honey_level: 3i32,
            } => Ok(20446u32),
            HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::East,
                honey_level: 4i32,
            } => Ok(20447u32),
            HiveBlock {
                block_type: HiveBlockType::BeeNest,
                facing: Direction::East,
                honey_level: 5i32,
            } => Ok(20448u32),
            HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::North,
                honey_level: 0i32,
            } => Ok(20449u32),
            HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::North,
                honey_level: 1i32,
            } => Ok(20450u32),
            HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::North,
                honey_level: 2i32,
            } => Ok(20451u32),
            HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::North,
                honey_level: 3i32,
            } => Ok(20452u32),
            HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::North,
                honey_level: 4i32,
            } => Ok(20453u32),
            HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::North,
                honey_level: 5i32,
            } => Ok(20454u32),
            HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::South,
                honey_level: 0i32,
            } => Ok(20455u32),
            HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::South,
                honey_level: 1i32,
            } => Ok(20456u32),
            HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::South,
                honey_level: 2i32,
            } => Ok(20457u32),
            HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::South,
                honey_level: 3i32,
            } => Ok(20458u32),
            HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::South,
                honey_level: 4i32,
            } => Ok(20459u32),
            HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::South,
                honey_level: 5i32,
            } => Ok(20460u32),
            HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::West,
                honey_level: 0i32,
            } => Ok(20461u32),
            HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::West,
                honey_level: 1i32,
            } => Ok(20462u32),
            HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::West,
                honey_level: 2i32,
            } => Ok(20463u32),
            HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::West,
                honey_level: 3i32,
            } => Ok(20464u32),
            HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::West,
                honey_level: 4i32,
            } => Ok(20465u32),
            HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::West,
                honey_level: 5i32,
            } => Ok(20466u32),
            HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::East,
                honey_level: 0i32,
            } => Ok(20467u32),
            HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::East,
                honey_level: 1i32,
            } => Ok(20468u32),
            HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::East,
                honey_level: 2i32,
            } => Ok(20469u32),
            HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::East,
                honey_level: 3i32,
            } => Ok(20470u32),
            HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::East,
                honey_level: 4i32,
            } => Ok(20471u32),
            HiveBlock {
                block_type: HiveBlockType::Beehive,
                facing: Direction::East,
                honey_level: 5i32,
            } => Ok(20472u32),
            _ => Err(()),
        }
    }
}

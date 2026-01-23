#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum WallSkullBlockType {
    CreeperWallHead,
    DragonWallHead,
    Observer,
    PiglinWallHead,
    PlayerWallHead,
    SkeletonWallSkull,
    WitherSkeletonWallSkull,
    ZombieWallHead,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct WallSkullBlock {
    pub block_type: WallSkullBlockType,
    pub facing: Direction,
    pub powered: bool,
}
impl WallSkullBlock {
    pub(crate) const VTABLE: crate::BlockBehaviorTable =
        crate::BlockBehaviorTable::from::<WallSkullBlock>();
}
impl TryFrom<u32> for WallSkullBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            9828u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::CreeperWallHead,
                facing: Direction::North,
                powered: true,
            }),
            9829u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::CreeperWallHead,
                facing: Direction::North,
                powered: false,
            }),
            9830u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::CreeperWallHead,
                facing: Direction::South,
                powered: true,
            }),
            9831u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::CreeperWallHead,
                facing: Direction::South,
                powered: false,
            }),
            9832u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::CreeperWallHead,
                facing: Direction::West,
                powered: true,
            }),
            9833u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::CreeperWallHead,
                facing: Direction::West,
                powered: false,
            }),
            9834u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::CreeperWallHead,
                facing: Direction::East,
                powered: true,
            }),
            9835u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::CreeperWallHead,
                facing: Direction::East,
                powered: false,
            }),
            9868u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::DragonWallHead,
                facing: Direction::North,
                powered: true,
            }),
            9869u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::DragonWallHead,
                facing: Direction::North,
                powered: false,
            }),
            9870u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::DragonWallHead,
                facing: Direction::South,
                powered: true,
            }),
            9871u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::DragonWallHead,
                facing: Direction::South,
                powered: false,
            }),
            9872u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::DragonWallHead,
                facing: Direction::West,
                powered: true,
            }),
            9873u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::DragonWallHead,
                facing: Direction::West,
                powered: false,
            }),
            9874u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::DragonWallHead,
                facing: Direction::East,
                powered: true,
            }),
            9875u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::DragonWallHead,
                facing: Direction::East,
                powered: false,
            }),
            13573u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::Observer,
                facing: Direction::North,
                powered: true,
            }),
            13574u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::Observer,
                facing: Direction::North,
                powered: false,
            }),
            13575u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::Observer,
                facing: Direction::East,
                powered: true,
            }),
            13576u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::Observer,
                facing: Direction::East,
                powered: false,
            }),
            13577u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::Observer,
                facing: Direction::South,
                powered: true,
            }),
            13578u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::Observer,
                facing: Direction::South,
                powered: false,
            }),
            13579u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::Observer,
                facing: Direction::West,
                powered: true,
            }),
            13580u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::Observer,
                facing: Direction::West,
                powered: false,
            }),
            13581u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::Observer,
                facing: Direction::Up,
                powered: true,
            }),
            13582u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::Observer,
                facing: Direction::Up,
                powered: false,
            }),
            13583u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::Observer,
                facing: Direction::Down,
                powered: true,
            }),
            13584u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::Observer,
                facing: Direction::Down,
                powered: false,
            }),
            9908u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::PiglinWallHead,
                facing: Direction::North,
                powered: true,
            }),
            9909u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::PiglinWallHead,
                facing: Direction::North,
                powered: false,
            }),
            9910u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::PiglinWallHead,
                facing: Direction::South,
                powered: true,
            }),
            9911u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::PiglinWallHead,
                facing: Direction::South,
                powered: false,
            }),
            9912u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::PiglinWallHead,
                facing: Direction::West,
                powered: true,
            }),
            9913u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::PiglinWallHead,
                facing: Direction::West,
                powered: false,
            }),
            9914u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::PiglinWallHead,
                facing: Direction::East,
                powered: true,
            }),
            9915u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::PiglinWallHead,
                facing: Direction::East,
                powered: false,
            }),
            9788u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::PlayerWallHead,
                facing: Direction::North,
                powered: true,
            }),
            9789u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::PlayerWallHead,
                facing: Direction::North,
                powered: false,
            }),
            9790u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::PlayerWallHead,
                facing: Direction::South,
                powered: true,
            }),
            9791u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::PlayerWallHead,
                facing: Direction::South,
                powered: false,
            }),
            9792u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::PlayerWallHead,
                facing: Direction::West,
                powered: true,
            }),
            9793u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::PlayerWallHead,
                facing: Direction::West,
                powered: false,
            }),
            9794u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::PlayerWallHead,
                facing: Direction::East,
                powered: true,
            }),
            9795u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::PlayerWallHead,
                facing: Direction::East,
                powered: false,
            }),
            9668u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::SkeletonWallSkull,
                facing: Direction::North,
                powered: true,
            }),
            9669u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::SkeletonWallSkull,
                facing: Direction::North,
                powered: false,
            }),
            9670u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::SkeletonWallSkull,
                facing: Direction::South,
                powered: true,
            }),
            9671u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::SkeletonWallSkull,
                facing: Direction::South,
                powered: false,
            }),
            9672u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::SkeletonWallSkull,
                facing: Direction::West,
                powered: true,
            }),
            9673u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::SkeletonWallSkull,
                facing: Direction::West,
                powered: false,
            }),
            9674u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::SkeletonWallSkull,
                facing: Direction::East,
                powered: true,
            }),
            9675u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::SkeletonWallSkull,
                facing: Direction::East,
                powered: false,
            }),
            9708u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::WitherSkeletonWallSkull,
                facing: Direction::North,
                powered: true,
            }),
            9709u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::WitherSkeletonWallSkull,
                facing: Direction::North,
                powered: false,
            }),
            9710u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::WitherSkeletonWallSkull,
                facing: Direction::South,
                powered: true,
            }),
            9711u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::WitherSkeletonWallSkull,
                facing: Direction::South,
                powered: false,
            }),
            9712u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::WitherSkeletonWallSkull,
                facing: Direction::West,
                powered: true,
            }),
            9713u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::WitherSkeletonWallSkull,
                facing: Direction::West,
                powered: false,
            }),
            9714u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::WitherSkeletonWallSkull,
                facing: Direction::East,
                powered: true,
            }),
            9715u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::WitherSkeletonWallSkull,
                facing: Direction::East,
                powered: false,
            }),
            9748u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::ZombieWallHead,
                facing: Direction::North,
                powered: true,
            }),
            9749u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::ZombieWallHead,
                facing: Direction::North,
                powered: false,
            }),
            9750u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::ZombieWallHead,
                facing: Direction::South,
                powered: true,
            }),
            9751u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::ZombieWallHead,
                facing: Direction::South,
                powered: false,
            }),
            9752u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::ZombieWallHead,
                facing: Direction::West,
                powered: true,
            }),
            9753u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::ZombieWallHead,
                facing: Direction::West,
                powered: false,
            }),
            9754u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::ZombieWallHead,
                facing: Direction::East,
                powered: true,
            }),
            9755u32 => Ok(WallSkullBlock {
                block_type: WallSkullBlockType::ZombieWallHead,
                facing: Direction::East,
                powered: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for WallSkullBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            WallSkullBlock {
                block_type: WallSkullBlockType::CreeperWallHead,
                facing: Direction::North,
                powered: true,
            } => Ok(9828u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::CreeperWallHead,
                facing: Direction::North,
                powered: false,
            } => Ok(9829u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::CreeperWallHead,
                facing: Direction::South,
                powered: true,
            } => Ok(9830u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::CreeperWallHead,
                facing: Direction::South,
                powered: false,
            } => Ok(9831u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::CreeperWallHead,
                facing: Direction::West,
                powered: true,
            } => Ok(9832u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::CreeperWallHead,
                facing: Direction::West,
                powered: false,
            } => Ok(9833u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::CreeperWallHead,
                facing: Direction::East,
                powered: true,
            } => Ok(9834u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::CreeperWallHead,
                facing: Direction::East,
                powered: false,
            } => Ok(9835u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::DragonWallHead,
                facing: Direction::North,
                powered: true,
            } => Ok(9868u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::DragonWallHead,
                facing: Direction::North,
                powered: false,
            } => Ok(9869u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::DragonWallHead,
                facing: Direction::South,
                powered: true,
            } => Ok(9870u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::DragonWallHead,
                facing: Direction::South,
                powered: false,
            } => Ok(9871u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::DragonWallHead,
                facing: Direction::West,
                powered: true,
            } => Ok(9872u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::DragonWallHead,
                facing: Direction::West,
                powered: false,
            } => Ok(9873u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::DragonWallHead,
                facing: Direction::East,
                powered: true,
            } => Ok(9874u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::DragonWallHead,
                facing: Direction::East,
                powered: false,
            } => Ok(9875u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::Observer,
                facing: Direction::North,
                powered: true,
            } => Ok(13573u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::Observer,
                facing: Direction::North,
                powered: false,
            } => Ok(13574u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::Observer,
                facing: Direction::East,
                powered: true,
            } => Ok(13575u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::Observer,
                facing: Direction::East,
                powered: false,
            } => Ok(13576u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::Observer,
                facing: Direction::South,
                powered: true,
            } => Ok(13577u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::Observer,
                facing: Direction::South,
                powered: false,
            } => Ok(13578u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::Observer,
                facing: Direction::West,
                powered: true,
            } => Ok(13579u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::Observer,
                facing: Direction::West,
                powered: false,
            } => Ok(13580u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::Observer,
                facing: Direction::Up,
                powered: true,
            } => Ok(13581u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::Observer,
                facing: Direction::Up,
                powered: false,
            } => Ok(13582u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::Observer,
                facing: Direction::Down,
                powered: true,
            } => Ok(13583u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::Observer,
                facing: Direction::Down,
                powered: false,
            } => Ok(13584u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::PiglinWallHead,
                facing: Direction::North,
                powered: true,
            } => Ok(9908u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::PiglinWallHead,
                facing: Direction::North,
                powered: false,
            } => Ok(9909u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::PiglinWallHead,
                facing: Direction::South,
                powered: true,
            } => Ok(9910u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::PiglinWallHead,
                facing: Direction::South,
                powered: false,
            } => Ok(9911u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::PiglinWallHead,
                facing: Direction::West,
                powered: true,
            } => Ok(9912u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::PiglinWallHead,
                facing: Direction::West,
                powered: false,
            } => Ok(9913u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::PiglinWallHead,
                facing: Direction::East,
                powered: true,
            } => Ok(9914u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::PiglinWallHead,
                facing: Direction::East,
                powered: false,
            } => Ok(9915u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::PlayerWallHead,
                facing: Direction::North,
                powered: true,
            } => Ok(9788u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::PlayerWallHead,
                facing: Direction::North,
                powered: false,
            } => Ok(9789u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::PlayerWallHead,
                facing: Direction::South,
                powered: true,
            } => Ok(9790u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::PlayerWallHead,
                facing: Direction::South,
                powered: false,
            } => Ok(9791u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::PlayerWallHead,
                facing: Direction::West,
                powered: true,
            } => Ok(9792u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::PlayerWallHead,
                facing: Direction::West,
                powered: false,
            } => Ok(9793u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::PlayerWallHead,
                facing: Direction::East,
                powered: true,
            } => Ok(9794u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::PlayerWallHead,
                facing: Direction::East,
                powered: false,
            } => Ok(9795u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::SkeletonWallSkull,
                facing: Direction::North,
                powered: true,
            } => Ok(9668u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::SkeletonWallSkull,
                facing: Direction::North,
                powered: false,
            } => Ok(9669u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::SkeletonWallSkull,
                facing: Direction::South,
                powered: true,
            } => Ok(9670u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::SkeletonWallSkull,
                facing: Direction::South,
                powered: false,
            } => Ok(9671u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::SkeletonWallSkull,
                facing: Direction::West,
                powered: true,
            } => Ok(9672u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::SkeletonWallSkull,
                facing: Direction::West,
                powered: false,
            } => Ok(9673u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::SkeletonWallSkull,
                facing: Direction::East,
                powered: true,
            } => Ok(9674u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::SkeletonWallSkull,
                facing: Direction::East,
                powered: false,
            } => Ok(9675u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::WitherSkeletonWallSkull,
                facing: Direction::North,
                powered: true,
            } => Ok(9708u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::WitherSkeletonWallSkull,
                facing: Direction::North,
                powered: false,
            } => Ok(9709u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::WitherSkeletonWallSkull,
                facing: Direction::South,
                powered: true,
            } => Ok(9710u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::WitherSkeletonWallSkull,
                facing: Direction::South,
                powered: false,
            } => Ok(9711u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::WitherSkeletonWallSkull,
                facing: Direction::West,
                powered: true,
            } => Ok(9712u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::WitherSkeletonWallSkull,
                facing: Direction::West,
                powered: false,
            } => Ok(9713u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::WitherSkeletonWallSkull,
                facing: Direction::East,
                powered: true,
            } => Ok(9714u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::WitherSkeletonWallSkull,
                facing: Direction::East,
                powered: false,
            } => Ok(9715u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::ZombieWallHead,
                facing: Direction::North,
                powered: true,
            } => Ok(9748u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::ZombieWallHead,
                facing: Direction::North,
                powered: false,
            } => Ok(9749u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::ZombieWallHead,
                facing: Direction::South,
                powered: true,
            } => Ok(9750u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::ZombieWallHead,
                facing: Direction::South,
                powered: false,
            } => Ok(9751u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::ZombieWallHead,
                facing: Direction::West,
                powered: true,
            } => Ok(9752u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::ZombieWallHead,
                facing: Direction::West,
                powered: false,
            } => Ok(9753u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::ZombieWallHead,
                facing: Direction::East,
                powered: true,
            } => Ok(9754u32),
            WallSkullBlock {
                block_type: WallSkullBlockType::ZombieWallHead,
                facing: Direction::East,
                powered: false,
            } => Ok(9755u32),
            _ => Err(()),
        }
    }
}

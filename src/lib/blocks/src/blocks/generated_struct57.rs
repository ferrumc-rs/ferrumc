#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub enum GeneratedStruct57Type {
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
pub struct GeneratedStruct57 {
    pub block_type: GeneratedStruct57Type,
    pub facing: Direction,
    pub powered: bool,
}
impl TryFrom<u32> for GeneratedStruct57 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            9828u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::CreeperWallHead,
                facing: Direction::North,
                powered: true,
            }),
            9829u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::CreeperWallHead,
                facing: Direction::North,
                powered: false,
            }),
            9830u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::CreeperWallHead,
                facing: Direction::South,
                powered: true,
            }),
            9831u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::CreeperWallHead,
                facing: Direction::South,
                powered: false,
            }),
            9832u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::CreeperWallHead,
                facing: Direction::West,
                powered: true,
            }),
            9833u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::CreeperWallHead,
                facing: Direction::West,
                powered: false,
            }),
            9834u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::CreeperWallHead,
                facing: Direction::East,
                powered: true,
            }),
            9835u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::CreeperWallHead,
                facing: Direction::East,
                powered: false,
            }),
            9868u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::DragonWallHead,
                facing: Direction::North,
                powered: true,
            }),
            9869u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::DragonWallHead,
                facing: Direction::North,
                powered: false,
            }),
            9870u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::DragonWallHead,
                facing: Direction::South,
                powered: true,
            }),
            9871u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::DragonWallHead,
                facing: Direction::South,
                powered: false,
            }),
            9872u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::DragonWallHead,
                facing: Direction::West,
                powered: true,
            }),
            9873u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::DragonWallHead,
                facing: Direction::West,
                powered: false,
            }),
            9874u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::DragonWallHead,
                facing: Direction::East,
                powered: true,
            }),
            9875u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::DragonWallHead,
                facing: Direction::East,
                powered: false,
            }),
            13573u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::Observer,
                facing: Direction::North,
                powered: true,
            }),
            13574u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::Observer,
                facing: Direction::North,
                powered: false,
            }),
            13575u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::Observer,
                facing: Direction::East,
                powered: true,
            }),
            13576u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::Observer,
                facing: Direction::East,
                powered: false,
            }),
            13577u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::Observer,
                facing: Direction::South,
                powered: true,
            }),
            13578u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::Observer,
                facing: Direction::South,
                powered: false,
            }),
            13579u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::Observer,
                facing: Direction::West,
                powered: true,
            }),
            13580u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::Observer,
                facing: Direction::West,
                powered: false,
            }),
            13581u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::Observer,
                facing: Direction::Up,
                powered: true,
            }),
            13582u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::Observer,
                facing: Direction::Up,
                powered: false,
            }),
            13583u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::Observer,
                facing: Direction::Down,
                powered: true,
            }),
            13584u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::Observer,
                facing: Direction::Down,
                powered: false,
            }),
            9908u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::PiglinWallHead,
                facing: Direction::North,
                powered: true,
            }),
            9909u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::PiglinWallHead,
                facing: Direction::North,
                powered: false,
            }),
            9910u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::PiglinWallHead,
                facing: Direction::South,
                powered: true,
            }),
            9911u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::PiglinWallHead,
                facing: Direction::South,
                powered: false,
            }),
            9912u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::PiglinWallHead,
                facing: Direction::West,
                powered: true,
            }),
            9913u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::PiglinWallHead,
                facing: Direction::West,
                powered: false,
            }),
            9914u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::PiglinWallHead,
                facing: Direction::East,
                powered: true,
            }),
            9915u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::PiglinWallHead,
                facing: Direction::East,
                powered: false,
            }),
            9788u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::PlayerWallHead,
                facing: Direction::North,
                powered: true,
            }),
            9789u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::PlayerWallHead,
                facing: Direction::North,
                powered: false,
            }),
            9790u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::PlayerWallHead,
                facing: Direction::South,
                powered: true,
            }),
            9791u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::PlayerWallHead,
                facing: Direction::South,
                powered: false,
            }),
            9792u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::PlayerWallHead,
                facing: Direction::West,
                powered: true,
            }),
            9793u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::PlayerWallHead,
                facing: Direction::West,
                powered: false,
            }),
            9794u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::PlayerWallHead,
                facing: Direction::East,
                powered: true,
            }),
            9795u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::PlayerWallHead,
                facing: Direction::East,
                powered: false,
            }),
            9668u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::SkeletonWallSkull,
                facing: Direction::North,
                powered: true,
            }),
            9669u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::SkeletonWallSkull,
                facing: Direction::North,
                powered: false,
            }),
            9670u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::SkeletonWallSkull,
                facing: Direction::South,
                powered: true,
            }),
            9671u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::SkeletonWallSkull,
                facing: Direction::South,
                powered: false,
            }),
            9672u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::SkeletonWallSkull,
                facing: Direction::West,
                powered: true,
            }),
            9673u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::SkeletonWallSkull,
                facing: Direction::West,
                powered: false,
            }),
            9674u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::SkeletonWallSkull,
                facing: Direction::East,
                powered: true,
            }),
            9675u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::SkeletonWallSkull,
                facing: Direction::East,
                powered: false,
            }),
            9708u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::WitherSkeletonWallSkull,
                facing: Direction::North,
                powered: true,
            }),
            9709u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::WitherSkeletonWallSkull,
                facing: Direction::North,
                powered: false,
            }),
            9710u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::WitherSkeletonWallSkull,
                facing: Direction::South,
                powered: true,
            }),
            9711u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::WitherSkeletonWallSkull,
                facing: Direction::South,
                powered: false,
            }),
            9712u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::WitherSkeletonWallSkull,
                facing: Direction::West,
                powered: true,
            }),
            9713u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::WitherSkeletonWallSkull,
                facing: Direction::West,
                powered: false,
            }),
            9714u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::WitherSkeletonWallSkull,
                facing: Direction::East,
                powered: true,
            }),
            9715u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::WitherSkeletonWallSkull,
                facing: Direction::East,
                powered: false,
            }),
            9748u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::ZombieWallHead,
                facing: Direction::North,
                powered: true,
            }),
            9749u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::ZombieWallHead,
                facing: Direction::North,
                powered: false,
            }),
            9750u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::ZombieWallHead,
                facing: Direction::South,
                powered: true,
            }),
            9751u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::ZombieWallHead,
                facing: Direction::South,
                powered: false,
            }),
            9752u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::ZombieWallHead,
                facing: Direction::West,
                powered: true,
            }),
            9753u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::ZombieWallHead,
                facing: Direction::West,
                powered: false,
            }),
            9754u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::ZombieWallHead,
                facing: Direction::East,
                powered: true,
            }),
            9755u32 => Ok(GeneratedStruct57 {
                block_type: GeneratedStruct57Type::ZombieWallHead,
                facing: Direction::East,
                powered: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct57 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::CreeperWallHead,
                facing: Direction::North,
                powered: true,
            } => Ok(9828u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::CreeperWallHead,
                facing: Direction::North,
                powered: false,
            } => Ok(9829u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::CreeperWallHead,
                facing: Direction::South,
                powered: true,
            } => Ok(9830u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::CreeperWallHead,
                facing: Direction::South,
                powered: false,
            } => Ok(9831u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::CreeperWallHead,
                facing: Direction::West,
                powered: true,
            } => Ok(9832u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::CreeperWallHead,
                facing: Direction::West,
                powered: false,
            } => Ok(9833u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::CreeperWallHead,
                facing: Direction::East,
                powered: true,
            } => Ok(9834u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::CreeperWallHead,
                facing: Direction::East,
                powered: false,
            } => Ok(9835u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::DragonWallHead,
                facing: Direction::North,
                powered: true,
            } => Ok(9868u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::DragonWallHead,
                facing: Direction::North,
                powered: false,
            } => Ok(9869u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::DragonWallHead,
                facing: Direction::South,
                powered: true,
            } => Ok(9870u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::DragonWallHead,
                facing: Direction::South,
                powered: false,
            } => Ok(9871u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::DragonWallHead,
                facing: Direction::West,
                powered: true,
            } => Ok(9872u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::DragonWallHead,
                facing: Direction::West,
                powered: false,
            } => Ok(9873u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::DragonWallHead,
                facing: Direction::East,
                powered: true,
            } => Ok(9874u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::DragonWallHead,
                facing: Direction::East,
                powered: false,
            } => Ok(9875u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::Observer,
                facing: Direction::North,
                powered: true,
            } => Ok(13573u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::Observer,
                facing: Direction::North,
                powered: false,
            } => Ok(13574u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::Observer,
                facing: Direction::East,
                powered: true,
            } => Ok(13575u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::Observer,
                facing: Direction::East,
                powered: false,
            } => Ok(13576u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::Observer,
                facing: Direction::South,
                powered: true,
            } => Ok(13577u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::Observer,
                facing: Direction::South,
                powered: false,
            } => Ok(13578u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::Observer,
                facing: Direction::West,
                powered: true,
            } => Ok(13579u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::Observer,
                facing: Direction::West,
                powered: false,
            } => Ok(13580u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::Observer,
                facing: Direction::Up,
                powered: true,
            } => Ok(13581u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::Observer,
                facing: Direction::Up,
                powered: false,
            } => Ok(13582u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::Observer,
                facing: Direction::Down,
                powered: true,
            } => Ok(13583u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::Observer,
                facing: Direction::Down,
                powered: false,
            } => Ok(13584u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::PiglinWallHead,
                facing: Direction::North,
                powered: true,
            } => Ok(9908u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::PiglinWallHead,
                facing: Direction::North,
                powered: false,
            } => Ok(9909u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::PiglinWallHead,
                facing: Direction::South,
                powered: true,
            } => Ok(9910u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::PiglinWallHead,
                facing: Direction::South,
                powered: false,
            } => Ok(9911u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::PiglinWallHead,
                facing: Direction::West,
                powered: true,
            } => Ok(9912u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::PiglinWallHead,
                facing: Direction::West,
                powered: false,
            } => Ok(9913u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::PiglinWallHead,
                facing: Direction::East,
                powered: true,
            } => Ok(9914u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::PiglinWallHead,
                facing: Direction::East,
                powered: false,
            } => Ok(9915u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::PlayerWallHead,
                facing: Direction::North,
                powered: true,
            } => Ok(9788u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::PlayerWallHead,
                facing: Direction::North,
                powered: false,
            } => Ok(9789u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::PlayerWallHead,
                facing: Direction::South,
                powered: true,
            } => Ok(9790u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::PlayerWallHead,
                facing: Direction::South,
                powered: false,
            } => Ok(9791u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::PlayerWallHead,
                facing: Direction::West,
                powered: true,
            } => Ok(9792u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::PlayerWallHead,
                facing: Direction::West,
                powered: false,
            } => Ok(9793u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::PlayerWallHead,
                facing: Direction::East,
                powered: true,
            } => Ok(9794u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::PlayerWallHead,
                facing: Direction::East,
                powered: false,
            } => Ok(9795u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::SkeletonWallSkull,
                facing: Direction::North,
                powered: true,
            } => Ok(9668u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::SkeletonWallSkull,
                facing: Direction::North,
                powered: false,
            } => Ok(9669u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::SkeletonWallSkull,
                facing: Direction::South,
                powered: true,
            } => Ok(9670u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::SkeletonWallSkull,
                facing: Direction::South,
                powered: false,
            } => Ok(9671u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::SkeletonWallSkull,
                facing: Direction::West,
                powered: true,
            } => Ok(9672u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::SkeletonWallSkull,
                facing: Direction::West,
                powered: false,
            } => Ok(9673u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::SkeletonWallSkull,
                facing: Direction::East,
                powered: true,
            } => Ok(9674u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::SkeletonWallSkull,
                facing: Direction::East,
                powered: false,
            } => Ok(9675u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::WitherSkeletonWallSkull,
                facing: Direction::North,
                powered: true,
            } => Ok(9708u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::WitherSkeletonWallSkull,
                facing: Direction::North,
                powered: false,
            } => Ok(9709u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::WitherSkeletonWallSkull,
                facing: Direction::South,
                powered: true,
            } => Ok(9710u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::WitherSkeletonWallSkull,
                facing: Direction::South,
                powered: false,
            } => Ok(9711u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::WitherSkeletonWallSkull,
                facing: Direction::West,
                powered: true,
            } => Ok(9712u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::WitherSkeletonWallSkull,
                facing: Direction::West,
                powered: false,
            } => Ok(9713u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::WitherSkeletonWallSkull,
                facing: Direction::East,
                powered: true,
            } => Ok(9714u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::WitherSkeletonWallSkull,
                facing: Direction::East,
                powered: false,
            } => Ok(9715u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::ZombieWallHead,
                facing: Direction::North,
                powered: true,
            } => Ok(9748u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::ZombieWallHead,
                facing: Direction::North,
                powered: false,
            } => Ok(9749u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::ZombieWallHead,
                facing: Direction::South,
                powered: true,
            } => Ok(9750u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::ZombieWallHead,
                facing: Direction::South,
                powered: false,
            } => Ok(9751u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::ZombieWallHead,
                facing: Direction::West,
                powered: true,
            } => Ok(9752u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::ZombieWallHead,
                facing: Direction::West,
                powered: false,
            } => Ok(9753u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::ZombieWallHead,
                facing: Direction::East,
                powered: true,
            } => Ok(9754u32),
            GeneratedStruct57 {
                block_type: GeneratedStruct57Type::ZombieWallHead,
                facing: Direction::East,
                powered: false,
            } => Ok(9755u32),
            _ => Err(()),
        }
    }
}

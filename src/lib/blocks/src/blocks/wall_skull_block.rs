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

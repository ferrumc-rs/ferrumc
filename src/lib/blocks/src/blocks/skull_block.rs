#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum SkullBlockType {
    CreeperHead,
    DragonHead,
    PiglinHead,
    PlayerHead,
    SkeletonSkull,
    WitherSkeletonSkull,
    ZombieHead,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SkullBlock {
    pub block_type: SkullBlockType,
    pub powered: bool,
    pub rotation: i32,
}
impl TryInto<u32> for SkullBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: true,
                rotation: 0i32,
            } => Ok(9796u32),
            SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: true,
                rotation: 1i32,
            } => Ok(9797u32),
            SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: true,
                rotation: 2i32,
            } => Ok(9798u32),
            SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: true,
                rotation: 3i32,
            } => Ok(9799u32),
            SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: true,
                rotation: 4i32,
            } => Ok(9800u32),
            SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: true,
                rotation: 5i32,
            } => Ok(9801u32),
            SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: true,
                rotation: 6i32,
            } => Ok(9802u32),
            SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: true,
                rotation: 7i32,
            } => Ok(9803u32),
            SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: true,
                rotation: 8i32,
            } => Ok(9804u32),
            SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: true,
                rotation: 9i32,
            } => Ok(9805u32),
            SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: true,
                rotation: 10i32,
            } => Ok(9806u32),
            SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: true,
                rotation: 11i32,
            } => Ok(9807u32),
            SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: true,
                rotation: 12i32,
            } => Ok(9808u32),
            SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: true,
                rotation: 13i32,
            } => Ok(9809u32),
            SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: true,
                rotation: 14i32,
            } => Ok(9810u32),
            SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: true,
                rotation: 15i32,
            } => Ok(9811u32),
            SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: false,
                rotation: 0i32,
            } => Ok(9812u32),
            SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: false,
                rotation: 1i32,
            } => Ok(9813u32),
            SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: false,
                rotation: 2i32,
            } => Ok(9814u32),
            SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: false,
                rotation: 3i32,
            } => Ok(9815u32),
            SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: false,
                rotation: 4i32,
            } => Ok(9816u32),
            SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: false,
                rotation: 5i32,
            } => Ok(9817u32),
            SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: false,
                rotation: 6i32,
            } => Ok(9818u32),
            SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: false,
                rotation: 7i32,
            } => Ok(9819u32),
            SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: false,
                rotation: 8i32,
            } => Ok(9820u32),
            SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: false,
                rotation: 9i32,
            } => Ok(9821u32),
            SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: false,
                rotation: 10i32,
            } => Ok(9822u32),
            SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: false,
                rotation: 11i32,
            } => Ok(9823u32),
            SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: false,
                rotation: 12i32,
            } => Ok(9824u32),
            SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: false,
                rotation: 13i32,
            } => Ok(9825u32),
            SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: false,
                rotation: 14i32,
            } => Ok(9826u32),
            SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: false,
                rotation: 15i32,
            } => Ok(9827u32),
            SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: true,
                rotation: 0i32,
            } => Ok(9836u32),
            SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: true,
                rotation: 1i32,
            } => Ok(9837u32),
            SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: true,
                rotation: 2i32,
            } => Ok(9838u32),
            SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: true,
                rotation: 3i32,
            } => Ok(9839u32),
            SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: true,
                rotation: 4i32,
            } => Ok(9840u32),
            SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: true,
                rotation: 5i32,
            } => Ok(9841u32),
            SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: true,
                rotation: 6i32,
            } => Ok(9842u32),
            SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: true,
                rotation: 7i32,
            } => Ok(9843u32),
            SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: true,
                rotation: 8i32,
            } => Ok(9844u32),
            SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: true,
                rotation: 9i32,
            } => Ok(9845u32),
            SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: true,
                rotation: 10i32,
            } => Ok(9846u32),
            SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: true,
                rotation: 11i32,
            } => Ok(9847u32),
            SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: true,
                rotation: 12i32,
            } => Ok(9848u32),
            SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: true,
                rotation: 13i32,
            } => Ok(9849u32),
            SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: true,
                rotation: 14i32,
            } => Ok(9850u32),
            SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: true,
                rotation: 15i32,
            } => Ok(9851u32),
            SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: false,
                rotation: 0i32,
            } => Ok(9852u32),
            SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: false,
                rotation: 1i32,
            } => Ok(9853u32),
            SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: false,
                rotation: 2i32,
            } => Ok(9854u32),
            SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: false,
                rotation: 3i32,
            } => Ok(9855u32),
            SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: false,
                rotation: 4i32,
            } => Ok(9856u32),
            SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: false,
                rotation: 5i32,
            } => Ok(9857u32),
            SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: false,
                rotation: 6i32,
            } => Ok(9858u32),
            SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: false,
                rotation: 7i32,
            } => Ok(9859u32),
            SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: false,
                rotation: 8i32,
            } => Ok(9860u32),
            SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: false,
                rotation: 9i32,
            } => Ok(9861u32),
            SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: false,
                rotation: 10i32,
            } => Ok(9862u32),
            SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: false,
                rotation: 11i32,
            } => Ok(9863u32),
            SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: false,
                rotation: 12i32,
            } => Ok(9864u32),
            SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: false,
                rotation: 13i32,
            } => Ok(9865u32),
            SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: false,
                rotation: 14i32,
            } => Ok(9866u32),
            SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: false,
                rotation: 15i32,
            } => Ok(9867u32),
            SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: true,
                rotation: 0i32,
            } => Ok(9876u32),
            SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: true,
                rotation: 1i32,
            } => Ok(9877u32),
            SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: true,
                rotation: 2i32,
            } => Ok(9878u32),
            SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: true,
                rotation: 3i32,
            } => Ok(9879u32),
            SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: true,
                rotation: 4i32,
            } => Ok(9880u32),
            SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: true,
                rotation: 5i32,
            } => Ok(9881u32),
            SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: true,
                rotation: 6i32,
            } => Ok(9882u32),
            SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: true,
                rotation: 7i32,
            } => Ok(9883u32),
            SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: true,
                rotation: 8i32,
            } => Ok(9884u32),
            SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: true,
                rotation: 9i32,
            } => Ok(9885u32),
            SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: true,
                rotation: 10i32,
            } => Ok(9886u32),
            SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: true,
                rotation: 11i32,
            } => Ok(9887u32),
            SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: true,
                rotation: 12i32,
            } => Ok(9888u32),
            SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: true,
                rotation: 13i32,
            } => Ok(9889u32),
            SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: true,
                rotation: 14i32,
            } => Ok(9890u32),
            SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: true,
                rotation: 15i32,
            } => Ok(9891u32),
            SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: false,
                rotation: 0i32,
            } => Ok(9892u32),
            SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: false,
                rotation: 1i32,
            } => Ok(9893u32),
            SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: false,
                rotation: 2i32,
            } => Ok(9894u32),
            SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: false,
                rotation: 3i32,
            } => Ok(9895u32),
            SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: false,
                rotation: 4i32,
            } => Ok(9896u32),
            SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: false,
                rotation: 5i32,
            } => Ok(9897u32),
            SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: false,
                rotation: 6i32,
            } => Ok(9898u32),
            SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: false,
                rotation: 7i32,
            } => Ok(9899u32),
            SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: false,
                rotation: 8i32,
            } => Ok(9900u32),
            SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: false,
                rotation: 9i32,
            } => Ok(9901u32),
            SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: false,
                rotation: 10i32,
            } => Ok(9902u32),
            SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: false,
                rotation: 11i32,
            } => Ok(9903u32),
            SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: false,
                rotation: 12i32,
            } => Ok(9904u32),
            SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: false,
                rotation: 13i32,
            } => Ok(9905u32),
            SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: false,
                rotation: 14i32,
            } => Ok(9906u32),
            SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: false,
                rotation: 15i32,
            } => Ok(9907u32),
            SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: true,
                rotation: 0i32,
            } => Ok(9756u32),
            SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: true,
                rotation: 1i32,
            } => Ok(9757u32),
            SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: true,
                rotation: 2i32,
            } => Ok(9758u32),
            SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: true,
                rotation: 3i32,
            } => Ok(9759u32),
            SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: true,
                rotation: 4i32,
            } => Ok(9760u32),
            SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: true,
                rotation: 5i32,
            } => Ok(9761u32),
            SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: true,
                rotation: 6i32,
            } => Ok(9762u32),
            SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: true,
                rotation: 7i32,
            } => Ok(9763u32),
            SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: true,
                rotation: 8i32,
            } => Ok(9764u32),
            SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: true,
                rotation: 9i32,
            } => Ok(9765u32),
            SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: true,
                rotation: 10i32,
            } => Ok(9766u32),
            SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: true,
                rotation: 11i32,
            } => Ok(9767u32),
            SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: true,
                rotation: 12i32,
            } => Ok(9768u32),
            SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: true,
                rotation: 13i32,
            } => Ok(9769u32),
            SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: true,
                rotation: 14i32,
            } => Ok(9770u32),
            SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: true,
                rotation: 15i32,
            } => Ok(9771u32),
            SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: false,
                rotation: 0i32,
            } => Ok(9772u32),
            SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: false,
                rotation: 1i32,
            } => Ok(9773u32),
            SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: false,
                rotation: 2i32,
            } => Ok(9774u32),
            SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: false,
                rotation: 3i32,
            } => Ok(9775u32),
            SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: false,
                rotation: 4i32,
            } => Ok(9776u32),
            SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: false,
                rotation: 5i32,
            } => Ok(9777u32),
            SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: false,
                rotation: 6i32,
            } => Ok(9778u32),
            SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: false,
                rotation: 7i32,
            } => Ok(9779u32),
            SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: false,
                rotation: 8i32,
            } => Ok(9780u32),
            SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: false,
                rotation: 9i32,
            } => Ok(9781u32),
            SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: false,
                rotation: 10i32,
            } => Ok(9782u32),
            SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: false,
                rotation: 11i32,
            } => Ok(9783u32),
            SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: false,
                rotation: 12i32,
            } => Ok(9784u32),
            SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: false,
                rotation: 13i32,
            } => Ok(9785u32),
            SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: false,
                rotation: 14i32,
            } => Ok(9786u32),
            SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: false,
                rotation: 15i32,
            } => Ok(9787u32),
            SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: true,
                rotation: 0i32,
            } => Ok(9636u32),
            SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: true,
                rotation: 1i32,
            } => Ok(9637u32),
            SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: true,
                rotation: 2i32,
            } => Ok(9638u32),
            SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: true,
                rotation: 3i32,
            } => Ok(9639u32),
            SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: true,
                rotation: 4i32,
            } => Ok(9640u32),
            SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: true,
                rotation: 5i32,
            } => Ok(9641u32),
            SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: true,
                rotation: 6i32,
            } => Ok(9642u32),
            SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: true,
                rotation: 7i32,
            } => Ok(9643u32),
            SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: true,
                rotation: 8i32,
            } => Ok(9644u32),
            SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: true,
                rotation: 9i32,
            } => Ok(9645u32),
            SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: true,
                rotation: 10i32,
            } => Ok(9646u32),
            SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: true,
                rotation: 11i32,
            } => Ok(9647u32),
            SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: true,
                rotation: 12i32,
            } => Ok(9648u32),
            SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: true,
                rotation: 13i32,
            } => Ok(9649u32),
            SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: true,
                rotation: 14i32,
            } => Ok(9650u32),
            SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: true,
                rotation: 15i32,
            } => Ok(9651u32),
            SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: false,
                rotation: 0i32,
            } => Ok(9652u32),
            SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: false,
                rotation: 1i32,
            } => Ok(9653u32),
            SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: false,
                rotation: 2i32,
            } => Ok(9654u32),
            SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: false,
                rotation: 3i32,
            } => Ok(9655u32),
            SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: false,
                rotation: 4i32,
            } => Ok(9656u32),
            SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: false,
                rotation: 5i32,
            } => Ok(9657u32),
            SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: false,
                rotation: 6i32,
            } => Ok(9658u32),
            SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: false,
                rotation: 7i32,
            } => Ok(9659u32),
            SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: false,
                rotation: 8i32,
            } => Ok(9660u32),
            SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: false,
                rotation: 9i32,
            } => Ok(9661u32),
            SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: false,
                rotation: 10i32,
            } => Ok(9662u32),
            SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: false,
                rotation: 11i32,
            } => Ok(9663u32),
            SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: false,
                rotation: 12i32,
            } => Ok(9664u32),
            SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: false,
                rotation: 13i32,
            } => Ok(9665u32),
            SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: false,
                rotation: 14i32,
            } => Ok(9666u32),
            SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: false,
                rotation: 15i32,
            } => Ok(9667u32),
            SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: true,
                rotation: 0i32,
            } => Ok(9676u32),
            SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: true,
                rotation: 1i32,
            } => Ok(9677u32),
            SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: true,
                rotation: 2i32,
            } => Ok(9678u32),
            SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: true,
                rotation: 3i32,
            } => Ok(9679u32),
            SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: true,
                rotation: 4i32,
            } => Ok(9680u32),
            SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: true,
                rotation: 5i32,
            } => Ok(9681u32),
            SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: true,
                rotation: 6i32,
            } => Ok(9682u32),
            SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: true,
                rotation: 7i32,
            } => Ok(9683u32),
            SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: true,
                rotation: 8i32,
            } => Ok(9684u32),
            SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: true,
                rotation: 9i32,
            } => Ok(9685u32),
            SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: true,
                rotation: 10i32,
            } => Ok(9686u32),
            SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: true,
                rotation: 11i32,
            } => Ok(9687u32),
            SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: true,
                rotation: 12i32,
            } => Ok(9688u32),
            SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: true,
                rotation: 13i32,
            } => Ok(9689u32),
            SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: true,
                rotation: 14i32,
            } => Ok(9690u32),
            SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: true,
                rotation: 15i32,
            } => Ok(9691u32),
            SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: false,
                rotation: 0i32,
            } => Ok(9692u32),
            SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: false,
                rotation: 1i32,
            } => Ok(9693u32),
            SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: false,
                rotation: 2i32,
            } => Ok(9694u32),
            SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: false,
                rotation: 3i32,
            } => Ok(9695u32),
            SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: false,
                rotation: 4i32,
            } => Ok(9696u32),
            SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: false,
                rotation: 5i32,
            } => Ok(9697u32),
            SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: false,
                rotation: 6i32,
            } => Ok(9698u32),
            SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: false,
                rotation: 7i32,
            } => Ok(9699u32),
            SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: false,
                rotation: 8i32,
            } => Ok(9700u32),
            SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: false,
                rotation: 9i32,
            } => Ok(9701u32),
            SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: false,
                rotation: 10i32,
            } => Ok(9702u32),
            SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: false,
                rotation: 11i32,
            } => Ok(9703u32),
            SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: false,
                rotation: 12i32,
            } => Ok(9704u32),
            SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: false,
                rotation: 13i32,
            } => Ok(9705u32),
            SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: false,
                rotation: 14i32,
            } => Ok(9706u32),
            SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: false,
                rotation: 15i32,
            } => Ok(9707u32),
            SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: true,
                rotation: 0i32,
            } => Ok(9716u32),
            SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: true,
                rotation: 1i32,
            } => Ok(9717u32),
            SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: true,
                rotation: 2i32,
            } => Ok(9718u32),
            SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: true,
                rotation: 3i32,
            } => Ok(9719u32),
            SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: true,
                rotation: 4i32,
            } => Ok(9720u32),
            SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: true,
                rotation: 5i32,
            } => Ok(9721u32),
            SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: true,
                rotation: 6i32,
            } => Ok(9722u32),
            SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: true,
                rotation: 7i32,
            } => Ok(9723u32),
            SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: true,
                rotation: 8i32,
            } => Ok(9724u32),
            SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: true,
                rotation: 9i32,
            } => Ok(9725u32),
            SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: true,
                rotation: 10i32,
            } => Ok(9726u32),
            SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: true,
                rotation: 11i32,
            } => Ok(9727u32),
            SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: true,
                rotation: 12i32,
            } => Ok(9728u32),
            SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: true,
                rotation: 13i32,
            } => Ok(9729u32),
            SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: true,
                rotation: 14i32,
            } => Ok(9730u32),
            SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: true,
                rotation: 15i32,
            } => Ok(9731u32),
            SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: false,
                rotation: 0i32,
            } => Ok(9732u32),
            SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: false,
                rotation: 1i32,
            } => Ok(9733u32),
            SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: false,
                rotation: 2i32,
            } => Ok(9734u32),
            SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: false,
                rotation: 3i32,
            } => Ok(9735u32),
            SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: false,
                rotation: 4i32,
            } => Ok(9736u32),
            SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: false,
                rotation: 5i32,
            } => Ok(9737u32),
            SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: false,
                rotation: 6i32,
            } => Ok(9738u32),
            SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: false,
                rotation: 7i32,
            } => Ok(9739u32),
            SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: false,
                rotation: 8i32,
            } => Ok(9740u32),
            SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: false,
                rotation: 9i32,
            } => Ok(9741u32),
            SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: false,
                rotation: 10i32,
            } => Ok(9742u32),
            SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: false,
                rotation: 11i32,
            } => Ok(9743u32),
            SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: false,
                rotation: 12i32,
            } => Ok(9744u32),
            SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: false,
                rotation: 13i32,
            } => Ok(9745u32),
            SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: false,
                rotation: 14i32,
            } => Ok(9746u32),
            SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: false,
                rotation: 15i32,
            } => Ok(9747u32),
            _ => Err(()),
        }
    }
}

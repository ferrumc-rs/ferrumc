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
impl SkullBlock {
    pub(crate) const VTABLE: crate::BlockBehaviorTable =
        crate::BlockBehaviorTable::from::<SkullBlock>();
}
impl TryFrom<u32> for SkullBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            9796u32 => Ok(SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: true,
                rotation: 0i32,
            }),
            9797u32 => Ok(SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: true,
                rotation: 1i32,
            }),
            9798u32 => Ok(SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: true,
                rotation: 2i32,
            }),
            9799u32 => Ok(SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: true,
                rotation: 3i32,
            }),
            9800u32 => Ok(SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: true,
                rotation: 4i32,
            }),
            9801u32 => Ok(SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: true,
                rotation: 5i32,
            }),
            9802u32 => Ok(SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: true,
                rotation: 6i32,
            }),
            9803u32 => Ok(SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: true,
                rotation: 7i32,
            }),
            9804u32 => Ok(SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: true,
                rotation: 8i32,
            }),
            9805u32 => Ok(SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: true,
                rotation: 9i32,
            }),
            9806u32 => Ok(SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: true,
                rotation: 10i32,
            }),
            9807u32 => Ok(SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: true,
                rotation: 11i32,
            }),
            9808u32 => Ok(SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: true,
                rotation: 12i32,
            }),
            9809u32 => Ok(SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: true,
                rotation: 13i32,
            }),
            9810u32 => Ok(SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: true,
                rotation: 14i32,
            }),
            9811u32 => Ok(SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: true,
                rotation: 15i32,
            }),
            9812u32 => Ok(SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: false,
                rotation: 0i32,
            }),
            9813u32 => Ok(SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: false,
                rotation: 1i32,
            }),
            9814u32 => Ok(SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: false,
                rotation: 2i32,
            }),
            9815u32 => Ok(SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: false,
                rotation: 3i32,
            }),
            9816u32 => Ok(SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: false,
                rotation: 4i32,
            }),
            9817u32 => Ok(SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: false,
                rotation: 5i32,
            }),
            9818u32 => Ok(SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: false,
                rotation: 6i32,
            }),
            9819u32 => Ok(SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: false,
                rotation: 7i32,
            }),
            9820u32 => Ok(SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: false,
                rotation: 8i32,
            }),
            9821u32 => Ok(SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: false,
                rotation: 9i32,
            }),
            9822u32 => Ok(SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: false,
                rotation: 10i32,
            }),
            9823u32 => Ok(SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: false,
                rotation: 11i32,
            }),
            9824u32 => Ok(SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: false,
                rotation: 12i32,
            }),
            9825u32 => Ok(SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: false,
                rotation: 13i32,
            }),
            9826u32 => Ok(SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: false,
                rotation: 14i32,
            }),
            9827u32 => Ok(SkullBlock {
                block_type: SkullBlockType::CreeperHead,
                powered: false,
                rotation: 15i32,
            }),
            9836u32 => Ok(SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: true,
                rotation: 0i32,
            }),
            9837u32 => Ok(SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: true,
                rotation: 1i32,
            }),
            9838u32 => Ok(SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: true,
                rotation: 2i32,
            }),
            9839u32 => Ok(SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: true,
                rotation: 3i32,
            }),
            9840u32 => Ok(SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: true,
                rotation: 4i32,
            }),
            9841u32 => Ok(SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: true,
                rotation: 5i32,
            }),
            9842u32 => Ok(SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: true,
                rotation: 6i32,
            }),
            9843u32 => Ok(SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: true,
                rotation: 7i32,
            }),
            9844u32 => Ok(SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: true,
                rotation: 8i32,
            }),
            9845u32 => Ok(SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: true,
                rotation: 9i32,
            }),
            9846u32 => Ok(SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: true,
                rotation: 10i32,
            }),
            9847u32 => Ok(SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: true,
                rotation: 11i32,
            }),
            9848u32 => Ok(SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: true,
                rotation: 12i32,
            }),
            9849u32 => Ok(SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: true,
                rotation: 13i32,
            }),
            9850u32 => Ok(SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: true,
                rotation: 14i32,
            }),
            9851u32 => Ok(SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: true,
                rotation: 15i32,
            }),
            9852u32 => Ok(SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: false,
                rotation: 0i32,
            }),
            9853u32 => Ok(SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: false,
                rotation: 1i32,
            }),
            9854u32 => Ok(SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: false,
                rotation: 2i32,
            }),
            9855u32 => Ok(SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: false,
                rotation: 3i32,
            }),
            9856u32 => Ok(SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: false,
                rotation: 4i32,
            }),
            9857u32 => Ok(SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: false,
                rotation: 5i32,
            }),
            9858u32 => Ok(SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: false,
                rotation: 6i32,
            }),
            9859u32 => Ok(SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: false,
                rotation: 7i32,
            }),
            9860u32 => Ok(SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: false,
                rotation: 8i32,
            }),
            9861u32 => Ok(SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: false,
                rotation: 9i32,
            }),
            9862u32 => Ok(SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: false,
                rotation: 10i32,
            }),
            9863u32 => Ok(SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: false,
                rotation: 11i32,
            }),
            9864u32 => Ok(SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: false,
                rotation: 12i32,
            }),
            9865u32 => Ok(SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: false,
                rotation: 13i32,
            }),
            9866u32 => Ok(SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: false,
                rotation: 14i32,
            }),
            9867u32 => Ok(SkullBlock {
                block_type: SkullBlockType::DragonHead,
                powered: false,
                rotation: 15i32,
            }),
            9876u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: true,
                rotation: 0i32,
            }),
            9877u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: true,
                rotation: 1i32,
            }),
            9878u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: true,
                rotation: 2i32,
            }),
            9879u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: true,
                rotation: 3i32,
            }),
            9880u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: true,
                rotation: 4i32,
            }),
            9881u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: true,
                rotation: 5i32,
            }),
            9882u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: true,
                rotation: 6i32,
            }),
            9883u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: true,
                rotation: 7i32,
            }),
            9884u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: true,
                rotation: 8i32,
            }),
            9885u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: true,
                rotation: 9i32,
            }),
            9886u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: true,
                rotation: 10i32,
            }),
            9887u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: true,
                rotation: 11i32,
            }),
            9888u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: true,
                rotation: 12i32,
            }),
            9889u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: true,
                rotation: 13i32,
            }),
            9890u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: true,
                rotation: 14i32,
            }),
            9891u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: true,
                rotation: 15i32,
            }),
            9892u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: false,
                rotation: 0i32,
            }),
            9893u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: false,
                rotation: 1i32,
            }),
            9894u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: false,
                rotation: 2i32,
            }),
            9895u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: false,
                rotation: 3i32,
            }),
            9896u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: false,
                rotation: 4i32,
            }),
            9897u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: false,
                rotation: 5i32,
            }),
            9898u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: false,
                rotation: 6i32,
            }),
            9899u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: false,
                rotation: 7i32,
            }),
            9900u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: false,
                rotation: 8i32,
            }),
            9901u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: false,
                rotation: 9i32,
            }),
            9902u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: false,
                rotation: 10i32,
            }),
            9903u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: false,
                rotation: 11i32,
            }),
            9904u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: false,
                rotation: 12i32,
            }),
            9905u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: false,
                rotation: 13i32,
            }),
            9906u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: false,
                rotation: 14i32,
            }),
            9907u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PiglinHead,
                powered: false,
                rotation: 15i32,
            }),
            9756u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: true,
                rotation: 0i32,
            }),
            9757u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: true,
                rotation: 1i32,
            }),
            9758u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: true,
                rotation: 2i32,
            }),
            9759u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: true,
                rotation: 3i32,
            }),
            9760u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: true,
                rotation: 4i32,
            }),
            9761u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: true,
                rotation: 5i32,
            }),
            9762u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: true,
                rotation: 6i32,
            }),
            9763u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: true,
                rotation: 7i32,
            }),
            9764u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: true,
                rotation: 8i32,
            }),
            9765u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: true,
                rotation: 9i32,
            }),
            9766u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: true,
                rotation: 10i32,
            }),
            9767u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: true,
                rotation: 11i32,
            }),
            9768u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: true,
                rotation: 12i32,
            }),
            9769u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: true,
                rotation: 13i32,
            }),
            9770u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: true,
                rotation: 14i32,
            }),
            9771u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: true,
                rotation: 15i32,
            }),
            9772u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: false,
                rotation: 0i32,
            }),
            9773u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: false,
                rotation: 1i32,
            }),
            9774u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: false,
                rotation: 2i32,
            }),
            9775u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: false,
                rotation: 3i32,
            }),
            9776u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: false,
                rotation: 4i32,
            }),
            9777u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: false,
                rotation: 5i32,
            }),
            9778u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: false,
                rotation: 6i32,
            }),
            9779u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: false,
                rotation: 7i32,
            }),
            9780u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: false,
                rotation: 8i32,
            }),
            9781u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: false,
                rotation: 9i32,
            }),
            9782u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: false,
                rotation: 10i32,
            }),
            9783u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: false,
                rotation: 11i32,
            }),
            9784u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: false,
                rotation: 12i32,
            }),
            9785u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: false,
                rotation: 13i32,
            }),
            9786u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: false,
                rotation: 14i32,
            }),
            9787u32 => Ok(SkullBlock {
                block_type: SkullBlockType::PlayerHead,
                powered: false,
                rotation: 15i32,
            }),
            9636u32 => Ok(SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: true,
                rotation: 0i32,
            }),
            9637u32 => Ok(SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: true,
                rotation: 1i32,
            }),
            9638u32 => Ok(SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: true,
                rotation: 2i32,
            }),
            9639u32 => Ok(SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: true,
                rotation: 3i32,
            }),
            9640u32 => Ok(SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: true,
                rotation: 4i32,
            }),
            9641u32 => Ok(SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: true,
                rotation: 5i32,
            }),
            9642u32 => Ok(SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: true,
                rotation: 6i32,
            }),
            9643u32 => Ok(SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: true,
                rotation: 7i32,
            }),
            9644u32 => Ok(SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: true,
                rotation: 8i32,
            }),
            9645u32 => Ok(SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: true,
                rotation: 9i32,
            }),
            9646u32 => Ok(SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: true,
                rotation: 10i32,
            }),
            9647u32 => Ok(SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: true,
                rotation: 11i32,
            }),
            9648u32 => Ok(SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: true,
                rotation: 12i32,
            }),
            9649u32 => Ok(SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: true,
                rotation: 13i32,
            }),
            9650u32 => Ok(SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: true,
                rotation: 14i32,
            }),
            9651u32 => Ok(SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: true,
                rotation: 15i32,
            }),
            9652u32 => Ok(SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: false,
                rotation: 0i32,
            }),
            9653u32 => Ok(SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: false,
                rotation: 1i32,
            }),
            9654u32 => Ok(SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: false,
                rotation: 2i32,
            }),
            9655u32 => Ok(SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: false,
                rotation: 3i32,
            }),
            9656u32 => Ok(SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: false,
                rotation: 4i32,
            }),
            9657u32 => Ok(SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: false,
                rotation: 5i32,
            }),
            9658u32 => Ok(SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: false,
                rotation: 6i32,
            }),
            9659u32 => Ok(SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: false,
                rotation: 7i32,
            }),
            9660u32 => Ok(SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: false,
                rotation: 8i32,
            }),
            9661u32 => Ok(SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: false,
                rotation: 9i32,
            }),
            9662u32 => Ok(SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: false,
                rotation: 10i32,
            }),
            9663u32 => Ok(SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: false,
                rotation: 11i32,
            }),
            9664u32 => Ok(SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: false,
                rotation: 12i32,
            }),
            9665u32 => Ok(SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: false,
                rotation: 13i32,
            }),
            9666u32 => Ok(SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: false,
                rotation: 14i32,
            }),
            9667u32 => Ok(SkullBlock {
                block_type: SkullBlockType::SkeletonSkull,
                powered: false,
                rotation: 15i32,
            }),
            9676u32 => Ok(SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: true,
                rotation: 0i32,
            }),
            9677u32 => Ok(SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: true,
                rotation: 1i32,
            }),
            9678u32 => Ok(SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: true,
                rotation: 2i32,
            }),
            9679u32 => Ok(SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: true,
                rotation: 3i32,
            }),
            9680u32 => Ok(SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: true,
                rotation: 4i32,
            }),
            9681u32 => Ok(SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: true,
                rotation: 5i32,
            }),
            9682u32 => Ok(SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: true,
                rotation: 6i32,
            }),
            9683u32 => Ok(SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: true,
                rotation: 7i32,
            }),
            9684u32 => Ok(SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: true,
                rotation: 8i32,
            }),
            9685u32 => Ok(SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: true,
                rotation: 9i32,
            }),
            9686u32 => Ok(SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: true,
                rotation: 10i32,
            }),
            9687u32 => Ok(SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: true,
                rotation: 11i32,
            }),
            9688u32 => Ok(SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: true,
                rotation: 12i32,
            }),
            9689u32 => Ok(SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: true,
                rotation: 13i32,
            }),
            9690u32 => Ok(SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: true,
                rotation: 14i32,
            }),
            9691u32 => Ok(SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: true,
                rotation: 15i32,
            }),
            9692u32 => Ok(SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: false,
                rotation: 0i32,
            }),
            9693u32 => Ok(SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: false,
                rotation: 1i32,
            }),
            9694u32 => Ok(SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: false,
                rotation: 2i32,
            }),
            9695u32 => Ok(SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: false,
                rotation: 3i32,
            }),
            9696u32 => Ok(SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: false,
                rotation: 4i32,
            }),
            9697u32 => Ok(SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: false,
                rotation: 5i32,
            }),
            9698u32 => Ok(SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: false,
                rotation: 6i32,
            }),
            9699u32 => Ok(SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: false,
                rotation: 7i32,
            }),
            9700u32 => Ok(SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: false,
                rotation: 8i32,
            }),
            9701u32 => Ok(SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: false,
                rotation: 9i32,
            }),
            9702u32 => Ok(SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: false,
                rotation: 10i32,
            }),
            9703u32 => Ok(SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: false,
                rotation: 11i32,
            }),
            9704u32 => Ok(SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: false,
                rotation: 12i32,
            }),
            9705u32 => Ok(SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: false,
                rotation: 13i32,
            }),
            9706u32 => Ok(SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: false,
                rotation: 14i32,
            }),
            9707u32 => Ok(SkullBlock {
                block_type: SkullBlockType::WitherSkeletonSkull,
                powered: false,
                rotation: 15i32,
            }),
            9716u32 => Ok(SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: true,
                rotation: 0i32,
            }),
            9717u32 => Ok(SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: true,
                rotation: 1i32,
            }),
            9718u32 => Ok(SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: true,
                rotation: 2i32,
            }),
            9719u32 => Ok(SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: true,
                rotation: 3i32,
            }),
            9720u32 => Ok(SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: true,
                rotation: 4i32,
            }),
            9721u32 => Ok(SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: true,
                rotation: 5i32,
            }),
            9722u32 => Ok(SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: true,
                rotation: 6i32,
            }),
            9723u32 => Ok(SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: true,
                rotation: 7i32,
            }),
            9724u32 => Ok(SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: true,
                rotation: 8i32,
            }),
            9725u32 => Ok(SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: true,
                rotation: 9i32,
            }),
            9726u32 => Ok(SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: true,
                rotation: 10i32,
            }),
            9727u32 => Ok(SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: true,
                rotation: 11i32,
            }),
            9728u32 => Ok(SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: true,
                rotation: 12i32,
            }),
            9729u32 => Ok(SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: true,
                rotation: 13i32,
            }),
            9730u32 => Ok(SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: true,
                rotation: 14i32,
            }),
            9731u32 => Ok(SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: true,
                rotation: 15i32,
            }),
            9732u32 => Ok(SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: false,
                rotation: 0i32,
            }),
            9733u32 => Ok(SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: false,
                rotation: 1i32,
            }),
            9734u32 => Ok(SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: false,
                rotation: 2i32,
            }),
            9735u32 => Ok(SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: false,
                rotation: 3i32,
            }),
            9736u32 => Ok(SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: false,
                rotation: 4i32,
            }),
            9737u32 => Ok(SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: false,
                rotation: 5i32,
            }),
            9738u32 => Ok(SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: false,
                rotation: 6i32,
            }),
            9739u32 => Ok(SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: false,
                rotation: 7i32,
            }),
            9740u32 => Ok(SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: false,
                rotation: 8i32,
            }),
            9741u32 => Ok(SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: false,
                rotation: 9i32,
            }),
            9742u32 => Ok(SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: false,
                rotation: 10i32,
            }),
            9743u32 => Ok(SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: false,
                rotation: 11i32,
            }),
            9744u32 => Ok(SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: false,
                rotation: 12i32,
            }),
            9745u32 => Ok(SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: false,
                rotation: 13i32,
            }),
            9746u32 => Ok(SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: false,
                rotation: 14i32,
            }),
            9747u32 => Ok(SkullBlock {
                block_type: SkullBlockType::ZombieHead,
                powered: false,
                rotation: 15i32,
            }),
            _ => Err(()),
        }
    }
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

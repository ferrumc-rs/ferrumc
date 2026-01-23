#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum CropBlockType {
    Beetroots,
    Cactus,
    Carrots,
    ChorusFlower,
    FrostedIce,
    Kelp,
    MelonStem,
    NetherWart,
    Potatoes,
    PumpkinStem,
    SugarCane,
    SweetBerryBush,
    TorchflowerCrop,
    TwistingVines,
    WeepingVines,
    Wheat,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CropBlock {
    pub block_type: CropBlockType,
    pub age: i32,
}
impl TryInto<u32> for CropBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            CropBlock {
                block_type: CropBlockType::Beetroots,
                age: 0i32,
            } => Ok(13532u32),
            CropBlock {
                block_type: CropBlockType::Beetroots,
                age: 1i32,
            } => Ok(13533u32),
            CropBlock {
                block_type: CropBlockType::Beetroots,
                age: 2i32,
            } => Ok(13534u32),
            CropBlock {
                block_type: CropBlockType::Beetroots,
                age: 3i32,
            } => Ok(13535u32),
            CropBlock {
                block_type: CropBlockType::Cactus,
                age: 0i32,
            } => Ok(5960u32),
            CropBlock {
                block_type: CropBlockType::Cactus,
                age: 1i32,
            } => Ok(5961u32),
            CropBlock {
                block_type: CropBlockType::Cactus,
                age: 2i32,
            } => Ok(5962u32),
            CropBlock {
                block_type: CropBlockType::Cactus,
                age: 3i32,
            } => Ok(5963u32),
            CropBlock {
                block_type: CropBlockType::Cactus,
                age: 4i32,
            } => Ok(5964u32),
            CropBlock {
                block_type: CropBlockType::Cactus,
                age: 5i32,
            } => Ok(5965u32),
            CropBlock {
                block_type: CropBlockType::Cactus,
                age: 6i32,
            } => Ok(5966u32),
            CropBlock {
                block_type: CropBlockType::Cactus,
                age: 7i32,
            } => Ok(5967u32),
            CropBlock {
                block_type: CropBlockType::Cactus,
                age: 8i32,
            } => Ok(5968u32),
            CropBlock {
                block_type: CropBlockType::Cactus,
                age: 9i32,
            } => Ok(5969u32),
            CropBlock {
                block_type: CropBlockType::Cactus,
                age: 10i32,
            } => Ok(5970u32),
            CropBlock {
                block_type: CropBlockType::Cactus,
                age: 11i32,
            } => Ok(5971u32),
            CropBlock {
                block_type: CropBlockType::Cactus,
                age: 12i32,
            } => Ok(5972u32),
            CropBlock {
                block_type: CropBlockType::Cactus,
                age: 13i32,
            } => Ok(5973u32),
            CropBlock {
                block_type: CropBlockType::Cactus,
                age: 14i32,
            } => Ok(5974u32),
            CropBlock {
                block_type: CropBlockType::Cactus,
                age: 15i32,
            } => Ok(5975u32),
            CropBlock {
                block_type: CropBlockType::Carrots,
                age: 0i32,
            } => Ok(9380u32),
            CropBlock {
                block_type: CropBlockType::Carrots,
                age: 1i32,
            } => Ok(9381u32),
            CropBlock {
                block_type: CropBlockType::Carrots,
                age: 2i32,
            } => Ok(9382u32),
            CropBlock {
                block_type: CropBlockType::Carrots,
                age: 3i32,
            } => Ok(9383u32),
            CropBlock {
                block_type: CropBlockType::Carrots,
                age: 4i32,
            } => Ok(9384u32),
            CropBlock {
                block_type: CropBlockType::Carrots,
                age: 5i32,
            } => Ok(9385u32),
            CropBlock {
                block_type: CropBlockType::Carrots,
                age: 6i32,
            } => Ok(9386u32),
            CropBlock {
                block_type: CropBlockType::Carrots,
                age: 7i32,
            } => Ok(9387u32),
            CropBlock {
                block_type: CropBlockType::ChorusFlower,
                age: 0i32,
            } => Ok(13427u32),
            CropBlock {
                block_type: CropBlockType::ChorusFlower,
                age: 1i32,
            } => Ok(13428u32),
            CropBlock {
                block_type: CropBlockType::ChorusFlower,
                age: 2i32,
            } => Ok(13429u32),
            CropBlock {
                block_type: CropBlockType::ChorusFlower,
                age: 3i32,
            } => Ok(13430u32),
            CropBlock {
                block_type: CropBlockType::ChorusFlower,
                age: 4i32,
            } => Ok(13431u32),
            CropBlock {
                block_type: CropBlockType::ChorusFlower,
                age: 5i32,
            } => Ok(13432u32),
            CropBlock {
                block_type: CropBlockType::FrostedIce,
                age: 0i32,
            } => Ok(13562u32),
            CropBlock {
                block_type: CropBlockType::FrostedIce,
                age: 1i32,
            } => Ok(13563u32),
            CropBlock {
                block_type: CropBlockType::FrostedIce,
                age: 2i32,
            } => Ok(13564u32),
            CropBlock {
                block_type: CropBlockType::FrostedIce,
                age: 3i32,
            } => Ok(13565u32),
            CropBlock {
                block_type: CropBlockType::Kelp,
                age: 0i32,
            } => Ok(13783u32),
            CropBlock {
                block_type: CropBlockType::Kelp,
                age: 1i32,
            } => Ok(13784u32),
            CropBlock {
                block_type: CropBlockType::Kelp,
                age: 2i32,
            } => Ok(13785u32),
            CropBlock {
                block_type: CropBlockType::Kelp,
                age: 3i32,
            } => Ok(13786u32),
            CropBlock {
                block_type: CropBlockType::Kelp,
                age: 4i32,
            } => Ok(13787u32),
            CropBlock {
                block_type: CropBlockType::Kelp,
                age: 5i32,
            } => Ok(13788u32),
            CropBlock {
                block_type: CropBlockType::Kelp,
                age: 6i32,
            } => Ok(13789u32),
            CropBlock {
                block_type: CropBlockType::Kelp,
                age: 7i32,
            } => Ok(13790u32),
            CropBlock {
                block_type: CropBlockType::Kelp,
                age: 8i32,
            } => Ok(13791u32),
            CropBlock {
                block_type: CropBlockType::Kelp,
                age: 9i32,
            } => Ok(13792u32),
            CropBlock {
                block_type: CropBlockType::Kelp,
                age: 10i32,
            } => Ok(13793u32),
            CropBlock {
                block_type: CropBlockType::Kelp,
                age: 11i32,
            } => Ok(13794u32),
            CropBlock {
                block_type: CropBlockType::Kelp,
                age: 12i32,
            } => Ok(13795u32),
            CropBlock {
                block_type: CropBlockType::Kelp,
                age: 13i32,
            } => Ok(13796u32),
            CropBlock {
                block_type: CropBlockType::Kelp,
                age: 14i32,
            } => Ok(13797u32),
            CropBlock {
                block_type: CropBlockType::Kelp,
                age: 15i32,
            } => Ok(13798u32),
            CropBlock {
                block_type: CropBlockType::Kelp,
                age: 16i32,
            } => Ok(13799u32),
            CropBlock {
                block_type: CropBlockType::Kelp,
                age: 17i32,
            } => Ok(13800u32),
            CropBlock {
                block_type: CropBlockType::Kelp,
                age: 18i32,
            } => Ok(13801u32),
            CropBlock {
                block_type: CropBlockType::Kelp,
                age: 19i32,
            } => Ok(13802u32),
            CropBlock {
                block_type: CropBlockType::Kelp,
                age: 20i32,
            } => Ok(13803u32),
            CropBlock {
                block_type: CropBlockType::Kelp,
                age: 21i32,
            } => Ok(13804u32),
            CropBlock {
                block_type: CropBlockType::Kelp,
                age: 22i32,
            } => Ok(13805u32),
            CropBlock {
                block_type: CropBlockType::Kelp,
                age: 23i32,
            } => Ok(13806u32),
            CropBlock {
                block_type: CropBlockType::Kelp,
                age: 24i32,
            } => Ok(13807u32),
            CropBlock {
                block_type: CropBlockType::Kelp,
                age: 25i32,
            } => Ok(13808u32),
            CropBlock {
                block_type: CropBlockType::MelonStem,
                age: 0i32,
            } => Ok(7072u32),
            CropBlock {
                block_type: CropBlockType::MelonStem,
                age: 1i32,
            } => Ok(7073u32),
            CropBlock {
                block_type: CropBlockType::MelonStem,
                age: 2i32,
            } => Ok(7074u32),
            CropBlock {
                block_type: CropBlockType::MelonStem,
                age: 3i32,
            } => Ok(7075u32),
            CropBlock {
                block_type: CropBlockType::MelonStem,
                age: 4i32,
            } => Ok(7076u32),
            CropBlock {
                block_type: CropBlockType::MelonStem,
                age: 5i32,
            } => Ok(7077u32),
            CropBlock {
                block_type: CropBlockType::MelonStem,
                age: 6i32,
            } => Ok(7078u32),
            CropBlock {
                block_type: CropBlockType::MelonStem,
                age: 7i32,
            } => Ok(7079u32),
            CropBlock {
                block_type: CropBlockType::NetherWart,
                age: 0i32,
            } => Ok(8169u32),
            CropBlock {
                block_type: CropBlockType::NetherWart,
                age: 1i32,
            } => Ok(8170u32),
            CropBlock {
                block_type: CropBlockType::NetherWart,
                age: 2i32,
            } => Ok(8171u32),
            CropBlock {
                block_type: CropBlockType::NetherWart,
                age: 3i32,
            } => Ok(8172u32),
            CropBlock {
                block_type: CropBlockType::Potatoes,
                age: 0i32,
            } => Ok(9388u32),
            CropBlock {
                block_type: CropBlockType::Potatoes,
                age: 1i32,
            } => Ok(9389u32),
            CropBlock {
                block_type: CropBlockType::Potatoes,
                age: 2i32,
            } => Ok(9390u32),
            CropBlock {
                block_type: CropBlockType::Potatoes,
                age: 3i32,
            } => Ok(9391u32),
            CropBlock {
                block_type: CropBlockType::Potatoes,
                age: 4i32,
            } => Ok(9392u32),
            CropBlock {
                block_type: CropBlockType::Potatoes,
                age: 5i32,
            } => Ok(9393u32),
            CropBlock {
                block_type: CropBlockType::Potatoes,
                age: 6i32,
            } => Ok(9394u32),
            CropBlock {
                block_type: CropBlockType::Potatoes,
                age: 7i32,
            } => Ok(9395u32),
            CropBlock {
                block_type: CropBlockType::PumpkinStem,
                age: 0i32,
            } => Ok(7064u32),
            CropBlock {
                block_type: CropBlockType::PumpkinStem,
                age: 1i32,
            } => Ok(7065u32),
            CropBlock {
                block_type: CropBlockType::PumpkinStem,
                age: 2i32,
            } => Ok(7066u32),
            CropBlock {
                block_type: CropBlockType::PumpkinStem,
                age: 3i32,
            } => Ok(7067u32),
            CropBlock {
                block_type: CropBlockType::PumpkinStem,
                age: 4i32,
            } => Ok(7068u32),
            CropBlock {
                block_type: CropBlockType::PumpkinStem,
                age: 5i32,
            } => Ok(7069u32),
            CropBlock {
                block_type: CropBlockType::PumpkinStem,
                age: 6i32,
            } => Ok(7070u32),
            CropBlock {
                block_type: CropBlockType::PumpkinStem,
                age: 7i32,
            } => Ok(7071u32),
            CropBlock {
                block_type: CropBlockType::SugarCane,
                age: 0i32,
            } => Ok(5978u32),
            CropBlock {
                block_type: CropBlockType::SugarCane,
                age: 1i32,
            } => Ok(5979u32),
            CropBlock {
                block_type: CropBlockType::SugarCane,
                age: 2i32,
            } => Ok(5980u32),
            CropBlock {
                block_type: CropBlockType::SugarCane,
                age: 3i32,
            } => Ok(5981u32),
            CropBlock {
                block_type: CropBlockType::SugarCane,
                age: 4i32,
            } => Ok(5982u32),
            CropBlock {
                block_type: CropBlockType::SugarCane,
                age: 5i32,
            } => Ok(5983u32),
            CropBlock {
                block_type: CropBlockType::SugarCane,
                age: 6i32,
            } => Ok(5984u32),
            CropBlock {
                block_type: CropBlockType::SugarCane,
                age: 7i32,
            } => Ok(5985u32),
            CropBlock {
                block_type: CropBlockType::SugarCane,
                age: 8i32,
            } => Ok(5986u32),
            CropBlock {
                block_type: CropBlockType::SugarCane,
                age: 9i32,
            } => Ok(5987u32),
            CropBlock {
                block_type: CropBlockType::SugarCane,
                age: 10i32,
            } => Ok(5988u32),
            CropBlock {
                block_type: CropBlockType::SugarCane,
                age: 11i32,
            } => Ok(5989u32),
            CropBlock {
                block_type: CropBlockType::SugarCane,
                age: 12i32,
            } => Ok(5990u32),
            CropBlock {
                block_type: CropBlockType::SugarCane,
                age: 13i32,
            } => Ok(5991u32),
            CropBlock {
                block_type: CropBlockType::SugarCane,
                age: 14i32,
            } => Ok(5992u32),
            CropBlock {
                block_type: CropBlockType::SugarCane,
                age: 15i32,
            } => Ok(5993u32),
            CropBlock {
                block_type: CropBlockType::SweetBerryBush,
                age: 0i32,
            } => Ok(19598u32),
            CropBlock {
                block_type: CropBlockType::SweetBerryBush,
                age: 1i32,
            } => Ok(19599u32),
            CropBlock {
                block_type: CropBlockType::SweetBerryBush,
                age: 2i32,
            } => Ok(19600u32),
            CropBlock {
                block_type: CropBlockType::SweetBerryBush,
                age: 3i32,
            } => Ok(19601u32),
            CropBlock {
                block_type: CropBlockType::TorchflowerCrop,
                age: 0i32,
            } => Ok(13518u32),
            CropBlock {
                block_type: CropBlockType::TorchflowerCrop,
                age: 1i32,
            } => Ok(13519u32),
            CropBlock {
                block_type: CropBlockType::TwistingVines,
                age: 0i32,
            } => Ok(19661u32),
            CropBlock {
                block_type: CropBlockType::TwistingVines,
                age: 1i32,
            } => Ok(19662u32),
            CropBlock {
                block_type: CropBlockType::TwistingVines,
                age: 2i32,
            } => Ok(19663u32),
            CropBlock {
                block_type: CropBlockType::TwistingVines,
                age: 3i32,
            } => Ok(19664u32),
            CropBlock {
                block_type: CropBlockType::TwistingVines,
                age: 4i32,
            } => Ok(19665u32),
            CropBlock {
                block_type: CropBlockType::TwistingVines,
                age: 5i32,
            } => Ok(19666u32),
            CropBlock {
                block_type: CropBlockType::TwistingVines,
                age: 6i32,
            } => Ok(19667u32),
            CropBlock {
                block_type: CropBlockType::TwistingVines,
                age: 7i32,
            } => Ok(19668u32),
            CropBlock {
                block_type: CropBlockType::TwistingVines,
                age: 8i32,
            } => Ok(19669u32),
            CropBlock {
                block_type: CropBlockType::TwistingVines,
                age: 9i32,
            } => Ok(19670u32),
            CropBlock {
                block_type: CropBlockType::TwistingVines,
                age: 10i32,
            } => Ok(19671u32),
            CropBlock {
                block_type: CropBlockType::TwistingVines,
                age: 11i32,
            } => Ok(19672u32),
            CropBlock {
                block_type: CropBlockType::TwistingVines,
                age: 12i32,
            } => Ok(19673u32),
            CropBlock {
                block_type: CropBlockType::TwistingVines,
                age: 13i32,
            } => Ok(19674u32),
            CropBlock {
                block_type: CropBlockType::TwistingVines,
                age: 14i32,
            } => Ok(19675u32),
            CropBlock {
                block_type: CropBlockType::TwistingVines,
                age: 15i32,
            } => Ok(19676u32),
            CropBlock {
                block_type: CropBlockType::TwistingVines,
                age: 16i32,
            } => Ok(19677u32),
            CropBlock {
                block_type: CropBlockType::TwistingVines,
                age: 17i32,
            } => Ok(19678u32),
            CropBlock {
                block_type: CropBlockType::TwistingVines,
                age: 18i32,
            } => Ok(19679u32),
            CropBlock {
                block_type: CropBlockType::TwistingVines,
                age: 19i32,
            } => Ok(19680u32),
            CropBlock {
                block_type: CropBlockType::TwistingVines,
                age: 20i32,
            } => Ok(19681u32),
            CropBlock {
                block_type: CropBlockType::TwistingVines,
                age: 21i32,
            } => Ok(19682u32),
            CropBlock {
                block_type: CropBlockType::TwistingVines,
                age: 22i32,
            } => Ok(19683u32),
            CropBlock {
                block_type: CropBlockType::TwistingVines,
                age: 23i32,
            } => Ok(19684u32),
            CropBlock {
                block_type: CropBlockType::TwistingVines,
                age: 24i32,
            } => Ok(19685u32),
            CropBlock {
                block_type: CropBlockType::TwistingVines,
                age: 25i32,
            } => Ok(19686u32),
            CropBlock {
                block_type: CropBlockType::WeepingVines,
                age: 0i32,
            } => Ok(19634u32),
            CropBlock {
                block_type: CropBlockType::WeepingVines,
                age: 1i32,
            } => Ok(19635u32),
            CropBlock {
                block_type: CropBlockType::WeepingVines,
                age: 2i32,
            } => Ok(19636u32),
            CropBlock {
                block_type: CropBlockType::WeepingVines,
                age: 3i32,
            } => Ok(19637u32),
            CropBlock {
                block_type: CropBlockType::WeepingVines,
                age: 4i32,
            } => Ok(19638u32),
            CropBlock {
                block_type: CropBlockType::WeepingVines,
                age: 5i32,
            } => Ok(19639u32),
            CropBlock {
                block_type: CropBlockType::WeepingVines,
                age: 6i32,
            } => Ok(19640u32),
            CropBlock {
                block_type: CropBlockType::WeepingVines,
                age: 7i32,
            } => Ok(19641u32),
            CropBlock {
                block_type: CropBlockType::WeepingVines,
                age: 8i32,
            } => Ok(19642u32),
            CropBlock {
                block_type: CropBlockType::WeepingVines,
                age: 9i32,
            } => Ok(19643u32),
            CropBlock {
                block_type: CropBlockType::WeepingVines,
                age: 10i32,
            } => Ok(19644u32),
            CropBlock {
                block_type: CropBlockType::WeepingVines,
                age: 11i32,
            } => Ok(19645u32),
            CropBlock {
                block_type: CropBlockType::WeepingVines,
                age: 12i32,
            } => Ok(19646u32),
            CropBlock {
                block_type: CropBlockType::WeepingVines,
                age: 13i32,
            } => Ok(19647u32),
            CropBlock {
                block_type: CropBlockType::WeepingVines,
                age: 14i32,
            } => Ok(19648u32),
            CropBlock {
                block_type: CropBlockType::WeepingVines,
                age: 15i32,
            } => Ok(19649u32),
            CropBlock {
                block_type: CropBlockType::WeepingVines,
                age: 16i32,
            } => Ok(19650u32),
            CropBlock {
                block_type: CropBlockType::WeepingVines,
                age: 17i32,
            } => Ok(19651u32),
            CropBlock {
                block_type: CropBlockType::WeepingVines,
                age: 18i32,
            } => Ok(19652u32),
            CropBlock {
                block_type: CropBlockType::WeepingVines,
                age: 19i32,
            } => Ok(19653u32),
            CropBlock {
                block_type: CropBlockType::WeepingVines,
                age: 20i32,
            } => Ok(19654u32),
            CropBlock {
                block_type: CropBlockType::WeepingVines,
                age: 21i32,
            } => Ok(19655u32),
            CropBlock {
                block_type: CropBlockType::WeepingVines,
                age: 22i32,
            } => Ok(19656u32),
            CropBlock {
                block_type: CropBlockType::WeepingVines,
                age: 23i32,
            } => Ok(19657u32),
            CropBlock {
                block_type: CropBlockType::WeepingVines,
                age: 24i32,
            } => Ok(19658u32),
            CropBlock {
                block_type: CropBlockType::WeepingVines,
                age: 25i32,
            } => Ok(19659u32),
            CropBlock {
                block_type: CropBlockType::Wheat,
                age: 0i32,
            } => Ok(4342u32),
            CropBlock {
                block_type: CropBlockType::Wheat,
                age: 1i32,
            } => Ok(4343u32),
            CropBlock {
                block_type: CropBlockType::Wheat,
                age: 2i32,
            } => Ok(4344u32),
            CropBlock {
                block_type: CropBlockType::Wheat,
                age: 3i32,
            } => Ok(4345u32),
            CropBlock {
                block_type: CropBlockType::Wheat,
                age: 4i32,
            } => Ok(4346u32),
            CropBlock {
                block_type: CropBlockType::Wheat,
                age: 5i32,
            } => Ok(4347u32),
            CropBlock {
                block_type: CropBlockType::Wheat,
                age: 6i32,
            } => Ok(4348u32),
            CropBlock {
                block_type: CropBlockType::Wheat,
                age: 7i32,
            } => Ok(4349u32),
            _ => Err(()),
        }
    }
}

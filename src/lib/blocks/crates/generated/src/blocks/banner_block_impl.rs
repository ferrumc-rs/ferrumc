use crate::BannerBlock;
use crate::BannerBlockType;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for BannerBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            11888u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BlackBanner,
                rotation: 0i32,
            }),
            11889u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BlackBanner,
                rotation: 1i32,
            }),
            11890u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BlackBanner,
                rotation: 2i32,
            }),
            11891u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BlackBanner,
                rotation: 3i32,
            }),
            11892u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BlackBanner,
                rotation: 4i32,
            }),
            11893u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BlackBanner,
                rotation: 5i32,
            }),
            11894u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BlackBanner,
                rotation: 6i32,
            }),
            11895u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BlackBanner,
                rotation: 7i32,
            }),
            11896u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BlackBanner,
                rotation: 8i32,
            }),
            11897u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BlackBanner,
                rotation: 9i32,
            }),
            11898u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BlackBanner,
                rotation: 10i32,
            }),
            11899u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BlackBanner,
                rotation: 11i32,
            }),
            11900u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BlackBanner,
                rotation: 12i32,
            }),
            11901u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BlackBanner,
                rotation: 13i32,
            }),
            11902u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BlackBanner,
                rotation: 14i32,
            }),
            11903u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BlackBanner,
                rotation: 15i32,
            }),
            11824u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BlueBanner,
                rotation: 0i32,
            }),
            11825u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BlueBanner,
                rotation: 1i32,
            }),
            11826u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BlueBanner,
                rotation: 2i32,
            }),
            11827u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BlueBanner,
                rotation: 3i32,
            }),
            11828u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BlueBanner,
                rotation: 4i32,
            }),
            11829u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BlueBanner,
                rotation: 5i32,
            }),
            11830u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BlueBanner,
                rotation: 6i32,
            }),
            11831u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BlueBanner,
                rotation: 7i32,
            }),
            11832u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BlueBanner,
                rotation: 8i32,
            }),
            11833u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BlueBanner,
                rotation: 9i32,
            }),
            11834u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BlueBanner,
                rotation: 10i32,
            }),
            11835u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BlueBanner,
                rotation: 11i32,
            }),
            11836u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BlueBanner,
                rotation: 12i32,
            }),
            11837u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BlueBanner,
                rotation: 13i32,
            }),
            11838u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BlueBanner,
                rotation: 14i32,
            }),
            11839u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BlueBanner,
                rotation: 15i32,
            }),
            11840u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BrownBanner,
                rotation: 0i32,
            }),
            11841u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BrownBanner,
                rotation: 1i32,
            }),
            11842u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BrownBanner,
                rotation: 2i32,
            }),
            11843u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BrownBanner,
                rotation: 3i32,
            }),
            11844u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BrownBanner,
                rotation: 4i32,
            }),
            11845u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BrownBanner,
                rotation: 5i32,
            }),
            11846u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BrownBanner,
                rotation: 6i32,
            }),
            11847u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BrownBanner,
                rotation: 7i32,
            }),
            11848u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BrownBanner,
                rotation: 8i32,
            }),
            11849u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BrownBanner,
                rotation: 9i32,
            }),
            11850u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BrownBanner,
                rotation: 10i32,
            }),
            11851u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BrownBanner,
                rotation: 11i32,
            }),
            11852u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BrownBanner,
                rotation: 12i32,
            }),
            11853u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BrownBanner,
                rotation: 13i32,
            }),
            11854u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BrownBanner,
                rotation: 14i32,
            }),
            11855u32 => Ok(BannerBlock {
                block_type: BannerBlockType::BrownBanner,
                rotation: 15i32,
            }),
            11792u32 => Ok(BannerBlock {
                block_type: BannerBlockType::CyanBanner,
                rotation: 0i32,
            }),
            11793u32 => Ok(BannerBlock {
                block_type: BannerBlockType::CyanBanner,
                rotation: 1i32,
            }),
            11794u32 => Ok(BannerBlock {
                block_type: BannerBlockType::CyanBanner,
                rotation: 2i32,
            }),
            11795u32 => Ok(BannerBlock {
                block_type: BannerBlockType::CyanBanner,
                rotation: 3i32,
            }),
            11796u32 => Ok(BannerBlock {
                block_type: BannerBlockType::CyanBanner,
                rotation: 4i32,
            }),
            11797u32 => Ok(BannerBlock {
                block_type: BannerBlockType::CyanBanner,
                rotation: 5i32,
            }),
            11798u32 => Ok(BannerBlock {
                block_type: BannerBlockType::CyanBanner,
                rotation: 6i32,
            }),
            11799u32 => Ok(BannerBlock {
                block_type: BannerBlockType::CyanBanner,
                rotation: 7i32,
            }),
            11800u32 => Ok(BannerBlock {
                block_type: BannerBlockType::CyanBanner,
                rotation: 8i32,
            }),
            11801u32 => Ok(BannerBlock {
                block_type: BannerBlockType::CyanBanner,
                rotation: 9i32,
            }),
            11802u32 => Ok(BannerBlock {
                block_type: BannerBlockType::CyanBanner,
                rotation: 10i32,
            }),
            11803u32 => Ok(BannerBlock {
                block_type: BannerBlockType::CyanBanner,
                rotation: 11i32,
            }),
            11804u32 => Ok(BannerBlock {
                block_type: BannerBlockType::CyanBanner,
                rotation: 12i32,
            }),
            11805u32 => Ok(BannerBlock {
                block_type: BannerBlockType::CyanBanner,
                rotation: 13i32,
            }),
            11806u32 => Ok(BannerBlock {
                block_type: BannerBlockType::CyanBanner,
                rotation: 14i32,
            }),
            11807u32 => Ok(BannerBlock {
                block_type: BannerBlockType::CyanBanner,
                rotation: 15i32,
            }),
            11760u32 => Ok(BannerBlock {
                block_type: BannerBlockType::GrayBanner,
                rotation: 0i32,
            }),
            11761u32 => Ok(BannerBlock {
                block_type: BannerBlockType::GrayBanner,
                rotation: 1i32,
            }),
            11762u32 => Ok(BannerBlock {
                block_type: BannerBlockType::GrayBanner,
                rotation: 2i32,
            }),
            11763u32 => Ok(BannerBlock {
                block_type: BannerBlockType::GrayBanner,
                rotation: 3i32,
            }),
            11764u32 => Ok(BannerBlock {
                block_type: BannerBlockType::GrayBanner,
                rotation: 4i32,
            }),
            11765u32 => Ok(BannerBlock {
                block_type: BannerBlockType::GrayBanner,
                rotation: 5i32,
            }),
            11766u32 => Ok(BannerBlock {
                block_type: BannerBlockType::GrayBanner,
                rotation: 6i32,
            }),
            11767u32 => Ok(BannerBlock {
                block_type: BannerBlockType::GrayBanner,
                rotation: 7i32,
            }),
            11768u32 => Ok(BannerBlock {
                block_type: BannerBlockType::GrayBanner,
                rotation: 8i32,
            }),
            11769u32 => Ok(BannerBlock {
                block_type: BannerBlockType::GrayBanner,
                rotation: 9i32,
            }),
            11770u32 => Ok(BannerBlock {
                block_type: BannerBlockType::GrayBanner,
                rotation: 10i32,
            }),
            11771u32 => Ok(BannerBlock {
                block_type: BannerBlockType::GrayBanner,
                rotation: 11i32,
            }),
            11772u32 => Ok(BannerBlock {
                block_type: BannerBlockType::GrayBanner,
                rotation: 12i32,
            }),
            11773u32 => Ok(BannerBlock {
                block_type: BannerBlockType::GrayBanner,
                rotation: 13i32,
            }),
            11774u32 => Ok(BannerBlock {
                block_type: BannerBlockType::GrayBanner,
                rotation: 14i32,
            }),
            11775u32 => Ok(BannerBlock {
                block_type: BannerBlockType::GrayBanner,
                rotation: 15i32,
            }),
            11856u32 => Ok(BannerBlock {
                block_type: BannerBlockType::GreenBanner,
                rotation: 0i32,
            }),
            11857u32 => Ok(BannerBlock {
                block_type: BannerBlockType::GreenBanner,
                rotation: 1i32,
            }),
            11858u32 => Ok(BannerBlock {
                block_type: BannerBlockType::GreenBanner,
                rotation: 2i32,
            }),
            11859u32 => Ok(BannerBlock {
                block_type: BannerBlockType::GreenBanner,
                rotation: 3i32,
            }),
            11860u32 => Ok(BannerBlock {
                block_type: BannerBlockType::GreenBanner,
                rotation: 4i32,
            }),
            11861u32 => Ok(BannerBlock {
                block_type: BannerBlockType::GreenBanner,
                rotation: 5i32,
            }),
            11862u32 => Ok(BannerBlock {
                block_type: BannerBlockType::GreenBanner,
                rotation: 6i32,
            }),
            11863u32 => Ok(BannerBlock {
                block_type: BannerBlockType::GreenBanner,
                rotation: 7i32,
            }),
            11864u32 => Ok(BannerBlock {
                block_type: BannerBlockType::GreenBanner,
                rotation: 8i32,
            }),
            11865u32 => Ok(BannerBlock {
                block_type: BannerBlockType::GreenBanner,
                rotation: 9i32,
            }),
            11866u32 => Ok(BannerBlock {
                block_type: BannerBlockType::GreenBanner,
                rotation: 10i32,
            }),
            11867u32 => Ok(BannerBlock {
                block_type: BannerBlockType::GreenBanner,
                rotation: 11i32,
            }),
            11868u32 => Ok(BannerBlock {
                block_type: BannerBlockType::GreenBanner,
                rotation: 12i32,
            }),
            11869u32 => Ok(BannerBlock {
                block_type: BannerBlockType::GreenBanner,
                rotation: 13i32,
            }),
            11870u32 => Ok(BannerBlock {
                block_type: BannerBlockType::GreenBanner,
                rotation: 14i32,
            }),
            11871u32 => Ok(BannerBlock {
                block_type: BannerBlockType::GreenBanner,
                rotation: 15i32,
            }),
            11696u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LightBlueBanner,
                rotation: 0i32,
            }),
            11697u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LightBlueBanner,
                rotation: 1i32,
            }),
            11698u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LightBlueBanner,
                rotation: 2i32,
            }),
            11699u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LightBlueBanner,
                rotation: 3i32,
            }),
            11700u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LightBlueBanner,
                rotation: 4i32,
            }),
            11701u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LightBlueBanner,
                rotation: 5i32,
            }),
            11702u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LightBlueBanner,
                rotation: 6i32,
            }),
            11703u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LightBlueBanner,
                rotation: 7i32,
            }),
            11704u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LightBlueBanner,
                rotation: 8i32,
            }),
            11705u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LightBlueBanner,
                rotation: 9i32,
            }),
            11706u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LightBlueBanner,
                rotation: 10i32,
            }),
            11707u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LightBlueBanner,
                rotation: 11i32,
            }),
            11708u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LightBlueBanner,
                rotation: 12i32,
            }),
            11709u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LightBlueBanner,
                rotation: 13i32,
            }),
            11710u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LightBlueBanner,
                rotation: 14i32,
            }),
            11711u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LightBlueBanner,
                rotation: 15i32,
            }),
            11776u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LightGrayBanner,
                rotation: 0i32,
            }),
            11777u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LightGrayBanner,
                rotation: 1i32,
            }),
            11778u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LightGrayBanner,
                rotation: 2i32,
            }),
            11779u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LightGrayBanner,
                rotation: 3i32,
            }),
            11780u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LightGrayBanner,
                rotation: 4i32,
            }),
            11781u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LightGrayBanner,
                rotation: 5i32,
            }),
            11782u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LightGrayBanner,
                rotation: 6i32,
            }),
            11783u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LightGrayBanner,
                rotation: 7i32,
            }),
            11784u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LightGrayBanner,
                rotation: 8i32,
            }),
            11785u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LightGrayBanner,
                rotation: 9i32,
            }),
            11786u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LightGrayBanner,
                rotation: 10i32,
            }),
            11787u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LightGrayBanner,
                rotation: 11i32,
            }),
            11788u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LightGrayBanner,
                rotation: 12i32,
            }),
            11789u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LightGrayBanner,
                rotation: 13i32,
            }),
            11790u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LightGrayBanner,
                rotation: 14i32,
            }),
            11791u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LightGrayBanner,
                rotation: 15i32,
            }),
            11728u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LimeBanner,
                rotation: 0i32,
            }),
            11729u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LimeBanner,
                rotation: 1i32,
            }),
            11730u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LimeBanner,
                rotation: 2i32,
            }),
            11731u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LimeBanner,
                rotation: 3i32,
            }),
            11732u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LimeBanner,
                rotation: 4i32,
            }),
            11733u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LimeBanner,
                rotation: 5i32,
            }),
            11734u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LimeBanner,
                rotation: 6i32,
            }),
            11735u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LimeBanner,
                rotation: 7i32,
            }),
            11736u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LimeBanner,
                rotation: 8i32,
            }),
            11737u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LimeBanner,
                rotation: 9i32,
            }),
            11738u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LimeBanner,
                rotation: 10i32,
            }),
            11739u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LimeBanner,
                rotation: 11i32,
            }),
            11740u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LimeBanner,
                rotation: 12i32,
            }),
            11741u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LimeBanner,
                rotation: 13i32,
            }),
            11742u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LimeBanner,
                rotation: 14i32,
            }),
            11743u32 => Ok(BannerBlock {
                block_type: BannerBlockType::LimeBanner,
                rotation: 15i32,
            }),
            11680u32 => Ok(BannerBlock {
                block_type: BannerBlockType::MagentaBanner,
                rotation: 0i32,
            }),
            11681u32 => Ok(BannerBlock {
                block_type: BannerBlockType::MagentaBanner,
                rotation: 1i32,
            }),
            11682u32 => Ok(BannerBlock {
                block_type: BannerBlockType::MagentaBanner,
                rotation: 2i32,
            }),
            11683u32 => Ok(BannerBlock {
                block_type: BannerBlockType::MagentaBanner,
                rotation: 3i32,
            }),
            11684u32 => Ok(BannerBlock {
                block_type: BannerBlockType::MagentaBanner,
                rotation: 4i32,
            }),
            11685u32 => Ok(BannerBlock {
                block_type: BannerBlockType::MagentaBanner,
                rotation: 5i32,
            }),
            11686u32 => Ok(BannerBlock {
                block_type: BannerBlockType::MagentaBanner,
                rotation: 6i32,
            }),
            11687u32 => Ok(BannerBlock {
                block_type: BannerBlockType::MagentaBanner,
                rotation: 7i32,
            }),
            11688u32 => Ok(BannerBlock {
                block_type: BannerBlockType::MagentaBanner,
                rotation: 8i32,
            }),
            11689u32 => Ok(BannerBlock {
                block_type: BannerBlockType::MagentaBanner,
                rotation: 9i32,
            }),
            11690u32 => Ok(BannerBlock {
                block_type: BannerBlockType::MagentaBanner,
                rotation: 10i32,
            }),
            11691u32 => Ok(BannerBlock {
                block_type: BannerBlockType::MagentaBanner,
                rotation: 11i32,
            }),
            11692u32 => Ok(BannerBlock {
                block_type: BannerBlockType::MagentaBanner,
                rotation: 12i32,
            }),
            11693u32 => Ok(BannerBlock {
                block_type: BannerBlockType::MagentaBanner,
                rotation: 13i32,
            }),
            11694u32 => Ok(BannerBlock {
                block_type: BannerBlockType::MagentaBanner,
                rotation: 14i32,
            }),
            11695u32 => Ok(BannerBlock {
                block_type: BannerBlockType::MagentaBanner,
                rotation: 15i32,
            }),
            11664u32 => Ok(BannerBlock {
                block_type: BannerBlockType::OrangeBanner,
                rotation: 0i32,
            }),
            11665u32 => Ok(BannerBlock {
                block_type: BannerBlockType::OrangeBanner,
                rotation: 1i32,
            }),
            11666u32 => Ok(BannerBlock {
                block_type: BannerBlockType::OrangeBanner,
                rotation: 2i32,
            }),
            11667u32 => Ok(BannerBlock {
                block_type: BannerBlockType::OrangeBanner,
                rotation: 3i32,
            }),
            11668u32 => Ok(BannerBlock {
                block_type: BannerBlockType::OrangeBanner,
                rotation: 4i32,
            }),
            11669u32 => Ok(BannerBlock {
                block_type: BannerBlockType::OrangeBanner,
                rotation: 5i32,
            }),
            11670u32 => Ok(BannerBlock {
                block_type: BannerBlockType::OrangeBanner,
                rotation: 6i32,
            }),
            11671u32 => Ok(BannerBlock {
                block_type: BannerBlockType::OrangeBanner,
                rotation: 7i32,
            }),
            11672u32 => Ok(BannerBlock {
                block_type: BannerBlockType::OrangeBanner,
                rotation: 8i32,
            }),
            11673u32 => Ok(BannerBlock {
                block_type: BannerBlockType::OrangeBanner,
                rotation: 9i32,
            }),
            11674u32 => Ok(BannerBlock {
                block_type: BannerBlockType::OrangeBanner,
                rotation: 10i32,
            }),
            11675u32 => Ok(BannerBlock {
                block_type: BannerBlockType::OrangeBanner,
                rotation: 11i32,
            }),
            11676u32 => Ok(BannerBlock {
                block_type: BannerBlockType::OrangeBanner,
                rotation: 12i32,
            }),
            11677u32 => Ok(BannerBlock {
                block_type: BannerBlockType::OrangeBanner,
                rotation: 13i32,
            }),
            11678u32 => Ok(BannerBlock {
                block_type: BannerBlockType::OrangeBanner,
                rotation: 14i32,
            }),
            11679u32 => Ok(BannerBlock {
                block_type: BannerBlockType::OrangeBanner,
                rotation: 15i32,
            }),
            11744u32 => Ok(BannerBlock {
                block_type: BannerBlockType::PinkBanner,
                rotation: 0i32,
            }),
            11745u32 => Ok(BannerBlock {
                block_type: BannerBlockType::PinkBanner,
                rotation: 1i32,
            }),
            11746u32 => Ok(BannerBlock {
                block_type: BannerBlockType::PinkBanner,
                rotation: 2i32,
            }),
            11747u32 => Ok(BannerBlock {
                block_type: BannerBlockType::PinkBanner,
                rotation: 3i32,
            }),
            11748u32 => Ok(BannerBlock {
                block_type: BannerBlockType::PinkBanner,
                rotation: 4i32,
            }),
            11749u32 => Ok(BannerBlock {
                block_type: BannerBlockType::PinkBanner,
                rotation: 5i32,
            }),
            11750u32 => Ok(BannerBlock {
                block_type: BannerBlockType::PinkBanner,
                rotation: 6i32,
            }),
            11751u32 => Ok(BannerBlock {
                block_type: BannerBlockType::PinkBanner,
                rotation: 7i32,
            }),
            11752u32 => Ok(BannerBlock {
                block_type: BannerBlockType::PinkBanner,
                rotation: 8i32,
            }),
            11753u32 => Ok(BannerBlock {
                block_type: BannerBlockType::PinkBanner,
                rotation: 9i32,
            }),
            11754u32 => Ok(BannerBlock {
                block_type: BannerBlockType::PinkBanner,
                rotation: 10i32,
            }),
            11755u32 => Ok(BannerBlock {
                block_type: BannerBlockType::PinkBanner,
                rotation: 11i32,
            }),
            11756u32 => Ok(BannerBlock {
                block_type: BannerBlockType::PinkBanner,
                rotation: 12i32,
            }),
            11757u32 => Ok(BannerBlock {
                block_type: BannerBlockType::PinkBanner,
                rotation: 13i32,
            }),
            11758u32 => Ok(BannerBlock {
                block_type: BannerBlockType::PinkBanner,
                rotation: 14i32,
            }),
            11759u32 => Ok(BannerBlock {
                block_type: BannerBlockType::PinkBanner,
                rotation: 15i32,
            }),
            11808u32 => Ok(BannerBlock {
                block_type: BannerBlockType::PurpleBanner,
                rotation: 0i32,
            }),
            11809u32 => Ok(BannerBlock {
                block_type: BannerBlockType::PurpleBanner,
                rotation: 1i32,
            }),
            11810u32 => Ok(BannerBlock {
                block_type: BannerBlockType::PurpleBanner,
                rotation: 2i32,
            }),
            11811u32 => Ok(BannerBlock {
                block_type: BannerBlockType::PurpleBanner,
                rotation: 3i32,
            }),
            11812u32 => Ok(BannerBlock {
                block_type: BannerBlockType::PurpleBanner,
                rotation: 4i32,
            }),
            11813u32 => Ok(BannerBlock {
                block_type: BannerBlockType::PurpleBanner,
                rotation: 5i32,
            }),
            11814u32 => Ok(BannerBlock {
                block_type: BannerBlockType::PurpleBanner,
                rotation: 6i32,
            }),
            11815u32 => Ok(BannerBlock {
                block_type: BannerBlockType::PurpleBanner,
                rotation: 7i32,
            }),
            11816u32 => Ok(BannerBlock {
                block_type: BannerBlockType::PurpleBanner,
                rotation: 8i32,
            }),
            11817u32 => Ok(BannerBlock {
                block_type: BannerBlockType::PurpleBanner,
                rotation: 9i32,
            }),
            11818u32 => Ok(BannerBlock {
                block_type: BannerBlockType::PurpleBanner,
                rotation: 10i32,
            }),
            11819u32 => Ok(BannerBlock {
                block_type: BannerBlockType::PurpleBanner,
                rotation: 11i32,
            }),
            11820u32 => Ok(BannerBlock {
                block_type: BannerBlockType::PurpleBanner,
                rotation: 12i32,
            }),
            11821u32 => Ok(BannerBlock {
                block_type: BannerBlockType::PurpleBanner,
                rotation: 13i32,
            }),
            11822u32 => Ok(BannerBlock {
                block_type: BannerBlockType::PurpleBanner,
                rotation: 14i32,
            }),
            11823u32 => Ok(BannerBlock {
                block_type: BannerBlockType::PurpleBanner,
                rotation: 15i32,
            }),
            11872u32 => Ok(BannerBlock {
                block_type: BannerBlockType::RedBanner,
                rotation: 0i32,
            }),
            11873u32 => Ok(BannerBlock {
                block_type: BannerBlockType::RedBanner,
                rotation: 1i32,
            }),
            11874u32 => Ok(BannerBlock {
                block_type: BannerBlockType::RedBanner,
                rotation: 2i32,
            }),
            11875u32 => Ok(BannerBlock {
                block_type: BannerBlockType::RedBanner,
                rotation: 3i32,
            }),
            11876u32 => Ok(BannerBlock {
                block_type: BannerBlockType::RedBanner,
                rotation: 4i32,
            }),
            11877u32 => Ok(BannerBlock {
                block_type: BannerBlockType::RedBanner,
                rotation: 5i32,
            }),
            11878u32 => Ok(BannerBlock {
                block_type: BannerBlockType::RedBanner,
                rotation: 6i32,
            }),
            11879u32 => Ok(BannerBlock {
                block_type: BannerBlockType::RedBanner,
                rotation: 7i32,
            }),
            11880u32 => Ok(BannerBlock {
                block_type: BannerBlockType::RedBanner,
                rotation: 8i32,
            }),
            11881u32 => Ok(BannerBlock {
                block_type: BannerBlockType::RedBanner,
                rotation: 9i32,
            }),
            11882u32 => Ok(BannerBlock {
                block_type: BannerBlockType::RedBanner,
                rotation: 10i32,
            }),
            11883u32 => Ok(BannerBlock {
                block_type: BannerBlockType::RedBanner,
                rotation: 11i32,
            }),
            11884u32 => Ok(BannerBlock {
                block_type: BannerBlockType::RedBanner,
                rotation: 12i32,
            }),
            11885u32 => Ok(BannerBlock {
                block_type: BannerBlockType::RedBanner,
                rotation: 13i32,
            }),
            11886u32 => Ok(BannerBlock {
                block_type: BannerBlockType::RedBanner,
                rotation: 14i32,
            }),
            11887u32 => Ok(BannerBlock {
                block_type: BannerBlockType::RedBanner,
                rotation: 15i32,
            }),
            11648u32 => Ok(BannerBlock {
                block_type: BannerBlockType::WhiteBanner,
                rotation: 0i32,
            }),
            11649u32 => Ok(BannerBlock {
                block_type: BannerBlockType::WhiteBanner,
                rotation: 1i32,
            }),
            11650u32 => Ok(BannerBlock {
                block_type: BannerBlockType::WhiteBanner,
                rotation: 2i32,
            }),
            11651u32 => Ok(BannerBlock {
                block_type: BannerBlockType::WhiteBanner,
                rotation: 3i32,
            }),
            11652u32 => Ok(BannerBlock {
                block_type: BannerBlockType::WhiteBanner,
                rotation: 4i32,
            }),
            11653u32 => Ok(BannerBlock {
                block_type: BannerBlockType::WhiteBanner,
                rotation: 5i32,
            }),
            11654u32 => Ok(BannerBlock {
                block_type: BannerBlockType::WhiteBanner,
                rotation: 6i32,
            }),
            11655u32 => Ok(BannerBlock {
                block_type: BannerBlockType::WhiteBanner,
                rotation: 7i32,
            }),
            11656u32 => Ok(BannerBlock {
                block_type: BannerBlockType::WhiteBanner,
                rotation: 8i32,
            }),
            11657u32 => Ok(BannerBlock {
                block_type: BannerBlockType::WhiteBanner,
                rotation: 9i32,
            }),
            11658u32 => Ok(BannerBlock {
                block_type: BannerBlockType::WhiteBanner,
                rotation: 10i32,
            }),
            11659u32 => Ok(BannerBlock {
                block_type: BannerBlockType::WhiteBanner,
                rotation: 11i32,
            }),
            11660u32 => Ok(BannerBlock {
                block_type: BannerBlockType::WhiteBanner,
                rotation: 12i32,
            }),
            11661u32 => Ok(BannerBlock {
                block_type: BannerBlockType::WhiteBanner,
                rotation: 13i32,
            }),
            11662u32 => Ok(BannerBlock {
                block_type: BannerBlockType::WhiteBanner,
                rotation: 14i32,
            }),
            11663u32 => Ok(BannerBlock {
                block_type: BannerBlockType::WhiteBanner,
                rotation: 15i32,
            }),
            11712u32 => Ok(BannerBlock {
                block_type: BannerBlockType::YellowBanner,
                rotation: 0i32,
            }),
            11713u32 => Ok(BannerBlock {
                block_type: BannerBlockType::YellowBanner,
                rotation: 1i32,
            }),
            11714u32 => Ok(BannerBlock {
                block_type: BannerBlockType::YellowBanner,
                rotation: 2i32,
            }),
            11715u32 => Ok(BannerBlock {
                block_type: BannerBlockType::YellowBanner,
                rotation: 3i32,
            }),
            11716u32 => Ok(BannerBlock {
                block_type: BannerBlockType::YellowBanner,
                rotation: 4i32,
            }),
            11717u32 => Ok(BannerBlock {
                block_type: BannerBlockType::YellowBanner,
                rotation: 5i32,
            }),
            11718u32 => Ok(BannerBlock {
                block_type: BannerBlockType::YellowBanner,
                rotation: 6i32,
            }),
            11719u32 => Ok(BannerBlock {
                block_type: BannerBlockType::YellowBanner,
                rotation: 7i32,
            }),
            11720u32 => Ok(BannerBlock {
                block_type: BannerBlockType::YellowBanner,
                rotation: 8i32,
            }),
            11721u32 => Ok(BannerBlock {
                block_type: BannerBlockType::YellowBanner,
                rotation: 9i32,
            }),
            11722u32 => Ok(BannerBlock {
                block_type: BannerBlockType::YellowBanner,
                rotation: 10i32,
            }),
            11723u32 => Ok(BannerBlock {
                block_type: BannerBlockType::YellowBanner,
                rotation: 11i32,
            }),
            11724u32 => Ok(BannerBlock {
                block_type: BannerBlockType::YellowBanner,
                rotation: 12i32,
            }),
            11725u32 => Ok(BannerBlock {
                block_type: BannerBlockType::YellowBanner,
                rotation: 13i32,
            }),
            11726u32 => Ok(BannerBlock {
                block_type: BannerBlockType::YellowBanner,
                rotation: 14i32,
            }),
            11727u32 => Ok(BannerBlock {
                block_type: BannerBlockType::YellowBanner,
                rotation: 15i32,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for BannerBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            BannerBlock {
                block_type: BannerBlockType::BlackBanner,
                rotation: 0i32,
            } => Ok(11888u32),
            BannerBlock {
                block_type: BannerBlockType::BlackBanner,
                rotation: 1i32,
            } => Ok(11889u32),
            BannerBlock {
                block_type: BannerBlockType::BlackBanner,
                rotation: 2i32,
            } => Ok(11890u32),
            BannerBlock {
                block_type: BannerBlockType::BlackBanner,
                rotation: 3i32,
            } => Ok(11891u32),
            BannerBlock {
                block_type: BannerBlockType::BlackBanner,
                rotation: 4i32,
            } => Ok(11892u32),
            BannerBlock {
                block_type: BannerBlockType::BlackBanner,
                rotation: 5i32,
            } => Ok(11893u32),
            BannerBlock {
                block_type: BannerBlockType::BlackBanner,
                rotation: 6i32,
            } => Ok(11894u32),
            BannerBlock {
                block_type: BannerBlockType::BlackBanner,
                rotation: 7i32,
            } => Ok(11895u32),
            BannerBlock {
                block_type: BannerBlockType::BlackBanner,
                rotation: 8i32,
            } => Ok(11896u32),
            BannerBlock {
                block_type: BannerBlockType::BlackBanner,
                rotation: 9i32,
            } => Ok(11897u32),
            BannerBlock {
                block_type: BannerBlockType::BlackBanner,
                rotation: 10i32,
            } => Ok(11898u32),
            BannerBlock {
                block_type: BannerBlockType::BlackBanner,
                rotation: 11i32,
            } => Ok(11899u32),
            BannerBlock {
                block_type: BannerBlockType::BlackBanner,
                rotation: 12i32,
            } => Ok(11900u32),
            BannerBlock {
                block_type: BannerBlockType::BlackBanner,
                rotation: 13i32,
            } => Ok(11901u32),
            BannerBlock {
                block_type: BannerBlockType::BlackBanner,
                rotation: 14i32,
            } => Ok(11902u32),
            BannerBlock {
                block_type: BannerBlockType::BlackBanner,
                rotation: 15i32,
            } => Ok(11903u32),
            BannerBlock {
                block_type: BannerBlockType::BlueBanner,
                rotation: 0i32,
            } => Ok(11824u32),
            BannerBlock {
                block_type: BannerBlockType::BlueBanner,
                rotation: 1i32,
            } => Ok(11825u32),
            BannerBlock {
                block_type: BannerBlockType::BlueBanner,
                rotation: 2i32,
            } => Ok(11826u32),
            BannerBlock {
                block_type: BannerBlockType::BlueBanner,
                rotation: 3i32,
            } => Ok(11827u32),
            BannerBlock {
                block_type: BannerBlockType::BlueBanner,
                rotation: 4i32,
            } => Ok(11828u32),
            BannerBlock {
                block_type: BannerBlockType::BlueBanner,
                rotation: 5i32,
            } => Ok(11829u32),
            BannerBlock {
                block_type: BannerBlockType::BlueBanner,
                rotation: 6i32,
            } => Ok(11830u32),
            BannerBlock {
                block_type: BannerBlockType::BlueBanner,
                rotation: 7i32,
            } => Ok(11831u32),
            BannerBlock {
                block_type: BannerBlockType::BlueBanner,
                rotation: 8i32,
            } => Ok(11832u32),
            BannerBlock {
                block_type: BannerBlockType::BlueBanner,
                rotation: 9i32,
            } => Ok(11833u32),
            BannerBlock {
                block_type: BannerBlockType::BlueBanner,
                rotation: 10i32,
            } => Ok(11834u32),
            BannerBlock {
                block_type: BannerBlockType::BlueBanner,
                rotation: 11i32,
            } => Ok(11835u32),
            BannerBlock {
                block_type: BannerBlockType::BlueBanner,
                rotation: 12i32,
            } => Ok(11836u32),
            BannerBlock {
                block_type: BannerBlockType::BlueBanner,
                rotation: 13i32,
            } => Ok(11837u32),
            BannerBlock {
                block_type: BannerBlockType::BlueBanner,
                rotation: 14i32,
            } => Ok(11838u32),
            BannerBlock {
                block_type: BannerBlockType::BlueBanner,
                rotation: 15i32,
            } => Ok(11839u32),
            BannerBlock {
                block_type: BannerBlockType::BrownBanner,
                rotation: 0i32,
            } => Ok(11840u32),
            BannerBlock {
                block_type: BannerBlockType::BrownBanner,
                rotation: 1i32,
            } => Ok(11841u32),
            BannerBlock {
                block_type: BannerBlockType::BrownBanner,
                rotation: 2i32,
            } => Ok(11842u32),
            BannerBlock {
                block_type: BannerBlockType::BrownBanner,
                rotation: 3i32,
            } => Ok(11843u32),
            BannerBlock {
                block_type: BannerBlockType::BrownBanner,
                rotation: 4i32,
            } => Ok(11844u32),
            BannerBlock {
                block_type: BannerBlockType::BrownBanner,
                rotation: 5i32,
            } => Ok(11845u32),
            BannerBlock {
                block_type: BannerBlockType::BrownBanner,
                rotation: 6i32,
            } => Ok(11846u32),
            BannerBlock {
                block_type: BannerBlockType::BrownBanner,
                rotation: 7i32,
            } => Ok(11847u32),
            BannerBlock {
                block_type: BannerBlockType::BrownBanner,
                rotation: 8i32,
            } => Ok(11848u32),
            BannerBlock {
                block_type: BannerBlockType::BrownBanner,
                rotation: 9i32,
            } => Ok(11849u32),
            BannerBlock {
                block_type: BannerBlockType::BrownBanner,
                rotation: 10i32,
            } => Ok(11850u32),
            BannerBlock {
                block_type: BannerBlockType::BrownBanner,
                rotation: 11i32,
            } => Ok(11851u32),
            BannerBlock {
                block_type: BannerBlockType::BrownBanner,
                rotation: 12i32,
            } => Ok(11852u32),
            BannerBlock {
                block_type: BannerBlockType::BrownBanner,
                rotation: 13i32,
            } => Ok(11853u32),
            BannerBlock {
                block_type: BannerBlockType::BrownBanner,
                rotation: 14i32,
            } => Ok(11854u32),
            BannerBlock {
                block_type: BannerBlockType::BrownBanner,
                rotation: 15i32,
            } => Ok(11855u32),
            BannerBlock {
                block_type: BannerBlockType::CyanBanner,
                rotation: 0i32,
            } => Ok(11792u32),
            BannerBlock {
                block_type: BannerBlockType::CyanBanner,
                rotation: 1i32,
            } => Ok(11793u32),
            BannerBlock {
                block_type: BannerBlockType::CyanBanner,
                rotation: 2i32,
            } => Ok(11794u32),
            BannerBlock {
                block_type: BannerBlockType::CyanBanner,
                rotation: 3i32,
            } => Ok(11795u32),
            BannerBlock {
                block_type: BannerBlockType::CyanBanner,
                rotation: 4i32,
            } => Ok(11796u32),
            BannerBlock {
                block_type: BannerBlockType::CyanBanner,
                rotation: 5i32,
            } => Ok(11797u32),
            BannerBlock {
                block_type: BannerBlockType::CyanBanner,
                rotation: 6i32,
            } => Ok(11798u32),
            BannerBlock {
                block_type: BannerBlockType::CyanBanner,
                rotation: 7i32,
            } => Ok(11799u32),
            BannerBlock {
                block_type: BannerBlockType::CyanBanner,
                rotation: 8i32,
            } => Ok(11800u32),
            BannerBlock {
                block_type: BannerBlockType::CyanBanner,
                rotation: 9i32,
            } => Ok(11801u32),
            BannerBlock {
                block_type: BannerBlockType::CyanBanner,
                rotation: 10i32,
            } => Ok(11802u32),
            BannerBlock {
                block_type: BannerBlockType::CyanBanner,
                rotation: 11i32,
            } => Ok(11803u32),
            BannerBlock {
                block_type: BannerBlockType::CyanBanner,
                rotation: 12i32,
            } => Ok(11804u32),
            BannerBlock {
                block_type: BannerBlockType::CyanBanner,
                rotation: 13i32,
            } => Ok(11805u32),
            BannerBlock {
                block_type: BannerBlockType::CyanBanner,
                rotation: 14i32,
            } => Ok(11806u32),
            BannerBlock {
                block_type: BannerBlockType::CyanBanner,
                rotation: 15i32,
            } => Ok(11807u32),
            BannerBlock {
                block_type: BannerBlockType::GrayBanner,
                rotation: 0i32,
            } => Ok(11760u32),
            BannerBlock {
                block_type: BannerBlockType::GrayBanner,
                rotation: 1i32,
            } => Ok(11761u32),
            BannerBlock {
                block_type: BannerBlockType::GrayBanner,
                rotation: 2i32,
            } => Ok(11762u32),
            BannerBlock {
                block_type: BannerBlockType::GrayBanner,
                rotation: 3i32,
            } => Ok(11763u32),
            BannerBlock {
                block_type: BannerBlockType::GrayBanner,
                rotation: 4i32,
            } => Ok(11764u32),
            BannerBlock {
                block_type: BannerBlockType::GrayBanner,
                rotation: 5i32,
            } => Ok(11765u32),
            BannerBlock {
                block_type: BannerBlockType::GrayBanner,
                rotation: 6i32,
            } => Ok(11766u32),
            BannerBlock {
                block_type: BannerBlockType::GrayBanner,
                rotation: 7i32,
            } => Ok(11767u32),
            BannerBlock {
                block_type: BannerBlockType::GrayBanner,
                rotation: 8i32,
            } => Ok(11768u32),
            BannerBlock {
                block_type: BannerBlockType::GrayBanner,
                rotation: 9i32,
            } => Ok(11769u32),
            BannerBlock {
                block_type: BannerBlockType::GrayBanner,
                rotation: 10i32,
            } => Ok(11770u32),
            BannerBlock {
                block_type: BannerBlockType::GrayBanner,
                rotation: 11i32,
            } => Ok(11771u32),
            BannerBlock {
                block_type: BannerBlockType::GrayBanner,
                rotation: 12i32,
            } => Ok(11772u32),
            BannerBlock {
                block_type: BannerBlockType::GrayBanner,
                rotation: 13i32,
            } => Ok(11773u32),
            BannerBlock {
                block_type: BannerBlockType::GrayBanner,
                rotation: 14i32,
            } => Ok(11774u32),
            BannerBlock {
                block_type: BannerBlockType::GrayBanner,
                rotation: 15i32,
            } => Ok(11775u32),
            BannerBlock {
                block_type: BannerBlockType::GreenBanner,
                rotation: 0i32,
            } => Ok(11856u32),
            BannerBlock {
                block_type: BannerBlockType::GreenBanner,
                rotation: 1i32,
            } => Ok(11857u32),
            BannerBlock {
                block_type: BannerBlockType::GreenBanner,
                rotation: 2i32,
            } => Ok(11858u32),
            BannerBlock {
                block_type: BannerBlockType::GreenBanner,
                rotation: 3i32,
            } => Ok(11859u32),
            BannerBlock {
                block_type: BannerBlockType::GreenBanner,
                rotation: 4i32,
            } => Ok(11860u32),
            BannerBlock {
                block_type: BannerBlockType::GreenBanner,
                rotation: 5i32,
            } => Ok(11861u32),
            BannerBlock {
                block_type: BannerBlockType::GreenBanner,
                rotation: 6i32,
            } => Ok(11862u32),
            BannerBlock {
                block_type: BannerBlockType::GreenBanner,
                rotation: 7i32,
            } => Ok(11863u32),
            BannerBlock {
                block_type: BannerBlockType::GreenBanner,
                rotation: 8i32,
            } => Ok(11864u32),
            BannerBlock {
                block_type: BannerBlockType::GreenBanner,
                rotation: 9i32,
            } => Ok(11865u32),
            BannerBlock {
                block_type: BannerBlockType::GreenBanner,
                rotation: 10i32,
            } => Ok(11866u32),
            BannerBlock {
                block_type: BannerBlockType::GreenBanner,
                rotation: 11i32,
            } => Ok(11867u32),
            BannerBlock {
                block_type: BannerBlockType::GreenBanner,
                rotation: 12i32,
            } => Ok(11868u32),
            BannerBlock {
                block_type: BannerBlockType::GreenBanner,
                rotation: 13i32,
            } => Ok(11869u32),
            BannerBlock {
                block_type: BannerBlockType::GreenBanner,
                rotation: 14i32,
            } => Ok(11870u32),
            BannerBlock {
                block_type: BannerBlockType::GreenBanner,
                rotation: 15i32,
            } => Ok(11871u32),
            BannerBlock {
                block_type: BannerBlockType::LightBlueBanner,
                rotation: 0i32,
            } => Ok(11696u32),
            BannerBlock {
                block_type: BannerBlockType::LightBlueBanner,
                rotation: 1i32,
            } => Ok(11697u32),
            BannerBlock {
                block_type: BannerBlockType::LightBlueBanner,
                rotation: 2i32,
            } => Ok(11698u32),
            BannerBlock {
                block_type: BannerBlockType::LightBlueBanner,
                rotation: 3i32,
            } => Ok(11699u32),
            BannerBlock {
                block_type: BannerBlockType::LightBlueBanner,
                rotation: 4i32,
            } => Ok(11700u32),
            BannerBlock {
                block_type: BannerBlockType::LightBlueBanner,
                rotation: 5i32,
            } => Ok(11701u32),
            BannerBlock {
                block_type: BannerBlockType::LightBlueBanner,
                rotation: 6i32,
            } => Ok(11702u32),
            BannerBlock {
                block_type: BannerBlockType::LightBlueBanner,
                rotation: 7i32,
            } => Ok(11703u32),
            BannerBlock {
                block_type: BannerBlockType::LightBlueBanner,
                rotation: 8i32,
            } => Ok(11704u32),
            BannerBlock {
                block_type: BannerBlockType::LightBlueBanner,
                rotation: 9i32,
            } => Ok(11705u32),
            BannerBlock {
                block_type: BannerBlockType::LightBlueBanner,
                rotation: 10i32,
            } => Ok(11706u32),
            BannerBlock {
                block_type: BannerBlockType::LightBlueBanner,
                rotation: 11i32,
            } => Ok(11707u32),
            BannerBlock {
                block_type: BannerBlockType::LightBlueBanner,
                rotation: 12i32,
            } => Ok(11708u32),
            BannerBlock {
                block_type: BannerBlockType::LightBlueBanner,
                rotation: 13i32,
            } => Ok(11709u32),
            BannerBlock {
                block_type: BannerBlockType::LightBlueBanner,
                rotation: 14i32,
            } => Ok(11710u32),
            BannerBlock {
                block_type: BannerBlockType::LightBlueBanner,
                rotation: 15i32,
            } => Ok(11711u32),
            BannerBlock {
                block_type: BannerBlockType::LightGrayBanner,
                rotation: 0i32,
            } => Ok(11776u32),
            BannerBlock {
                block_type: BannerBlockType::LightGrayBanner,
                rotation: 1i32,
            } => Ok(11777u32),
            BannerBlock {
                block_type: BannerBlockType::LightGrayBanner,
                rotation: 2i32,
            } => Ok(11778u32),
            BannerBlock {
                block_type: BannerBlockType::LightGrayBanner,
                rotation: 3i32,
            } => Ok(11779u32),
            BannerBlock {
                block_type: BannerBlockType::LightGrayBanner,
                rotation: 4i32,
            } => Ok(11780u32),
            BannerBlock {
                block_type: BannerBlockType::LightGrayBanner,
                rotation: 5i32,
            } => Ok(11781u32),
            BannerBlock {
                block_type: BannerBlockType::LightGrayBanner,
                rotation: 6i32,
            } => Ok(11782u32),
            BannerBlock {
                block_type: BannerBlockType::LightGrayBanner,
                rotation: 7i32,
            } => Ok(11783u32),
            BannerBlock {
                block_type: BannerBlockType::LightGrayBanner,
                rotation: 8i32,
            } => Ok(11784u32),
            BannerBlock {
                block_type: BannerBlockType::LightGrayBanner,
                rotation: 9i32,
            } => Ok(11785u32),
            BannerBlock {
                block_type: BannerBlockType::LightGrayBanner,
                rotation: 10i32,
            } => Ok(11786u32),
            BannerBlock {
                block_type: BannerBlockType::LightGrayBanner,
                rotation: 11i32,
            } => Ok(11787u32),
            BannerBlock {
                block_type: BannerBlockType::LightGrayBanner,
                rotation: 12i32,
            } => Ok(11788u32),
            BannerBlock {
                block_type: BannerBlockType::LightGrayBanner,
                rotation: 13i32,
            } => Ok(11789u32),
            BannerBlock {
                block_type: BannerBlockType::LightGrayBanner,
                rotation: 14i32,
            } => Ok(11790u32),
            BannerBlock {
                block_type: BannerBlockType::LightGrayBanner,
                rotation: 15i32,
            } => Ok(11791u32),
            BannerBlock {
                block_type: BannerBlockType::LimeBanner,
                rotation: 0i32,
            } => Ok(11728u32),
            BannerBlock {
                block_type: BannerBlockType::LimeBanner,
                rotation: 1i32,
            } => Ok(11729u32),
            BannerBlock {
                block_type: BannerBlockType::LimeBanner,
                rotation: 2i32,
            } => Ok(11730u32),
            BannerBlock {
                block_type: BannerBlockType::LimeBanner,
                rotation: 3i32,
            } => Ok(11731u32),
            BannerBlock {
                block_type: BannerBlockType::LimeBanner,
                rotation: 4i32,
            } => Ok(11732u32),
            BannerBlock {
                block_type: BannerBlockType::LimeBanner,
                rotation: 5i32,
            } => Ok(11733u32),
            BannerBlock {
                block_type: BannerBlockType::LimeBanner,
                rotation: 6i32,
            } => Ok(11734u32),
            BannerBlock {
                block_type: BannerBlockType::LimeBanner,
                rotation: 7i32,
            } => Ok(11735u32),
            BannerBlock {
                block_type: BannerBlockType::LimeBanner,
                rotation: 8i32,
            } => Ok(11736u32),
            BannerBlock {
                block_type: BannerBlockType::LimeBanner,
                rotation: 9i32,
            } => Ok(11737u32),
            BannerBlock {
                block_type: BannerBlockType::LimeBanner,
                rotation: 10i32,
            } => Ok(11738u32),
            BannerBlock {
                block_type: BannerBlockType::LimeBanner,
                rotation: 11i32,
            } => Ok(11739u32),
            BannerBlock {
                block_type: BannerBlockType::LimeBanner,
                rotation: 12i32,
            } => Ok(11740u32),
            BannerBlock {
                block_type: BannerBlockType::LimeBanner,
                rotation: 13i32,
            } => Ok(11741u32),
            BannerBlock {
                block_type: BannerBlockType::LimeBanner,
                rotation: 14i32,
            } => Ok(11742u32),
            BannerBlock {
                block_type: BannerBlockType::LimeBanner,
                rotation: 15i32,
            } => Ok(11743u32),
            BannerBlock {
                block_type: BannerBlockType::MagentaBanner,
                rotation: 0i32,
            } => Ok(11680u32),
            BannerBlock {
                block_type: BannerBlockType::MagentaBanner,
                rotation: 1i32,
            } => Ok(11681u32),
            BannerBlock {
                block_type: BannerBlockType::MagentaBanner,
                rotation: 2i32,
            } => Ok(11682u32),
            BannerBlock {
                block_type: BannerBlockType::MagentaBanner,
                rotation: 3i32,
            } => Ok(11683u32),
            BannerBlock {
                block_type: BannerBlockType::MagentaBanner,
                rotation: 4i32,
            } => Ok(11684u32),
            BannerBlock {
                block_type: BannerBlockType::MagentaBanner,
                rotation: 5i32,
            } => Ok(11685u32),
            BannerBlock {
                block_type: BannerBlockType::MagentaBanner,
                rotation: 6i32,
            } => Ok(11686u32),
            BannerBlock {
                block_type: BannerBlockType::MagentaBanner,
                rotation: 7i32,
            } => Ok(11687u32),
            BannerBlock {
                block_type: BannerBlockType::MagentaBanner,
                rotation: 8i32,
            } => Ok(11688u32),
            BannerBlock {
                block_type: BannerBlockType::MagentaBanner,
                rotation: 9i32,
            } => Ok(11689u32),
            BannerBlock {
                block_type: BannerBlockType::MagentaBanner,
                rotation: 10i32,
            } => Ok(11690u32),
            BannerBlock {
                block_type: BannerBlockType::MagentaBanner,
                rotation: 11i32,
            } => Ok(11691u32),
            BannerBlock {
                block_type: BannerBlockType::MagentaBanner,
                rotation: 12i32,
            } => Ok(11692u32),
            BannerBlock {
                block_type: BannerBlockType::MagentaBanner,
                rotation: 13i32,
            } => Ok(11693u32),
            BannerBlock {
                block_type: BannerBlockType::MagentaBanner,
                rotation: 14i32,
            } => Ok(11694u32),
            BannerBlock {
                block_type: BannerBlockType::MagentaBanner,
                rotation: 15i32,
            } => Ok(11695u32),
            BannerBlock {
                block_type: BannerBlockType::OrangeBanner,
                rotation: 0i32,
            } => Ok(11664u32),
            BannerBlock {
                block_type: BannerBlockType::OrangeBanner,
                rotation: 1i32,
            } => Ok(11665u32),
            BannerBlock {
                block_type: BannerBlockType::OrangeBanner,
                rotation: 2i32,
            } => Ok(11666u32),
            BannerBlock {
                block_type: BannerBlockType::OrangeBanner,
                rotation: 3i32,
            } => Ok(11667u32),
            BannerBlock {
                block_type: BannerBlockType::OrangeBanner,
                rotation: 4i32,
            } => Ok(11668u32),
            BannerBlock {
                block_type: BannerBlockType::OrangeBanner,
                rotation: 5i32,
            } => Ok(11669u32),
            BannerBlock {
                block_type: BannerBlockType::OrangeBanner,
                rotation: 6i32,
            } => Ok(11670u32),
            BannerBlock {
                block_type: BannerBlockType::OrangeBanner,
                rotation: 7i32,
            } => Ok(11671u32),
            BannerBlock {
                block_type: BannerBlockType::OrangeBanner,
                rotation: 8i32,
            } => Ok(11672u32),
            BannerBlock {
                block_type: BannerBlockType::OrangeBanner,
                rotation: 9i32,
            } => Ok(11673u32),
            BannerBlock {
                block_type: BannerBlockType::OrangeBanner,
                rotation: 10i32,
            } => Ok(11674u32),
            BannerBlock {
                block_type: BannerBlockType::OrangeBanner,
                rotation: 11i32,
            } => Ok(11675u32),
            BannerBlock {
                block_type: BannerBlockType::OrangeBanner,
                rotation: 12i32,
            } => Ok(11676u32),
            BannerBlock {
                block_type: BannerBlockType::OrangeBanner,
                rotation: 13i32,
            } => Ok(11677u32),
            BannerBlock {
                block_type: BannerBlockType::OrangeBanner,
                rotation: 14i32,
            } => Ok(11678u32),
            BannerBlock {
                block_type: BannerBlockType::OrangeBanner,
                rotation: 15i32,
            } => Ok(11679u32),
            BannerBlock {
                block_type: BannerBlockType::PinkBanner,
                rotation: 0i32,
            } => Ok(11744u32),
            BannerBlock {
                block_type: BannerBlockType::PinkBanner,
                rotation: 1i32,
            } => Ok(11745u32),
            BannerBlock {
                block_type: BannerBlockType::PinkBanner,
                rotation: 2i32,
            } => Ok(11746u32),
            BannerBlock {
                block_type: BannerBlockType::PinkBanner,
                rotation: 3i32,
            } => Ok(11747u32),
            BannerBlock {
                block_type: BannerBlockType::PinkBanner,
                rotation: 4i32,
            } => Ok(11748u32),
            BannerBlock {
                block_type: BannerBlockType::PinkBanner,
                rotation: 5i32,
            } => Ok(11749u32),
            BannerBlock {
                block_type: BannerBlockType::PinkBanner,
                rotation: 6i32,
            } => Ok(11750u32),
            BannerBlock {
                block_type: BannerBlockType::PinkBanner,
                rotation: 7i32,
            } => Ok(11751u32),
            BannerBlock {
                block_type: BannerBlockType::PinkBanner,
                rotation: 8i32,
            } => Ok(11752u32),
            BannerBlock {
                block_type: BannerBlockType::PinkBanner,
                rotation: 9i32,
            } => Ok(11753u32),
            BannerBlock {
                block_type: BannerBlockType::PinkBanner,
                rotation: 10i32,
            } => Ok(11754u32),
            BannerBlock {
                block_type: BannerBlockType::PinkBanner,
                rotation: 11i32,
            } => Ok(11755u32),
            BannerBlock {
                block_type: BannerBlockType::PinkBanner,
                rotation: 12i32,
            } => Ok(11756u32),
            BannerBlock {
                block_type: BannerBlockType::PinkBanner,
                rotation: 13i32,
            } => Ok(11757u32),
            BannerBlock {
                block_type: BannerBlockType::PinkBanner,
                rotation: 14i32,
            } => Ok(11758u32),
            BannerBlock {
                block_type: BannerBlockType::PinkBanner,
                rotation: 15i32,
            } => Ok(11759u32),
            BannerBlock {
                block_type: BannerBlockType::PurpleBanner,
                rotation: 0i32,
            } => Ok(11808u32),
            BannerBlock {
                block_type: BannerBlockType::PurpleBanner,
                rotation: 1i32,
            } => Ok(11809u32),
            BannerBlock {
                block_type: BannerBlockType::PurpleBanner,
                rotation: 2i32,
            } => Ok(11810u32),
            BannerBlock {
                block_type: BannerBlockType::PurpleBanner,
                rotation: 3i32,
            } => Ok(11811u32),
            BannerBlock {
                block_type: BannerBlockType::PurpleBanner,
                rotation: 4i32,
            } => Ok(11812u32),
            BannerBlock {
                block_type: BannerBlockType::PurpleBanner,
                rotation: 5i32,
            } => Ok(11813u32),
            BannerBlock {
                block_type: BannerBlockType::PurpleBanner,
                rotation: 6i32,
            } => Ok(11814u32),
            BannerBlock {
                block_type: BannerBlockType::PurpleBanner,
                rotation: 7i32,
            } => Ok(11815u32),
            BannerBlock {
                block_type: BannerBlockType::PurpleBanner,
                rotation: 8i32,
            } => Ok(11816u32),
            BannerBlock {
                block_type: BannerBlockType::PurpleBanner,
                rotation: 9i32,
            } => Ok(11817u32),
            BannerBlock {
                block_type: BannerBlockType::PurpleBanner,
                rotation: 10i32,
            } => Ok(11818u32),
            BannerBlock {
                block_type: BannerBlockType::PurpleBanner,
                rotation: 11i32,
            } => Ok(11819u32),
            BannerBlock {
                block_type: BannerBlockType::PurpleBanner,
                rotation: 12i32,
            } => Ok(11820u32),
            BannerBlock {
                block_type: BannerBlockType::PurpleBanner,
                rotation: 13i32,
            } => Ok(11821u32),
            BannerBlock {
                block_type: BannerBlockType::PurpleBanner,
                rotation: 14i32,
            } => Ok(11822u32),
            BannerBlock {
                block_type: BannerBlockType::PurpleBanner,
                rotation: 15i32,
            } => Ok(11823u32),
            BannerBlock {
                block_type: BannerBlockType::RedBanner,
                rotation: 0i32,
            } => Ok(11872u32),
            BannerBlock {
                block_type: BannerBlockType::RedBanner,
                rotation: 1i32,
            } => Ok(11873u32),
            BannerBlock {
                block_type: BannerBlockType::RedBanner,
                rotation: 2i32,
            } => Ok(11874u32),
            BannerBlock {
                block_type: BannerBlockType::RedBanner,
                rotation: 3i32,
            } => Ok(11875u32),
            BannerBlock {
                block_type: BannerBlockType::RedBanner,
                rotation: 4i32,
            } => Ok(11876u32),
            BannerBlock {
                block_type: BannerBlockType::RedBanner,
                rotation: 5i32,
            } => Ok(11877u32),
            BannerBlock {
                block_type: BannerBlockType::RedBanner,
                rotation: 6i32,
            } => Ok(11878u32),
            BannerBlock {
                block_type: BannerBlockType::RedBanner,
                rotation: 7i32,
            } => Ok(11879u32),
            BannerBlock {
                block_type: BannerBlockType::RedBanner,
                rotation: 8i32,
            } => Ok(11880u32),
            BannerBlock {
                block_type: BannerBlockType::RedBanner,
                rotation: 9i32,
            } => Ok(11881u32),
            BannerBlock {
                block_type: BannerBlockType::RedBanner,
                rotation: 10i32,
            } => Ok(11882u32),
            BannerBlock {
                block_type: BannerBlockType::RedBanner,
                rotation: 11i32,
            } => Ok(11883u32),
            BannerBlock {
                block_type: BannerBlockType::RedBanner,
                rotation: 12i32,
            } => Ok(11884u32),
            BannerBlock {
                block_type: BannerBlockType::RedBanner,
                rotation: 13i32,
            } => Ok(11885u32),
            BannerBlock {
                block_type: BannerBlockType::RedBanner,
                rotation: 14i32,
            } => Ok(11886u32),
            BannerBlock {
                block_type: BannerBlockType::RedBanner,
                rotation: 15i32,
            } => Ok(11887u32),
            BannerBlock {
                block_type: BannerBlockType::WhiteBanner,
                rotation: 0i32,
            } => Ok(11648u32),
            BannerBlock {
                block_type: BannerBlockType::WhiteBanner,
                rotation: 1i32,
            } => Ok(11649u32),
            BannerBlock {
                block_type: BannerBlockType::WhiteBanner,
                rotation: 2i32,
            } => Ok(11650u32),
            BannerBlock {
                block_type: BannerBlockType::WhiteBanner,
                rotation: 3i32,
            } => Ok(11651u32),
            BannerBlock {
                block_type: BannerBlockType::WhiteBanner,
                rotation: 4i32,
            } => Ok(11652u32),
            BannerBlock {
                block_type: BannerBlockType::WhiteBanner,
                rotation: 5i32,
            } => Ok(11653u32),
            BannerBlock {
                block_type: BannerBlockType::WhiteBanner,
                rotation: 6i32,
            } => Ok(11654u32),
            BannerBlock {
                block_type: BannerBlockType::WhiteBanner,
                rotation: 7i32,
            } => Ok(11655u32),
            BannerBlock {
                block_type: BannerBlockType::WhiteBanner,
                rotation: 8i32,
            } => Ok(11656u32),
            BannerBlock {
                block_type: BannerBlockType::WhiteBanner,
                rotation: 9i32,
            } => Ok(11657u32),
            BannerBlock {
                block_type: BannerBlockType::WhiteBanner,
                rotation: 10i32,
            } => Ok(11658u32),
            BannerBlock {
                block_type: BannerBlockType::WhiteBanner,
                rotation: 11i32,
            } => Ok(11659u32),
            BannerBlock {
                block_type: BannerBlockType::WhiteBanner,
                rotation: 12i32,
            } => Ok(11660u32),
            BannerBlock {
                block_type: BannerBlockType::WhiteBanner,
                rotation: 13i32,
            } => Ok(11661u32),
            BannerBlock {
                block_type: BannerBlockType::WhiteBanner,
                rotation: 14i32,
            } => Ok(11662u32),
            BannerBlock {
                block_type: BannerBlockType::WhiteBanner,
                rotation: 15i32,
            } => Ok(11663u32),
            BannerBlock {
                block_type: BannerBlockType::YellowBanner,
                rotation: 0i32,
            } => Ok(11712u32),
            BannerBlock {
                block_type: BannerBlockType::YellowBanner,
                rotation: 1i32,
            } => Ok(11713u32),
            BannerBlock {
                block_type: BannerBlockType::YellowBanner,
                rotation: 2i32,
            } => Ok(11714u32),
            BannerBlock {
                block_type: BannerBlockType::YellowBanner,
                rotation: 3i32,
            } => Ok(11715u32),
            BannerBlock {
                block_type: BannerBlockType::YellowBanner,
                rotation: 4i32,
            } => Ok(11716u32),
            BannerBlock {
                block_type: BannerBlockType::YellowBanner,
                rotation: 5i32,
            } => Ok(11717u32),
            BannerBlock {
                block_type: BannerBlockType::YellowBanner,
                rotation: 6i32,
            } => Ok(11718u32),
            BannerBlock {
                block_type: BannerBlockType::YellowBanner,
                rotation: 7i32,
            } => Ok(11719u32),
            BannerBlock {
                block_type: BannerBlockType::YellowBanner,
                rotation: 8i32,
            } => Ok(11720u32),
            BannerBlock {
                block_type: BannerBlockType::YellowBanner,
                rotation: 9i32,
            } => Ok(11721u32),
            BannerBlock {
                block_type: BannerBlockType::YellowBanner,
                rotation: 10i32,
            } => Ok(11722u32),
            BannerBlock {
                block_type: BannerBlockType::YellowBanner,
                rotation: 11i32,
            } => Ok(11723u32),
            BannerBlock {
                block_type: BannerBlockType::YellowBanner,
                rotation: 12i32,
            } => Ok(11724u32),
            BannerBlock {
                block_type: BannerBlockType::YellowBanner,
                rotation: 13i32,
            } => Ok(11725u32),
            BannerBlock {
                block_type: BannerBlockType::YellowBanner,
                rotation: 14i32,
            } => Ok(11726u32),
            BannerBlock {
                block_type: BannerBlockType::YellowBanner,
                rotation: 15i32,
            } => Ok(11727u32),
            _ => Err(()),
        }
    }
}

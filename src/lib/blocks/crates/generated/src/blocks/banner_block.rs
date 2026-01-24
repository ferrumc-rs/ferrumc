#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum BannerBlockType {
    BlackBanner,
    BlueBanner,
    BrownBanner,
    CyanBanner,
    GrayBanner,
    GreenBanner,
    LightBlueBanner,
    LightGrayBanner,
    LimeBanner,
    MagentaBanner,
    OrangeBanner,
    PinkBanner,
    PurpleBanner,
    RedBanner,
    WhiteBanner,
    YellowBanner,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BannerBlock {
    pub block_type: BannerBlockType,
    pub rotation: i32,
}

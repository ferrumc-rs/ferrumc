#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum CandleBlockType {
    BlackCandle,
    BlueCandle,
    BrownCandle,
    Candle,
    CyanCandle,
    GrayCandle,
    GreenCandle,
    LightBlueCandle,
    LightGrayCandle,
    LimeCandle,
    MagentaCandle,
    OrangeCandle,
    PinkCandle,
    PurpleCandle,
    RedCandle,
    WhiteCandle,
    YellowCandle,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CandleBlock {
    pub block_type: CandleBlockType,
    pub candles: i32,
    pub lit: bool,
    pub waterlogged: bool,
}

#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SeaPickleBlock {
    pub pickles: i32,
    pub waterlogged: bool,
}
impl TryInto<u32> for SeaPickleBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            SeaPickleBlock {
                pickles: 1i32,
                waterlogged: true,
            } => Ok(13956u32),
            SeaPickleBlock {
                pickles: 1i32,
                waterlogged: false,
            } => Ok(13957u32),
            SeaPickleBlock {
                pickles: 2i32,
                waterlogged: true,
            } => Ok(13958u32),
            SeaPickleBlock {
                pickles: 2i32,
                waterlogged: false,
            } => Ok(13959u32),
            SeaPickleBlock {
                pickles: 3i32,
                waterlogged: true,
            } => Ok(13960u32),
            SeaPickleBlock {
                pickles: 3i32,
                waterlogged: false,
            } => Ok(13961u32),
            SeaPickleBlock {
                pickles: 4i32,
                waterlogged: true,
            } => Ok(13962u32),
            SeaPickleBlock {
                pickles: 4i32,
                waterlogged: false,
            } => Ok(13963u32),
            _ => Err(()),
        }
    }
}

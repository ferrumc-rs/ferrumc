use crate::GlowBerriesPlantBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for GlowBerriesPlantBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            25849u32 => Ok(GlowBerriesPlantBlock { berries: true }),
            25850u32 => Ok(GlowBerriesPlantBlock { berries: false }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GlowBerriesPlantBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GlowBerriesPlantBlock { berries: true } => Ok(25849u32),
            GlowBerriesPlantBlock { berries: false } => Ok(25850u32),
            _ => Err(()),
        }
    }
}

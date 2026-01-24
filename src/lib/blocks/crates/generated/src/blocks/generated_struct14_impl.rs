use crate::GeneratedStruct14;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for GeneratedStruct14 {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            25849u32 => Ok(GeneratedStruct14 { berries: true }),
            25850u32 => Ok(GeneratedStruct14 { berries: false }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct14 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct14 { berries: true } => Ok(25849u32),
            GeneratedStruct14 { berries: false } => Ok(25850u32),
            _ => Err(()),
        }
    }
}

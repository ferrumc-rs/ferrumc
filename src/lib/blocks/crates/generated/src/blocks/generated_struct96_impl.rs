use crate::GeneratedStruct96;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for GeneratedStruct96 {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            27907u32 => Ok(GeneratedStruct96 { tip: true }),
            27908u32 => Ok(GeneratedStruct96 { tip: false }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct96 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct96 { tip: true } => Ok(27907u32),
            GeneratedStruct96 { tip: false } => Ok(27908u32),
            _ => Err(()),
        }
    }
}

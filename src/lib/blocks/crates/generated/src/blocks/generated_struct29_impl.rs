use crate::GeneratedStruct29;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for GeneratedStruct29 {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            13983u32 => Ok(GeneratedStruct29 { drag: true }),
            13984u32 => Ok(GeneratedStruct29 { drag: false }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct29 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct29 { drag: true } => Ok(13983u32),
            GeneratedStruct29 { drag: false } => Ok(13984u32),
            _ => Err(()),
        }
    }
}

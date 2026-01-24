use crate::GeneratedStruct70;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for GeneratedStruct70 {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            5994u32 => Ok(GeneratedStruct70 { has_record: true }),
            5995u32 => Ok(GeneratedStruct70 { has_record: false }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct70 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct70 { has_record: true } => Ok(5994u32),
            GeneratedStruct70 { has_record: false } => Ok(5995u32),
            _ => Err(()),
        }
    }
}

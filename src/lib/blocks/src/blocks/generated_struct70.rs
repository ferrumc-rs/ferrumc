#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub struct GeneratedStruct70 {
    pub has_record: bool,
}
impl TryFrom<u32> for GeneratedStruct70 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
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

#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GeneratedStruct70 {
    pub has_record: bool,
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

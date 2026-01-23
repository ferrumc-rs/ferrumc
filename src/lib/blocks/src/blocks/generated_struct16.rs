#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub struct GeneratedStruct16 {
    pub bloom: bool,
}
impl TryFrom<u32> for GeneratedStruct16 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            23956u32 => Ok(GeneratedStruct16 { bloom: true }),
            23957u32 => Ok(GeneratedStruct16 { bloom: false }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct16 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct16 { bloom: true } => Ok(23956u32),
            GeneratedStruct16 { bloom: false } => Ok(23957u32),
            _ => Err(()),
        }
    }
}

#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub struct GeneratedStruct29 {
    pub drag: bool,
}
impl TryFrom<u32> for GeneratedStruct29 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
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

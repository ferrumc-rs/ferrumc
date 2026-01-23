#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub struct GeneratedStruct98 {
    pub unstable: bool,
}
impl TryFrom<u32> for GeneratedStruct98 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            2140u32 => Ok(GeneratedStruct98 { unstable: true }),
            2141u32 => Ok(GeneratedStruct98 { unstable: false }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct98 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct98 { unstable: true } => Ok(2140u32),
            GeneratedStruct98 { unstable: false } => Ok(2141u32),
            _ => Err(()),
        }
    }
}

#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GeneratedStruct98 {
    pub unstable: bool,
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

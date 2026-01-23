#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GeneratedStruct29 {
    pub drag: bool,
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

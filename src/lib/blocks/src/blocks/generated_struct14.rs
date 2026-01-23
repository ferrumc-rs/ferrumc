#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GeneratedStruct14 {
    pub berries: bool,
}
impl TryInto<u32> for GeneratedStruct14 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct14 { berries: true } => Ok(25849u32),
            GeneratedStruct14 { berries: false } => Ok(25850u32),
            _ => Err(()),
        }
    }
}

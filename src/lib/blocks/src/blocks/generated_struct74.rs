#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GeneratedStruct74 {
    pub layers: i32,
}
impl TryInto<u32> for GeneratedStruct74 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct74 { layers: 1i32 } => Ok(5950u32),
            GeneratedStruct74 { layers: 2i32 } => Ok(5951u32),
            GeneratedStruct74 { layers: 3i32 } => Ok(5952u32),
            GeneratedStruct74 { layers: 4i32 } => Ok(5953u32),
            GeneratedStruct74 { layers: 5i32 } => Ok(5954u32),
            GeneratedStruct74 { layers: 6i32 } => Ok(5955u32),
            GeneratedStruct74 { layers: 7i32 } => Ok(5956u32),
            GeneratedStruct74 { layers: 8i32 } => Ok(5957u32),
            _ => Err(()),
        }
    }
}

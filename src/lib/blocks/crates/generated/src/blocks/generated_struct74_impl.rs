use crate::GeneratedStruct74;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for GeneratedStruct74 {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            5950u32 => Ok(GeneratedStruct74 { layers: 1i32 }),
            5951u32 => Ok(GeneratedStruct74 { layers: 2i32 }),
            5952u32 => Ok(GeneratedStruct74 { layers: 3i32 }),
            5953u32 => Ok(GeneratedStruct74 { layers: 4i32 }),
            5954u32 => Ok(GeneratedStruct74 { layers: 5i32 }),
            5955u32 => Ok(GeneratedStruct74 { layers: 6i32 }),
            5956u32 => Ok(GeneratedStruct74 { layers: 7i32 }),
            5957u32 => Ok(GeneratedStruct74 { layers: 8i32 }),
            _ => Err(()),
        }
    }
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

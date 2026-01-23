#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub struct GeneratedStruct15 {
    pub bites: i32,
}
impl TryFrom<u32> for GeneratedStruct15 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            6053u32 => Ok(GeneratedStruct15 { bites: 0i32 }),
            6054u32 => Ok(GeneratedStruct15 { bites: 1i32 }),
            6055u32 => Ok(GeneratedStruct15 { bites: 2i32 }),
            6056u32 => Ok(GeneratedStruct15 { bites: 3i32 }),
            6057u32 => Ok(GeneratedStruct15 { bites: 4i32 }),
            6058u32 => Ok(GeneratedStruct15 { bites: 5i32 }),
            6059u32 => Ok(GeneratedStruct15 { bites: 6i32 }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct15 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct15 { bites: 0i32 } => Ok(6053u32),
            GeneratedStruct15 { bites: 1i32 } => Ok(6054u32),
            GeneratedStruct15 { bites: 2i32 } => Ok(6055u32),
            GeneratedStruct15 { bites: 3i32 } => Ok(6056u32),
            GeneratedStruct15 { bites: 4i32 } => Ok(6057u32),
            GeneratedStruct15 { bites: 5i32 } => Ok(6058u32),
            GeneratedStruct15 { bites: 6i32 } => Ok(6059u32),
            _ => Err(()),
        }
    }
}

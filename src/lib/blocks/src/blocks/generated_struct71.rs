#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub struct GeneratedStruct71 {
    pub hatch: i32,
}
impl TryFrom<u32> for GeneratedStruct71 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            13823u32 => Ok(GeneratedStruct71 { hatch: 0i32 }),
            13824u32 => Ok(GeneratedStruct71 { hatch: 1i32 }),
            13825u32 => Ok(GeneratedStruct71 { hatch: 2i32 }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct71 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct71 { hatch: 0i32 } => Ok(13823u32),
            GeneratedStruct71 { hatch: 1i32 } => Ok(13824u32),
            GeneratedStruct71 { hatch: 2i32 } => Ok(13825u32),
            _ => Err(()),
        }
    }
}

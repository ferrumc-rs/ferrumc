#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GeneratedStruct69 {
    pub has_bottle_0: bool,
    pub has_bottle_1: bool,
    pub has_bottle_2: bool,
}
impl TryInto<u32> for GeneratedStruct69 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct69 {
                has_bottle_0: true,
                has_bottle_1: true,
                has_bottle_2: true,
            } => Ok(8174u32),
            GeneratedStruct69 {
                has_bottle_0: true,
                has_bottle_1: true,
                has_bottle_2: false,
            } => Ok(8175u32),
            GeneratedStruct69 {
                has_bottle_0: true,
                has_bottle_1: false,
                has_bottle_2: true,
            } => Ok(8176u32),
            GeneratedStruct69 {
                has_bottle_0: true,
                has_bottle_1: false,
                has_bottle_2: false,
            } => Ok(8177u32),
            GeneratedStruct69 {
                has_bottle_0: false,
                has_bottle_1: true,
                has_bottle_2: true,
            } => Ok(8178u32),
            GeneratedStruct69 {
                has_bottle_0: false,
                has_bottle_1: true,
                has_bottle_2: false,
            } => Ok(8179u32),
            GeneratedStruct69 {
                has_bottle_0: false,
                has_bottle_1: false,
                has_bottle_2: true,
            } => Ok(8180u32),
            GeneratedStruct69 {
                has_bottle_0: false,
                has_bottle_1: false,
                has_bottle_2: false,
            } => Ok(8181u32),
            _ => Err(()),
        }
    }
}

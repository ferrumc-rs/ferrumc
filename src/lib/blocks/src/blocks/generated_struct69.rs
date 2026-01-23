#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub struct GeneratedStruct69 {
    pub has_bottle_0: bool,
    pub has_bottle_1: bool,
    pub has_bottle_2: bool,
}
impl TryFrom<u32> for GeneratedStruct69 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            8174u32 => Ok(GeneratedStruct69 {
                has_bottle_0: true,
                has_bottle_1: true,
                has_bottle_2: true,
            }),
            8175u32 => Ok(GeneratedStruct69 {
                has_bottle_0: true,
                has_bottle_2: false,
                has_bottle_1: true,
            }),
            8176u32 => Ok(GeneratedStruct69 {
                has_bottle_1: false,
                has_bottle_0: true,
                has_bottle_2: true,
            }),
            8177u32 => Ok(GeneratedStruct69 {
                has_bottle_1: false,
                has_bottle_2: false,
                has_bottle_0: true,
            }),
            8178u32 => Ok(GeneratedStruct69 {
                has_bottle_0: false,
                has_bottle_2: true,
                has_bottle_1: true,
            }),
            8179u32 => Ok(GeneratedStruct69 {
                has_bottle_0: false,
                has_bottle_1: true,
                has_bottle_2: false,
            }),
            8180u32 => Ok(GeneratedStruct69 {
                has_bottle_1: false,
                has_bottle_0: false,
                has_bottle_2: true,
            }),
            8181u32 => Ok(GeneratedStruct69 {
                has_bottle_1: false,
                has_bottle_0: false,
                has_bottle_2: false,
            }),
            _ => Err(()),
        }
    }
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
                has_bottle_2: false,
                has_bottle_1: true,
            } => Ok(8175u32),
            GeneratedStruct69 {
                has_bottle_1: false,
                has_bottle_0: true,
                has_bottle_2: true,
            } => Ok(8176u32),
            GeneratedStruct69 {
                has_bottle_1: false,
                has_bottle_2: false,
                has_bottle_0: true,
            } => Ok(8177u32),
            GeneratedStruct69 {
                has_bottle_0: false,
                has_bottle_2: true,
                has_bottle_1: true,
            } => Ok(8178u32),
            GeneratedStruct69 {
                has_bottle_0: false,
                has_bottle_1: true,
                has_bottle_2: false,
            } => Ok(8179u32),
            GeneratedStruct69 {
                has_bottle_1: false,
                has_bottle_0: false,
                has_bottle_2: true,
            } => Ok(8180u32),
            GeneratedStruct69 {
                has_bottle_1: false,
                has_bottle_0: false,
                has_bottle_2: false,
            } => Ok(8181u32),
            _ => Err(()),
        }
    }
}

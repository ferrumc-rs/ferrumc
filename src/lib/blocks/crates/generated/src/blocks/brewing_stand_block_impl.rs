use crate::BrewingStandBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for BrewingStandBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            8174u32 => Ok(BrewingStandBlock {
                has_bottle_0: true,
                has_bottle_1: true,
                has_bottle_2: true,
            }),
            8175u32 => Ok(BrewingStandBlock {
                has_bottle_0: true,
                has_bottle_1: true,
                has_bottle_2: false,
            }),
            8176u32 => Ok(BrewingStandBlock {
                has_bottle_0: true,
                has_bottle_1: false,
                has_bottle_2: true,
            }),
            8177u32 => Ok(BrewingStandBlock {
                has_bottle_0: true,
                has_bottle_1: false,
                has_bottle_2: false,
            }),
            8178u32 => Ok(BrewingStandBlock {
                has_bottle_0: false,
                has_bottle_1: true,
                has_bottle_2: true,
            }),
            8179u32 => Ok(BrewingStandBlock {
                has_bottle_0: false,
                has_bottle_1: true,
                has_bottle_2: false,
            }),
            8180u32 => Ok(BrewingStandBlock {
                has_bottle_0: false,
                has_bottle_1: false,
                has_bottle_2: true,
            }),
            8181u32 => Ok(BrewingStandBlock {
                has_bottle_0: false,
                has_bottle_1: false,
                has_bottle_2: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for BrewingStandBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            BrewingStandBlock {
                has_bottle_0: true,
                has_bottle_1: true,
                has_bottle_2: true,
            } => Ok(8174u32),
            BrewingStandBlock {
                has_bottle_0: true,
                has_bottle_1: true,
                has_bottle_2: false,
            } => Ok(8175u32),
            BrewingStandBlock {
                has_bottle_0: true,
                has_bottle_1: false,
                has_bottle_2: true,
            } => Ok(8176u32),
            BrewingStandBlock {
                has_bottle_0: true,
                has_bottle_1: false,
                has_bottle_2: false,
            } => Ok(8177u32),
            BrewingStandBlock {
                has_bottle_0: false,
                has_bottle_1: true,
                has_bottle_2: true,
            } => Ok(8178u32),
            BrewingStandBlock {
                has_bottle_0: false,
                has_bottle_1: true,
                has_bottle_2: false,
            } => Ok(8179u32),
            BrewingStandBlock {
                has_bottle_0: false,
                has_bottle_1: false,
                has_bottle_2: true,
            } => Ok(8180u32),
            BrewingStandBlock {
                has_bottle_0: false,
                has_bottle_1: false,
                has_bottle_2: false,
            } => Ok(8181u32),
            _ => Err(()),
        }
    }
}

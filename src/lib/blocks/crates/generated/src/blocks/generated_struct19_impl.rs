use crate::GeneratedStruct19;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for GeneratedStruct19 {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            23958u32 => Ok(GeneratedStruct19 {
                can_summon: true,
                shrieking: true,
                waterlogged: true,
            }),
            23959u32 => Ok(GeneratedStruct19 {
                can_summon: true,
                shrieking: true,
                waterlogged: false,
            }),
            23960u32 => Ok(GeneratedStruct19 {
                can_summon: true,
                shrieking: false,
                waterlogged: true,
            }),
            23961u32 => Ok(GeneratedStruct19 {
                can_summon: true,
                shrieking: false,
                waterlogged: false,
            }),
            23962u32 => Ok(GeneratedStruct19 {
                can_summon: false,
                shrieking: true,
                waterlogged: true,
            }),
            23963u32 => Ok(GeneratedStruct19 {
                can_summon: false,
                shrieking: true,
                waterlogged: false,
            }),
            23964u32 => Ok(GeneratedStruct19 {
                can_summon: false,
                shrieking: false,
                waterlogged: true,
            }),
            23965u32 => Ok(GeneratedStruct19 {
                can_summon: false,
                shrieking: false,
                waterlogged: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct19 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct19 {
                can_summon: true,
                shrieking: true,
                waterlogged: true,
            } => Ok(23958u32),
            GeneratedStruct19 {
                can_summon: true,
                shrieking: true,
                waterlogged: false,
            } => Ok(23959u32),
            GeneratedStruct19 {
                can_summon: true,
                shrieking: false,
                waterlogged: true,
            } => Ok(23960u32),
            GeneratedStruct19 {
                can_summon: true,
                shrieking: false,
                waterlogged: false,
            } => Ok(23961u32),
            GeneratedStruct19 {
                can_summon: false,
                shrieking: true,
                waterlogged: true,
            } => Ok(23962u32),
            GeneratedStruct19 {
                can_summon: false,
                shrieking: true,
                waterlogged: false,
            } => Ok(23963u32),
            GeneratedStruct19 {
                can_summon: false,
                shrieking: false,
                waterlogged: true,
            } => Ok(23964u32),
            GeneratedStruct19 {
                can_summon: false,
                shrieking: false,
                waterlogged: false,
            } => Ok(23965u32),
            _ => Err(()),
        }
    }
}

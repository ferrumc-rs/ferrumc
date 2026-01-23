#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub struct GeneratedStruct19 {
    pub can_summon: bool,
    pub shrieking: bool,
    pub waterlogged: bool,
}
impl TryFrom<u32> for GeneratedStruct19 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            23958u32 => Ok(GeneratedStruct19 {
                can_summon: true,
                waterlogged: true,
                shrieking: true,
            }),
            23959u32 => Ok(GeneratedStruct19 {
                can_summon: true,
                waterlogged: false,
                shrieking: true,
            }),
            23960u32 => Ok(GeneratedStruct19 {
                shrieking: false,
                waterlogged: true,
                can_summon: true,
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
                waterlogged: false,
                can_summon: false,
                shrieking: true,
            }),
            23964u32 => Ok(GeneratedStruct19 {
                waterlogged: true,
                can_summon: false,
                shrieking: false,
            }),
            23965u32 => Ok(GeneratedStruct19 {
                waterlogged: false,
                shrieking: false,
                can_summon: false,
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
                waterlogged: true,
                shrieking: true,
            } => Ok(23958u32),
            GeneratedStruct19 {
                can_summon: true,
                waterlogged: false,
                shrieking: true,
            } => Ok(23959u32),
            GeneratedStruct19 {
                shrieking: false,
                waterlogged: true,
                can_summon: true,
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
                waterlogged: false,
                can_summon: false,
                shrieking: true,
            } => Ok(23963u32),
            GeneratedStruct19 {
                waterlogged: true,
                can_summon: false,
                shrieking: false,
            } => Ok(23964u32),
            GeneratedStruct19 {
                waterlogged: false,
                shrieking: false,
                can_summon: false,
            } => Ok(23965u32),
            _ => Err(()),
        }
    }
}

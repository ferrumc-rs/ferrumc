#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub struct GeneratedStruct84 {
    pub pickles: i32,
    pub waterlogged: bool,
}
impl TryFrom<u32> for GeneratedStruct84 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            13956u32 => Ok(GeneratedStruct84 {
                pickles: 1i32,
                waterlogged: true,
            }),
            13957u32 => Ok(GeneratedStruct84 {
                pickles: 1i32,
                waterlogged: false,
            }),
            13958u32 => Ok(GeneratedStruct84 {
                waterlogged: true,
                pickles: 2i32,
            }),
            13959u32 => Ok(GeneratedStruct84 {
                waterlogged: false,
                pickles: 2i32,
            }),
            13960u32 => Ok(GeneratedStruct84 {
                waterlogged: true,
                pickles: 3i32,
            }),
            13961u32 => Ok(GeneratedStruct84 {
                pickles: 3i32,
                waterlogged: false,
            }),
            13962u32 => Ok(GeneratedStruct84 {
                pickles: 4i32,
                waterlogged: true,
            }),
            13963u32 => Ok(GeneratedStruct84 {
                pickles: 4i32,
                waterlogged: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct84 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct84 {
                pickles: 1i32,
                waterlogged: true,
            } => Ok(13956u32),
            GeneratedStruct84 {
                pickles: 1i32,
                waterlogged: false,
            } => Ok(13957u32),
            GeneratedStruct84 {
                waterlogged: true,
                pickles: 2i32,
            } => Ok(13958u32),
            GeneratedStruct84 {
                waterlogged: false,
                pickles: 2i32,
            } => Ok(13959u32),
            GeneratedStruct84 {
                waterlogged: true,
                pickles: 3i32,
            } => Ok(13960u32),
            GeneratedStruct84 {
                pickles: 3i32,
                waterlogged: false,
            } => Ok(13961u32),
            GeneratedStruct84 {
                pickles: 4i32,
                waterlogged: true,
            } => Ok(13962u32),
            GeneratedStruct84 {
                pickles: 4i32,
                waterlogged: false,
            } => Ok(13963u32),
            _ => Err(()),
        }
    }
}

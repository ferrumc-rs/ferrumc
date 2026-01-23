#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub struct GeneratedStruct13 {
    pub axis: Axis,
    pub waterlogged: bool,
}
impl TryFrom<u32> for GeneratedStruct13 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            7016u32 => Ok(GeneratedStruct13 {
                axis: Axis::X,
                waterlogged: true,
            }),
            7017u32 => Ok(GeneratedStruct13 {
                waterlogged: false,
                axis: Axis::X,
            }),
            7018u32 => Ok(GeneratedStruct13 {
                axis: Axis::Y,
                waterlogged: true,
            }),
            7019u32 => Ok(GeneratedStruct13 {
                waterlogged: false,
                axis: Axis::Y,
            }),
            7020u32 => Ok(GeneratedStruct13 {
                waterlogged: true,
                axis: Axis::Z,
            }),
            7021u32 => Ok(GeneratedStruct13 {
                axis: Axis::Z,
                waterlogged: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct13 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct13 {
                axis: Axis::X,
                waterlogged: true,
            } => Ok(7016u32),
            GeneratedStruct13 {
                waterlogged: false,
                axis: Axis::X,
            } => Ok(7017u32),
            GeneratedStruct13 {
                axis: Axis::Y,
                waterlogged: true,
            } => Ok(7018u32),
            GeneratedStruct13 {
                waterlogged: false,
                axis: Axis::Y,
            } => Ok(7019u32),
            GeneratedStruct13 {
                waterlogged: true,
                axis: Axis::Z,
            } => Ok(7020u32),
            GeneratedStruct13 {
                axis: Axis::Z,
                waterlogged: false,
            } => Ok(7021u32),
            _ => Err(()),
        }
    }
}

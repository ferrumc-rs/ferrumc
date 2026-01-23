#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GeneratedStruct13 {
    pub axis: Axis,
    pub waterlogged: bool,
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
                axis: Axis::X,
                waterlogged: false,
            } => Ok(7017u32),
            GeneratedStruct13 {
                axis: Axis::Y,
                waterlogged: true,
            } => Ok(7018u32),
            GeneratedStruct13 {
                axis: Axis::Y,
                waterlogged: false,
            } => Ok(7019u32),
            GeneratedStruct13 {
                axis: Axis::Z,
                waterlogged: true,
            } => Ok(7020u32),
            GeneratedStruct13 {
                axis: Axis::Z,
                waterlogged: false,
            } => Ok(7021u32),
            _ => Err(()),
        }
    }
}

#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GeneratedStruct38 {
    pub eye: bool,
    pub facing: Direction,
}
impl TryInto<u32> for GeneratedStruct38 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct38 {
                eye: true,
                facing: Direction::North,
            } => Ok(8191u32),
            GeneratedStruct38 {
                eye: true,
                facing: Direction::South,
            } => Ok(8192u32),
            GeneratedStruct38 {
                eye: true,
                facing: Direction::West,
            } => Ok(8193u32),
            GeneratedStruct38 {
                eye: true,
                facing: Direction::East,
            } => Ok(8194u32),
            GeneratedStruct38 {
                eye: false,
                facing: Direction::North,
            } => Ok(8195u32),
            GeneratedStruct38 {
                eye: false,
                facing: Direction::South,
            } => Ok(8196u32),
            GeneratedStruct38 {
                eye: false,
                facing: Direction::West,
            } => Ok(8197u32),
            GeneratedStruct38 {
                eye: false,
                facing: Direction::East,
            } => Ok(8198u32),
            _ => Err(()),
        }
    }
}

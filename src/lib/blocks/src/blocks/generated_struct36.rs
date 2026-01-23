#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GeneratedStruct36 {
    pub enabled: bool,
    pub facing: Direction,
}
impl TryInto<u32> for GeneratedStruct36 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct36 {
                enabled: true,
                facing: Direction::Down,
            } => Ok(10034u32),
            GeneratedStruct36 {
                enabled: true,
                facing: Direction::North,
            } => Ok(10035u32),
            GeneratedStruct36 {
                enabled: true,
                facing: Direction::South,
            } => Ok(10036u32),
            GeneratedStruct36 {
                enabled: true,
                facing: Direction::West,
            } => Ok(10037u32),
            GeneratedStruct36 {
                enabled: true,
                facing: Direction::East,
            } => Ok(10038u32),
            GeneratedStruct36 {
                enabled: false,
                facing: Direction::Down,
            } => Ok(10039u32),
            GeneratedStruct36 {
                enabled: false,
                facing: Direction::North,
            } => Ok(10040u32),
            GeneratedStruct36 {
                enabled: false,
                facing: Direction::South,
            } => Ok(10041u32),
            GeneratedStruct36 {
                enabled: false,
                facing: Direction::West,
            } => Ok(10042u32),
            GeneratedStruct36 {
                enabled: false,
                facing: Direction::East,
            } => Ok(10043u32),
            _ => Err(()),
        }
    }
}

#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GeneratedStruct8 {
    pub attached: bool,
    pub facing: Direction,
    pub powered: bool,
}
impl TryInto<u32> for GeneratedStruct8 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct8 {
                attached: true,
                facing: Direction::North,
                powered: true,
            } => Ok(8305u32),
            GeneratedStruct8 {
                attached: true,
                facing: Direction::North,
                powered: false,
            } => Ok(8306u32),
            GeneratedStruct8 {
                attached: true,
                facing: Direction::South,
                powered: true,
            } => Ok(8307u32),
            GeneratedStruct8 {
                attached: true,
                facing: Direction::South,
                powered: false,
            } => Ok(8308u32),
            GeneratedStruct8 {
                attached: true,
                facing: Direction::West,
                powered: true,
            } => Ok(8309u32),
            GeneratedStruct8 {
                attached: true,
                facing: Direction::West,
                powered: false,
            } => Ok(8310u32),
            GeneratedStruct8 {
                attached: true,
                facing: Direction::East,
                powered: true,
            } => Ok(8311u32),
            GeneratedStruct8 {
                attached: true,
                facing: Direction::East,
                powered: false,
            } => Ok(8312u32),
            GeneratedStruct8 {
                attached: false,
                facing: Direction::North,
                powered: true,
            } => Ok(8313u32),
            GeneratedStruct8 {
                attached: false,
                facing: Direction::North,
                powered: false,
            } => Ok(8314u32),
            GeneratedStruct8 {
                attached: false,
                facing: Direction::South,
                powered: true,
            } => Ok(8315u32),
            GeneratedStruct8 {
                attached: false,
                facing: Direction::South,
                powered: false,
            } => Ok(8316u32),
            GeneratedStruct8 {
                attached: false,
                facing: Direction::West,
                powered: true,
            } => Ok(8317u32),
            GeneratedStruct8 {
                attached: false,
                facing: Direction::West,
                powered: false,
            } => Ok(8318u32),
            GeneratedStruct8 {
                attached: false,
                facing: Direction::East,
                powered: true,
            } => Ok(8319u32),
            GeneratedStruct8 {
                attached: false,
                facing: Direction::East,
                powered: false,
            } => Ok(8320u32),
            _ => Err(()),
        }
    }
}

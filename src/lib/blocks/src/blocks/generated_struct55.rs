#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GeneratedStruct55 {
    pub facing: Direction,
    pub open: bool,
}
impl TryInto<u32> for GeneratedStruct55 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct55 {
                facing: Direction::North,
                open: true,
            } => Ok(19431u32),
            GeneratedStruct55 {
                facing: Direction::North,
                open: false,
            } => Ok(19432u32),
            GeneratedStruct55 {
                facing: Direction::East,
                open: true,
            } => Ok(19433u32),
            GeneratedStruct55 {
                facing: Direction::East,
                open: false,
            } => Ok(19434u32),
            GeneratedStruct55 {
                facing: Direction::South,
                open: true,
            } => Ok(19435u32),
            GeneratedStruct55 {
                facing: Direction::South,
                open: false,
            } => Ok(19436u32),
            GeneratedStruct55 {
                facing: Direction::West,
                open: true,
            } => Ok(19437u32),
            GeneratedStruct55 {
                facing: Direction::West,
                open: false,
            } => Ok(19438u32),
            GeneratedStruct55 {
                facing: Direction::Up,
                open: true,
            } => Ok(19439u32),
            GeneratedStruct55 {
                facing: Direction::Up,
                open: false,
            } => Ok(19440u32),
            GeneratedStruct55 {
                facing: Direction::Down,
                open: true,
            } => Ok(19441u32),
            GeneratedStruct55 {
                facing: Direction::Down,
                open: false,
            } => Ok(19442u32),
            _ => Err(()),
        }
    }
}

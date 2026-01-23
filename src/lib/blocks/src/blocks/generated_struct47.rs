#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GeneratedStruct47 {
    pub facing: Direction,
    pub has_book: bool,
    pub powered: bool,
}
impl TryInto<u32> for GeneratedStruct47 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct47 {
                facing: Direction::North,
                has_book: true,
                powered: true,
            } => Ok(19473u32),
            GeneratedStruct47 {
                facing: Direction::North,
                has_book: true,
                powered: false,
            } => Ok(19474u32),
            GeneratedStruct47 {
                facing: Direction::North,
                has_book: false,
                powered: true,
            } => Ok(19475u32),
            GeneratedStruct47 {
                facing: Direction::North,
                has_book: false,
                powered: false,
            } => Ok(19476u32),
            GeneratedStruct47 {
                facing: Direction::South,
                has_book: true,
                powered: true,
            } => Ok(19477u32),
            GeneratedStruct47 {
                facing: Direction::South,
                has_book: true,
                powered: false,
            } => Ok(19478u32),
            GeneratedStruct47 {
                facing: Direction::South,
                has_book: false,
                powered: true,
            } => Ok(19479u32),
            GeneratedStruct47 {
                facing: Direction::South,
                has_book: false,
                powered: false,
            } => Ok(19480u32),
            GeneratedStruct47 {
                facing: Direction::West,
                has_book: true,
                powered: true,
            } => Ok(19481u32),
            GeneratedStruct47 {
                facing: Direction::West,
                has_book: true,
                powered: false,
            } => Ok(19482u32),
            GeneratedStruct47 {
                facing: Direction::West,
                has_book: false,
                powered: true,
            } => Ok(19483u32),
            GeneratedStruct47 {
                facing: Direction::West,
                has_book: false,
                powered: false,
            } => Ok(19484u32),
            GeneratedStruct47 {
                facing: Direction::East,
                has_book: true,
                powered: true,
            } => Ok(19485u32),
            GeneratedStruct47 {
                facing: Direction::East,
                has_book: true,
                powered: false,
            } => Ok(19486u32),
            GeneratedStruct47 {
                facing: Direction::East,
                has_book: false,
                powered: true,
            } => Ok(19487u32),
            GeneratedStruct47 {
                facing: Direction::East,
                has_book: false,
                powered: false,
            } => Ok(19488u32),
            _ => Err(()),
        }
    }
}

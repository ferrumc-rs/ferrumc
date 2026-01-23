#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub struct GeneratedStruct55 {
    pub facing: Direction,
    pub open: bool,
}
impl TryFrom<u32> for GeneratedStruct55 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            19431u32 => Ok(GeneratedStruct55 {
                open: true,
                facing: Direction::North,
            }),
            19432u32 => Ok(GeneratedStruct55 {
                open: false,
                facing: Direction::North,
            }),
            19433u32 => Ok(GeneratedStruct55 {
                facing: Direction::East,
                open: true,
            }),
            19434u32 => Ok(GeneratedStruct55 {
                open: false,
                facing: Direction::East,
            }),
            19435u32 => Ok(GeneratedStruct55 {
                facing: Direction::South,
                open: true,
            }),
            19436u32 => Ok(GeneratedStruct55 {
                facing: Direction::South,
                open: false,
            }),
            19437u32 => Ok(GeneratedStruct55 {
                facing: Direction::West,
                open: true,
            }),
            19438u32 => Ok(GeneratedStruct55 {
                open: false,
                facing: Direction::West,
            }),
            19439u32 => Ok(GeneratedStruct55 {
                open: true,
                facing: Direction::Up,
            }),
            19440u32 => Ok(GeneratedStruct55 {
                open: false,
                facing: Direction::Up,
            }),
            19441u32 => Ok(GeneratedStruct55 {
                facing: Direction::Down,
                open: true,
            }),
            19442u32 => Ok(GeneratedStruct55 {
                facing: Direction::Down,
                open: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct55 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct55 {
                open: true,
                facing: Direction::North,
            } => Ok(19431u32),
            GeneratedStruct55 {
                open: false,
                facing: Direction::North,
            } => Ok(19432u32),
            GeneratedStruct55 {
                facing: Direction::East,
                open: true,
            } => Ok(19433u32),
            GeneratedStruct55 {
                open: false,
                facing: Direction::East,
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
                open: false,
                facing: Direction::West,
            } => Ok(19438u32),
            GeneratedStruct55 {
                open: true,
                facing: Direction::Up,
            } => Ok(19439u32),
            GeneratedStruct55 {
                open: false,
                facing: Direction::Up,
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

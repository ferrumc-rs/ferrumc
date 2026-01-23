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
                facing: Direction::North,
                open: true,
            }),
            19432u32 => Ok(GeneratedStruct55 {
                facing: Direction::North,
                open: false,
            }),
            19433u32 => Ok(GeneratedStruct55 {
                open: true,
                facing: Direction::East,
            }),
            19434u32 => Ok(GeneratedStruct55 {
                facing: Direction::East,
                open: false,
            }),
            19435u32 => Ok(GeneratedStruct55 {
                open: true,
                facing: Direction::South,
            }),
            19436u32 => Ok(GeneratedStruct55 {
                facing: Direction::South,
                open: false,
            }),
            19437u32 => Ok(GeneratedStruct55 {
                open: true,
                facing: Direction::West,
            }),
            19438u32 => Ok(GeneratedStruct55 {
                facing: Direction::West,
                open: false,
            }),
            19439u32 => Ok(GeneratedStruct55 {
                facing: Direction::Up,
                open: true,
            }),
            19440u32 => Ok(GeneratedStruct55 {
                facing: Direction::Up,
                open: false,
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
                facing: Direction::North,
                open: true,
            } => Ok(19431u32),
            GeneratedStruct55 {
                facing: Direction::North,
                open: false,
            } => Ok(19432u32),
            GeneratedStruct55 {
                open: true,
                facing: Direction::East,
            } => Ok(19433u32),
            GeneratedStruct55 {
                facing: Direction::East,
                open: false,
            } => Ok(19434u32),
            GeneratedStruct55 {
                open: true,
                facing: Direction::South,
            } => Ok(19435u32),
            GeneratedStruct55 {
                facing: Direction::South,
                open: false,
            } => Ok(19436u32),
            GeneratedStruct55 {
                open: true,
                facing: Direction::West,
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

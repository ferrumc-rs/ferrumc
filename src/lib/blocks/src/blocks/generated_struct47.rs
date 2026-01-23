#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub struct GeneratedStruct47 {
    pub facing: Direction,
    pub has_book: bool,
    pub powered: bool,
}
impl TryFrom<u32> for GeneratedStruct47 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            19473u32 => Ok(GeneratedStruct47 {
                facing: Direction::North,
                powered: true,
                has_book: true,
            }),
            19474u32 => Ok(GeneratedStruct47 {
                has_book: true,
                powered: false,
                facing: Direction::North,
            }),
            19475u32 => Ok(GeneratedStruct47 {
                has_book: false,
                facing: Direction::North,
                powered: true,
            }),
            19476u32 => Ok(GeneratedStruct47 {
                facing: Direction::North,
                powered: false,
                has_book: false,
            }),
            19477u32 => Ok(GeneratedStruct47 {
                has_book: true,
                powered: true,
                facing: Direction::South,
            }),
            19478u32 => Ok(GeneratedStruct47 {
                has_book: true,
                powered: false,
                facing: Direction::South,
            }),
            19479u32 => Ok(GeneratedStruct47 {
                has_book: false,
                powered: true,
                facing: Direction::South,
            }),
            19480u32 => Ok(GeneratedStruct47 {
                has_book: false,
                facing: Direction::South,
                powered: false,
            }),
            19481u32 => Ok(GeneratedStruct47 {
                has_book: true,
                powered: true,
                facing: Direction::West,
            }),
            19482u32 => Ok(GeneratedStruct47 {
                facing: Direction::West,
                has_book: true,
                powered: false,
            }),
            19483u32 => Ok(GeneratedStruct47 {
                facing: Direction::West,
                powered: true,
                has_book: false,
            }),
            19484u32 => Ok(GeneratedStruct47 {
                facing: Direction::West,
                powered: false,
                has_book: false,
            }),
            19485u32 => Ok(GeneratedStruct47 {
                facing: Direction::East,
                has_book: true,
                powered: true,
            }),
            19486u32 => Ok(GeneratedStruct47 {
                facing: Direction::East,
                has_book: true,
                powered: false,
            }),
            19487u32 => Ok(GeneratedStruct47 {
                facing: Direction::East,
                has_book: false,
                powered: true,
            }),
            19488u32 => Ok(GeneratedStruct47 {
                facing: Direction::East,
                has_book: false,
                powered: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct47 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct47 {
                facing: Direction::North,
                powered: true,
                has_book: true,
            } => Ok(19473u32),
            GeneratedStruct47 {
                has_book: true,
                powered: false,
                facing: Direction::North,
            } => Ok(19474u32),
            GeneratedStruct47 {
                has_book: false,
                facing: Direction::North,
                powered: true,
            } => Ok(19475u32),
            GeneratedStruct47 {
                facing: Direction::North,
                powered: false,
                has_book: false,
            } => Ok(19476u32),
            GeneratedStruct47 {
                has_book: true,
                powered: true,
                facing: Direction::South,
            } => Ok(19477u32),
            GeneratedStruct47 {
                has_book: true,
                powered: false,
                facing: Direction::South,
            } => Ok(19478u32),
            GeneratedStruct47 {
                has_book: false,
                powered: true,
                facing: Direction::South,
            } => Ok(19479u32),
            GeneratedStruct47 {
                has_book: false,
                facing: Direction::South,
                powered: false,
            } => Ok(19480u32),
            GeneratedStruct47 {
                has_book: true,
                powered: true,
                facing: Direction::West,
            } => Ok(19481u32),
            GeneratedStruct47 {
                facing: Direction::West,
                has_book: true,
                powered: false,
            } => Ok(19482u32),
            GeneratedStruct47 {
                facing: Direction::West,
                powered: true,
                has_book: false,
            } => Ok(19483u32),
            GeneratedStruct47 {
                facing: Direction::West,
                powered: false,
                has_book: false,
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

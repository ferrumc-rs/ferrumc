#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub struct GeneratedStruct60 {
    pub facing: Direction,
    pub short: bool,
    pub ty: PistonType,
}
impl TryFrom<u32> for GeneratedStruct60 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            2069u32 => Ok(GeneratedStruct60 {
                short: true,
                facing: Direction::North,
                ty: PistonType::Default,
            }),
            2070u32 => Ok(GeneratedStruct60 {
                facing: Direction::North,
                ty: PistonType::Sticky,
                short: true,
            }),
            2071u32 => Ok(GeneratedStruct60 {
                facing: Direction::North,
                ty: PistonType::Default,
                short: false,
            }),
            2072u32 => Ok(GeneratedStruct60 {
                facing: Direction::North,
                short: false,
                ty: PistonType::Sticky,
            }),
            2073u32 => Ok(GeneratedStruct60 {
                short: true,
                ty: PistonType::Default,
                facing: Direction::East,
            }),
            2074u32 => Ok(GeneratedStruct60 {
                facing: Direction::East,
                ty: PistonType::Sticky,
                short: true,
            }),
            2075u32 => Ok(GeneratedStruct60 {
                ty: PistonType::Default,
                facing: Direction::East,
                short: false,
            }),
            2076u32 => Ok(GeneratedStruct60 {
                facing: Direction::East,
                short: false,
                ty: PistonType::Sticky,
            }),
            2077u32 => Ok(GeneratedStruct60 {
                ty: PistonType::Default,
                facing: Direction::South,
                short: true,
            }),
            2078u32 => Ok(GeneratedStruct60 {
                facing: Direction::South,
                short: true,
                ty: PistonType::Sticky,
            }),
            2079u32 => Ok(GeneratedStruct60 {
                short: false,
                facing: Direction::South,
                ty: PistonType::Default,
            }),
            2080u32 => Ok(GeneratedStruct60 {
                facing: Direction::South,
                short: false,
                ty: PistonType::Sticky,
            }),
            2081u32 => Ok(GeneratedStruct60 {
                short: true,
                facing: Direction::West,
                ty: PistonType::Default,
            }),
            2082u32 => Ok(GeneratedStruct60 {
                short: true,
                facing: Direction::West,
                ty: PistonType::Sticky,
            }),
            2083u32 => Ok(GeneratedStruct60 {
                short: false,
                ty: PistonType::Default,
                facing: Direction::West,
            }),
            2084u32 => Ok(GeneratedStruct60 {
                short: false,
                ty: PistonType::Sticky,
                facing: Direction::West,
            }),
            2085u32 => Ok(GeneratedStruct60 {
                ty: PistonType::Default,
                short: true,
                facing: Direction::Up,
            }),
            2086u32 => Ok(GeneratedStruct60 {
                ty: PistonType::Sticky,
                short: true,
                facing: Direction::Up,
            }),
            2087u32 => Ok(GeneratedStruct60 {
                ty: PistonType::Default,
                facing: Direction::Up,
                short: false,
            }),
            2088u32 => Ok(GeneratedStruct60 {
                facing: Direction::Up,
                ty: PistonType::Sticky,
                short: false,
            }),
            2089u32 => Ok(GeneratedStruct60 {
                short: true,
                facing: Direction::Down,
                ty: PistonType::Default,
            }),
            2090u32 => Ok(GeneratedStruct60 {
                ty: PistonType::Sticky,
                facing: Direction::Down,
                short: true,
            }),
            2091u32 => Ok(GeneratedStruct60 {
                facing: Direction::Down,
                ty: PistonType::Default,
                short: false,
            }),
            2092u32 => Ok(GeneratedStruct60 {
                ty: PistonType::Sticky,
                facing: Direction::Down,
                short: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct60 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct60 {
                short: true,
                facing: Direction::North,
                ty: PistonType::Default,
            } => Ok(2069u32),
            GeneratedStruct60 {
                facing: Direction::North,
                ty: PistonType::Sticky,
                short: true,
            } => Ok(2070u32),
            GeneratedStruct60 {
                facing: Direction::North,
                ty: PistonType::Default,
                short: false,
            } => Ok(2071u32),
            GeneratedStruct60 {
                facing: Direction::North,
                short: false,
                ty: PistonType::Sticky,
            } => Ok(2072u32),
            GeneratedStruct60 {
                short: true,
                ty: PistonType::Default,
                facing: Direction::East,
            } => Ok(2073u32),
            GeneratedStruct60 {
                facing: Direction::East,
                ty: PistonType::Sticky,
                short: true,
            } => Ok(2074u32),
            GeneratedStruct60 {
                ty: PistonType::Default,
                facing: Direction::East,
                short: false,
            } => Ok(2075u32),
            GeneratedStruct60 {
                facing: Direction::East,
                short: false,
                ty: PistonType::Sticky,
            } => Ok(2076u32),
            GeneratedStruct60 {
                ty: PistonType::Default,
                facing: Direction::South,
                short: true,
            } => Ok(2077u32),
            GeneratedStruct60 {
                facing: Direction::South,
                short: true,
                ty: PistonType::Sticky,
            } => Ok(2078u32),
            GeneratedStruct60 {
                short: false,
                facing: Direction::South,
                ty: PistonType::Default,
            } => Ok(2079u32),
            GeneratedStruct60 {
                facing: Direction::South,
                short: false,
                ty: PistonType::Sticky,
            } => Ok(2080u32),
            GeneratedStruct60 {
                short: true,
                facing: Direction::West,
                ty: PistonType::Default,
            } => Ok(2081u32),
            GeneratedStruct60 {
                short: true,
                facing: Direction::West,
                ty: PistonType::Sticky,
            } => Ok(2082u32),
            GeneratedStruct60 {
                short: false,
                ty: PistonType::Default,
                facing: Direction::West,
            } => Ok(2083u32),
            GeneratedStruct60 {
                short: false,
                ty: PistonType::Sticky,
                facing: Direction::West,
            } => Ok(2084u32),
            GeneratedStruct60 {
                ty: PistonType::Default,
                short: true,
                facing: Direction::Up,
            } => Ok(2085u32),
            GeneratedStruct60 {
                ty: PistonType::Sticky,
                short: true,
                facing: Direction::Up,
            } => Ok(2086u32),
            GeneratedStruct60 {
                ty: PistonType::Default,
                facing: Direction::Up,
                short: false,
            } => Ok(2087u32),
            GeneratedStruct60 {
                facing: Direction::Up,
                ty: PistonType::Sticky,
                short: false,
            } => Ok(2088u32),
            GeneratedStruct60 {
                short: true,
                facing: Direction::Down,
                ty: PistonType::Default,
            } => Ok(2089u32),
            GeneratedStruct60 {
                ty: PistonType::Sticky,
                facing: Direction::Down,
                short: true,
            } => Ok(2090u32),
            GeneratedStruct60 {
                facing: Direction::Down,
                ty: PistonType::Default,
                short: false,
            } => Ok(2091u32),
            GeneratedStruct60 {
                ty: PistonType::Sticky,
                facing: Direction::Down,
                short: false,
            } => Ok(2092u32),
            _ => Err(()),
        }
    }
}

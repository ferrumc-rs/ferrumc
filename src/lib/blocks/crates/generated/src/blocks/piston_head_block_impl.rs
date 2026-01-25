use crate::PistonHeadBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for PistonHeadBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            2069u32 => Ok(PistonHeadBlock {
                facing: Direction::North,
                short: true,
                ty: PistonType::Default,
            }),
            2070u32 => Ok(PistonHeadBlock {
                facing: Direction::North,
                short: true,
                ty: PistonType::Sticky,
            }),
            2071u32 => Ok(PistonHeadBlock {
                facing: Direction::North,
                short: false,
                ty: PistonType::Default,
            }),
            2072u32 => Ok(PistonHeadBlock {
                facing: Direction::North,
                short: false,
                ty: PistonType::Sticky,
            }),
            2073u32 => Ok(PistonHeadBlock {
                facing: Direction::East,
                short: true,
                ty: PistonType::Default,
            }),
            2074u32 => Ok(PistonHeadBlock {
                facing: Direction::East,
                short: true,
                ty: PistonType::Sticky,
            }),
            2075u32 => Ok(PistonHeadBlock {
                facing: Direction::East,
                short: false,
                ty: PistonType::Default,
            }),
            2076u32 => Ok(PistonHeadBlock {
                facing: Direction::East,
                short: false,
                ty: PistonType::Sticky,
            }),
            2077u32 => Ok(PistonHeadBlock {
                facing: Direction::South,
                short: true,
                ty: PistonType::Default,
            }),
            2078u32 => Ok(PistonHeadBlock {
                facing: Direction::South,
                short: true,
                ty: PistonType::Sticky,
            }),
            2079u32 => Ok(PistonHeadBlock {
                facing: Direction::South,
                short: false,
                ty: PistonType::Default,
            }),
            2080u32 => Ok(PistonHeadBlock {
                facing: Direction::South,
                short: false,
                ty: PistonType::Sticky,
            }),
            2081u32 => Ok(PistonHeadBlock {
                facing: Direction::West,
                short: true,
                ty: PistonType::Default,
            }),
            2082u32 => Ok(PistonHeadBlock {
                facing: Direction::West,
                short: true,
                ty: PistonType::Sticky,
            }),
            2083u32 => Ok(PistonHeadBlock {
                facing: Direction::West,
                short: false,
                ty: PistonType::Default,
            }),
            2084u32 => Ok(PistonHeadBlock {
                facing: Direction::West,
                short: false,
                ty: PistonType::Sticky,
            }),
            2085u32 => Ok(PistonHeadBlock {
                facing: Direction::Up,
                short: true,
                ty: PistonType::Default,
            }),
            2086u32 => Ok(PistonHeadBlock {
                facing: Direction::Up,
                short: true,
                ty: PistonType::Sticky,
            }),
            2087u32 => Ok(PistonHeadBlock {
                facing: Direction::Up,
                short: false,
                ty: PistonType::Default,
            }),
            2088u32 => Ok(PistonHeadBlock {
                facing: Direction::Up,
                short: false,
                ty: PistonType::Sticky,
            }),
            2089u32 => Ok(PistonHeadBlock {
                facing: Direction::Down,
                short: true,
                ty: PistonType::Default,
            }),
            2090u32 => Ok(PistonHeadBlock {
                facing: Direction::Down,
                short: true,
                ty: PistonType::Sticky,
            }),
            2091u32 => Ok(PistonHeadBlock {
                facing: Direction::Down,
                short: false,
                ty: PistonType::Default,
            }),
            2092u32 => Ok(PistonHeadBlock {
                facing: Direction::Down,
                short: false,
                ty: PistonType::Sticky,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for PistonHeadBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            PistonHeadBlock {
                facing: Direction::North,
                short: true,
                ty: PistonType::Default,
            } => Ok(2069u32),
            PistonHeadBlock {
                facing: Direction::North,
                short: true,
                ty: PistonType::Sticky,
            } => Ok(2070u32),
            PistonHeadBlock {
                facing: Direction::North,
                short: false,
                ty: PistonType::Default,
            } => Ok(2071u32),
            PistonHeadBlock {
                facing: Direction::North,
                short: false,
                ty: PistonType::Sticky,
            } => Ok(2072u32),
            PistonHeadBlock {
                facing: Direction::East,
                short: true,
                ty: PistonType::Default,
            } => Ok(2073u32),
            PistonHeadBlock {
                facing: Direction::East,
                short: true,
                ty: PistonType::Sticky,
            } => Ok(2074u32),
            PistonHeadBlock {
                facing: Direction::East,
                short: false,
                ty: PistonType::Default,
            } => Ok(2075u32),
            PistonHeadBlock {
                facing: Direction::East,
                short: false,
                ty: PistonType::Sticky,
            } => Ok(2076u32),
            PistonHeadBlock {
                facing: Direction::South,
                short: true,
                ty: PistonType::Default,
            } => Ok(2077u32),
            PistonHeadBlock {
                facing: Direction::South,
                short: true,
                ty: PistonType::Sticky,
            } => Ok(2078u32),
            PistonHeadBlock {
                facing: Direction::South,
                short: false,
                ty: PistonType::Default,
            } => Ok(2079u32),
            PistonHeadBlock {
                facing: Direction::South,
                short: false,
                ty: PistonType::Sticky,
            } => Ok(2080u32),
            PistonHeadBlock {
                facing: Direction::West,
                short: true,
                ty: PistonType::Default,
            } => Ok(2081u32),
            PistonHeadBlock {
                facing: Direction::West,
                short: true,
                ty: PistonType::Sticky,
            } => Ok(2082u32),
            PistonHeadBlock {
                facing: Direction::West,
                short: false,
                ty: PistonType::Default,
            } => Ok(2083u32),
            PistonHeadBlock {
                facing: Direction::West,
                short: false,
                ty: PistonType::Sticky,
            } => Ok(2084u32),
            PistonHeadBlock {
                facing: Direction::Up,
                short: true,
                ty: PistonType::Default,
            } => Ok(2085u32),
            PistonHeadBlock {
                facing: Direction::Up,
                short: true,
                ty: PistonType::Sticky,
            } => Ok(2086u32),
            PistonHeadBlock {
                facing: Direction::Up,
                short: false,
                ty: PistonType::Default,
            } => Ok(2087u32),
            PistonHeadBlock {
                facing: Direction::Up,
                short: false,
                ty: PistonType::Sticky,
            } => Ok(2088u32),
            PistonHeadBlock {
                facing: Direction::Down,
                short: true,
                ty: PistonType::Default,
            } => Ok(2089u32),
            PistonHeadBlock {
                facing: Direction::Down,
                short: true,
                ty: PistonType::Sticky,
            } => Ok(2090u32),
            PistonHeadBlock {
                facing: Direction::Down,
                short: false,
                ty: PistonType::Default,
            } => Ok(2091u32),
            PistonHeadBlock {
                facing: Direction::Down,
                short: false,
                ty: PistonType::Sticky,
            } => Ok(2092u32),
            _ => Err(()),
        }
    }
}

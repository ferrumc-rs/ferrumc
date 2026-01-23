#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GeneratedStruct60 {
    pub facing: Direction,
    pub short: bool,
    pub ty: PistonType,
}
impl TryInto<u32> for GeneratedStruct60 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct60 {
                facing: Direction::North,
                short: true,
                ty: PistonType::Default,
            } => Ok(2069u32),
            GeneratedStruct60 {
                facing: Direction::North,
                short: true,
                ty: PistonType::Sticky,
            } => Ok(2070u32),
            GeneratedStruct60 {
                facing: Direction::North,
                short: false,
                ty: PistonType::Default,
            } => Ok(2071u32),
            GeneratedStruct60 {
                facing: Direction::North,
                short: false,
                ty: PistonType::Sticky,
            } => Ok(2072u32),
            GeneratedStruct60 {
                facing: Direction::East,
                short: true,
                ty: PistonType::Default,
            } => Ok(2073u32),
            GeneratedStruct60 {
                facing: Direction::East,
                short: true,
                ty: PistonType::Sticky,
            } => Ok(2074u32),
            GeneratedStruct60 {
                facing: Direction::East,
                short: false,
                ty: PistonType::Default,
            } => Ok(2075u32),
            GeneratedStruct60 {
                facing: Direction::East,
                short: false,
                ty: PistonType::Sticky,
            } => Ok(2076u32),
            GeneratedStruct60 {
                facing: Direction::South,
                short: true,
                ty: PistonType::Default,
            } => Ok(2077u32),
            GeneratedStruct60 {
                facing: Direction::South,
                short: true,
                ty: PistonType::Sticky,
            } => Ok(2078u32),
            GeneratedStruct60 {
                facing: Direction::South,
                short: false,
                ty: PistonType::Default,
            } => Ok(2079u32),
            GeneratedStruct60 {
                facing: Direction::South,
                short: false,
                ty: PistonType::Sticky,
            } => Ok(2080u32),
            GeneratedStruct60 {
                facing: Direction::West,
                short: true,
                ty: PistonType::Default,
            } => Ok(2081u32),
            GeneratedStruct60 {
                facing: Direction::West,
                short: true,
                ty: PistonType::Sticky,
            } => Ok(2082u32),
            GeneratedStruct60 {
                facing: Direction::West,
                short: false,
                ty: PistonType::Default,
            } => Ok(2083u32),
            GeneratedStruct60 {
                facing: Direction::West,
                short: false,
                ty: PistonType::Sticky,
            } => Ok(2084u32),
            GeneratedStruct60 {
                facing: Direction::Up,
                short: true,
                ty: PistonType::Default,
            } => Ok(2085u32),
            GeneratedStruct60 {
                facing: Direction::Up,
                short: true,
                ty: PistonType::Sticky,
            } => Ok(2086u32),
            GeneratedStruct60 {
                facing: Direction::Up,
                short: false,
                ty: PistonType::Default,
            } => Ok(2087u32),
            GeneratedStruct60 {
                facing: Direction::Up,
                short: false,
                ty: PistonType::Sticky,
            } => Ok(2088u32),
            GeneratedStruct60 {
                facing: Direction::Down,
                short: true,
                ty: PistonType::Default,
            } => Ok(2089u32),
            GeneratedStruct60 {
                facing: Direction::Down,
                short: true,
                ty: PistonType::Sticky,
            } => Ok(2090u32),
            GeneratedStruct60 {
                facing: Direction::Down,
                short: false,
                ty: PistonType::Default,
            } => Ok(2091u32),
            GeneratedStruct60 {
                facing: Direction::Down,
                short: false,
                ty: PistonType::Sticky,
            } => Ok(2092u32),
            _ => Err(()),
        }
    }
}

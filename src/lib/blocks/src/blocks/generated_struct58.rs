#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GeneratedStruct58 {
    pub facing: Direction,
    pub powered: bool,
    pub waterlogged: bool,
}
impl TryInto<u32> for GeneratedStruct58 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct58 {
                facing: Direction::North,
                powered: true,
                waterlogged: true,
            } => Ok(25752u32),
            GeneratedStruct58 {
                facing: Direction::North,
                powered: true,
                waterlogged: false,
            } => Ok(25753u32),
            GeneratedStruct58 {
                facing: Direction::North,
                powered: false,
                waterlogged: true,
            } => Ok(25754u32),
            GeneratedStruct58 {
                facing: Direction::North,
                powered: false,
                waterlogged: false,
            } => Ok(25755u32),
            GeneratedStruct58 {
                facing: Direction::East,
                powered: true,
                waterlogged: true,
            } => Ok(25756u32),
            GeneratedStruct58 {
                facing: Direction::East,
                powered: true,
                waterlogged: false,
            } => Ok(25757u32),
            GeneratedStruct58 {
                facing: Direction::East,
                powered: false,
                waterlogged: true,
            } => Ok(25758u32),
            GeneratedStruct58 {
                facing: Direction::East,
                powered: false,
                waterlogged: false,
            } => Ok(25759u32),
            GeneratedStruct58 {
                facing: Direction::South,
                powered: true,
                waterlogged: true,
            } => Ok(25760u32),
            GeneratedStruct58 {
                facing: Direction::South,
                powered: true,
                waterlogged: false,
            } => Ok(25761u32),
            GeneratedStruct58 {
                facing: Direction::South,
                powered: false,
                waterlogged: true,
            } => Ok(25762u32),
            GeneratedStruct58 {
                facing: Direction::South,
                powered: false,
                waterlogged: false,
            } => Ok(25763u32),
            GeneratedStruct58 {
                facing: Direction::West,
                powered: true,
                waterlogged: true,
            } => Ok(25764u32),
            GeneratedStruct58 {
                facing: Direction::West,
                powered: true,
                waterlogged: false,
            } => Ok(25765u32),
            GeneratedStruct58 {
                facing: Direction::West,
                powered: false,
                waterlogged: true,
            } => Ok(25766u32),
            GeneratedStruct58 {
                facing: Direction::West,
                powered: false,
                waterlogged: false,
            } => Ok(25767u32),
            GeneratedStruct58 {
                facing: Direction::Up,
                powered: true,
                waterlogged: true,
            } => Ok(25768u32),
            GeneratedStruct58 {
                facing: Direction::Up,
                powered: true,
                waterlogged: false,
            } => Ok(25769u32),
            GeneratedStruct58 {
                facing: Direction::Up,
                powered: false,
                waterlogged: true,
            } => Ok(25770u32),
            GeneratedStruct58 {
                facing: Direction::Up,
                powered: false,
                waterlogged: false,
            } => Ok(25771u32),
            GeneratedStruct58 {
                facing: Direction::Down,
                powered: true,
                waterlogged: true,
            } => Ok(25772u32),
            GeneratedStruct58 {
                facing: Direction::Down,
                powered: true,
                waterlogged: false,
            } => Ok(25773u32),
            GeneratedStruct58 {
                facing: Direction::Down,
                powered: false,
                waterlogged: true,
            } => Ok(25774u32),
            GeneratedStruct58 {
                facing: Direction::Down,
                powered: false,
                waterlogged: false,
            } => Ok(25775u32),
            _ => Err(()),
        }
    }
}

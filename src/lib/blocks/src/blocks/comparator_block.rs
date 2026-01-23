#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ComparatorBlock {
    pub facing: Direction,
    pub mode: ComparatorMode,
    pub powered: bool,
}
impl TryInto<u32> for ComparatorBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            ComparatorBlock {
                facing: Direction::North,
                mode: ComparatorMode::Compare,
                powered: true,
            } => Ok(9984u32),
            ComparatorBlock {
                facing: Direction::North,
                mode: ComparatorMode::Compare,
                powered: false,
            } => Ok(9985u32),
            ComparatorBlock {
                facing: Direction::North,
                mode: ComparatorMode::Subtract,
                powered: true,
            } => Ok(9986u32),
            ComparatorBlock {
                facing: Direction::North,
                mode: ComparatorMode::Subtract,
                powered: false,
            } => Ok(9987u32),
            ComparatorBlock {
                facing: Direction::South,
                mode: ComparatorMode::Compare,
                powered: true,
            } => Ok(9988u32),
            ComparatorBlock {
                facing: Direction::South,
                mode: ComparatorMode::Compare,
                powered: false,
            } => Ok(9989u32),
            ComparatorBlock {
                facing: Direction::South,
                mode: ComparatorMode::Subtract,
                powered: true,
            } => Ok(9990u32),
            ComparatorBlock {
                facing: Direction::South,
                mode: ComparatorMode::Subtract,
                powered: false,
            } => Ok(9991u32),
            ComparatorBlock {
                facing: Direction::West,
                mode: ComparatorMode::Compare,
                powered: true,
            } => Ok(9992u32),
            ComparatorBlock {
                facing: Direction::West,
                mode: ComparatorMode::Compare,
                powered: false,
            } => Ok(9993u32),
            ComparatorBlock {
                facing: Direction::West,
                mode: ComparatorMode::Subtract,
                powered: true,
            } => Ok(9994u32),
            ComparatorBlock {
                facing: Direction::West,
                mode: ComparatorMode::Subtract,
                powered: false,
            } => Ok(9995u32),
            ComparatorBlock {
                facing: Direction::East,
                mode: ComparatorMode::Compare,
                powered: true,
            } => Ok(9996u32),
            ComparatorBlock {
                facing: Direction::East,
                mode: ComparatorMode::Compare,
                powered: false,
            } => Ok(9997u32),
            ComparatorBlock {
                facing: Direction::East,
                mode: ComparatorMode::Subtract,
                powered: true,
            } => Ok(9998u32),
            ComparatorBlock {
                facing: Direction::East,
                mode: ComparatorMode::Subtract,
                powered: false,
            } => Ok(9999u32),
            _ => Err(()),
        }
    }
}

#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct RailBlock {
    pub shape: RailShape,
    pub waterlogged: bool,
}
impl TryInto<u32> for RailBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            RailBlock {
                shape: RailShape::NorthSouth,
                waterlogged: true,
            } => Ok(4758u32),
            RailBlock {
                shape: RailShape::NorthSouth,
                waterlogged: false,
            } => Ok(4759u32),
            RailBlock {
                shape: RailShape::EastWest,
                waterlogged: true,
            } => Ok(4760u32),
            RailBlock {
                shape: RailShape::EastWest,
                waterlogged: false,
            } => Ok(4761u32),
            RailBlock {
                shape: RailShape::AscendingEast,
                waterlogged: true,
            } => Ok(4762u32),
            RailBlock {
                shape: RailShape::AscendingEast,
                waterlogged: false,
            } => Ok(4763u32),
            RailBlock {
                shape: RailShape::AscendingWest,
                waterlogged: true,
            } => Ok(4764u32),
            RailBlock {
                shape: RailShape::AscendingWest,
                waterlogged: false,
            } => Ok(4765u32),
            RailBlock {
                shape: RailShape::AscendingNorth,
                waterlogged: true,
            } => Ok(4766u32),
            RailBlock {
                shape: RailShape::AscendingNorth,
                waterlogged: false,
            } => Ok(4767u32),
            RailBlock {
                shape: RailShape::AscendingSouth,
                waterlogged: true,
            } => Ok(4768u32),
            RailBlock {
                shape: RailShape::AscendingSouth,
                waterlogged: false,
            } => Ok(4769u32),
            RailBlock {
                shape: RailShape::SouthEast,
                waterlogged: true,
            } => Ok(4770u32),
            RailBlock {
                shape: RailShape::SouthEast,
                waterlogged: false,
            } => Ok(4771u32),
            RailBlock {
                shape: RailShape::SouthWest,
                waterlogged: true,
            } => Ok(4772u32),
            RailBlock {
                shape: RailShape::SouthWest,
                waterlogged: false,
            } => Ok(4773u32),
            RailBlock {
                shape: RailShape::NorthWest,
                waterlogged: true,
            } => Ok(4774u32),
            RailBlock {
                shape: RailShape::NorthWest,
                waterlogged: false,
            } => Ok(4775u32),
            RailBlock {
                shape: RailShape::NorthEast,
                waterlogged: true,
            } => Ok(4776u32),
            RailBlock {
                shape: RailShape::NorthEast,
                waterlogged: false,
            } => Ok(4777u32),
            _ => Err(()),
        }
    }
}

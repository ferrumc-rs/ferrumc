#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct DripstoneBlock {
    pub thickness: DripstoneThickness,
    pub vertical_direction: Direction,
    pub waterlogged: bool,
}
impl TryInto<u32> for DripstoneBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            DripstoneBlock {
                thickness: DripstoneThickness::TipMerge,
                vertical_direction: Direction::Up,
                waterlogged: true,
            } => Ok(25776u32),
            DripstoneBlock {
                thickness: DripstoneThickness::TipMerge,
                vertical_direction: Direction::Up,
                waterlogged: false,
            } => Ok(25777u32),
            DripstoneBlock {
                thickness: DripstoneThickness::TipMerge,
                vertical_direction: Direction::Down,
                waterlogged: true,
            } => Ok(25778u32),
            DripstoneBlock {
                thickness: DripstoneThickness::TipMerge,
                vertical_direction: Direction::Down,
                waterlogged: false,
            } => Ok(25779u32),
            DripstoneBlock {
                thickness: DripstoneThickness::Tip,
                vertical_direction: Direction::Up,
                waterlogged: true,
            } => Ok(25780u32),
            DripstoneBlock {
                thickness: DripstoneThickness::Tip,
                vertical_direction: Direction::Up,
                waterlogged: false,
            } => Ok(25781u32),
            DripstoneBlock {
                thickness: DripstoneThickness::Tip,
                vertical_direction: Direction::Down,
                waterlogged: true,
            } => Ok(25782u32),
            DripstoneBlock {
                thickness: DripstoneThickness::Tip,
                vertical_direction: Direction::Down,
                waterlogged: false,
            } => Ok(25783u32),
            DripstoneBlock {
                thickness: DripstoneThickness::Frustum,
                vertical_direction: Direction::Up,
                waterlogged: true,
            } => Ok(25784u32),
            DripstoneBlock {
                thickness: DripstoneThickness::Frustum,
                vertical_direction: Direction::Up,
                waterlogged: false,
            } => Ok(25785u32),
            DripstoneBlock {
                thickness: DripstoneThickness::Frustum,
                vertical_direction: Direction::Down,
                waterlogged: true,
            } => Ok(25786u32),
            DripstoneBlock {
                thickness: DripstoneThickness::Frustum,
                vertical_direction: Direction::Down,
                waterlogged: false,
            } => Ok(25787u32),
            DripstoneBlock {
                thickness: DripstoneThickness::Middle,
                vertical_direction: Direction::Up,
                waterlogged: true,
            } => Ok(25788u32),
            DripstoneBlock {
                thickness: DripstoneThickness::Middle,
                vertical_direction: Direction::Up,
                waterlogged: false,
            } => Ok(25789u32),
            DripstoneBlock {
                thickness: DripstoneThickness::Middle,
                vertical_direction: Direction::Down,
                waterlogged: true,
            } => Ok(25790u32),
            DripstoneBlock {
                thickness: DripstoneThickness::Middle,
                vertical_direction: Direction::Down,
                waterlogged: false,
            } => Ok(25791u32),
            DripstoneBlock {
                thickness: DripstoneThickness::Base,
                vertical_direction: Direction::Up,
                waterlogged: true,
            } => Ok(25792u32),
            DripstoneBlock {
                thickness: DripstoneThickness::Base,
                vertical_direction: Direction::Up,
                waterlogged: false,
            } => Ok(25793u32),
            DripstoneBlock {
                thickness: DripstoneThickness::Base,
                vertical_direction: Direction::Down,
                waterlogged: true,
            } => Ok(25794u32),
            DripstoneBlock {
                thickness: DripstoneThickness::Base,
                vertical_direction: Direction::Down,
                waterlogged: false,
            } => Ok(25795u32),
            _ => Err(()),
        }
    }
}

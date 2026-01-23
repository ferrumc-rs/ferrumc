#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub struct DripstoneBlock {
    pub thickness: DripstoneThickness,
    pub vertical_direction: Direction,
    pub waterlogged: bool,
}
impl TryFrom<u32> for DripstoneBlock {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            25776u32 => Ok(DripstoneBlock {
                waterlogged: true,
                thickness: DripstoneThickness::TipMerge,
                vertical_direction: Direction::Up,
            }),
            25777u32 => Ok(DripstoneBlock {
                vertical_direction: Direction::Up,
                waterlogged: false,
                thickness: DripstoneThickness::TipMerge,
            }),
            25778u32 => Ok(DripstoneBlock {
                waterlogged: true,
                thickness: DripstoneThickness::TipMerge,
                vertical_direction: Direction::Down,
            }),
            25779u32 => Ok(DripstoneBlock {
                thickness: DripstoneThickness::TipMerge,
                vertical_direction: Direction::Down,
                waterlogged: false,
            }),
            25780u32 => Ok(DripstoneBlock {
                vertical_direction: Direction::Up,
                waterlogged: true,
                thickness: DripstoneThickness::Tip,
            }),
            25781u32 => Ok(DripstoneBlock {
                waterlogged: false,
                vertical_direction: Direction::Up,
                thickness: DripstoneThickness::Tip,
            }),
            25782u32 => Ok(DripstoneBlock {
                vertical_direction: Direction::Down,
                thickness: DripstoneThickness::Tip,
                waterlogged: true,
            }),
            25783u32 => Ok(DripstoneBlock {
                thickness: DripstoneThickness::Tip,
                vertical_direction: Direction::Down,
                waterlogged: false,
            }),
            25784u32 => Ok(DripstoneBlock {
                vertical_direction: Direction::Up,
                waterlogged: true,
                thickness: DripstoneThickness::Frustum,
            }),
            25785u32 => Ok(DripstoneBlock {
                thickness: DripstoneThickness::Frustum,
                waterlogged: false,
                vertical_direction: Direction::Up,
            }),
            25786u32 => Ok(DripstoneBlock {
                waterlogged: true,
                vertical_direction: Direction::Down,
                thickness: DripstoneThickness::Frustum,
            }),
            25787u32 => Ok(DripstoneBlock {
                thickness: DripstoneThickness::Frustum,
                vertical_direction: Direction::Down,
                waterlogged: false,
            }),
            25788u32 => Ok(DripstoneBlock {
                thickness: DripstoneThickness::Middle,
                vertical_direction: Direction::Up,
                waterlogged: true,
            }),
            25789u32 => Ok(DripstoneBlock {
                thickness: DripstoneThickness::Middle,
                waterlogged: false,
                vertical_direction: Direction::Up,
            }),
            25790u32 => Ok(DripstoneBlock {
                waterlogged: true,
                thickness: DripstoneThickness::Middle,
                vertical_direction: Direction::Down,
            }),
            25791u32 => Ok(DripstoneBlock {
                thickness: DripstoneThickness::Middle,
                vertical_direction: Direction::Down,
                waterlogged: false,
            }),
            25792u32 => Ok(DripstoneBlock {
                waterlogged: true,
                thickness: DripstoneThickness::Base,
                vertical_direction: Direction::Up,
            }),
            25793u32 => Ok(DripstoneBlock {
                vertical_direction: Direction::Up,
                thickness: DripstoneThickness::Base,
                waterlogged: false,
            }),
            25794u32 => Ok(DripstoneBlock {
                waterlogged: true,
                vertical_direction: Direction::Down,
                thickness: DripstoneThickness::Base,
            }),
            25795u32 => Ok(DripstoneBlock {
                vertical_direction: Direction::Down,
                waterlogged: false,
                thickness: DripstoneThickness::Base,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for DripstoneBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            DripstoneBlock {
                waterlogged: true,
                thickness: DripstoneThickness::TipMerge,
                vertical_direction: Direction::Up,
            } => Ok(25776u32),
            DripstoneBlock {
                vertical_direction: Direction::Up,
                waterlogged: false,
                thickness: DripstoneThickness::TipMerge,
            } => Ok(25777u32),
            DripstoneBlock {
                waterlogged: true,
                thickness: DripstoneThickness::TipMerge,
                vertical_direction: Direction::Down,
            } => Ok(25778u32),
            DripstoneBlock {
                thickness: DripstoneThickness::TipMerge,
                vertical_direction: Direction::Down,
                waterlogged: false,
            } => Ok(25779u32),
            DripstoneBlock {
                vertical_direction: Direction::Up,
                waterlogged: true,
                thickness: DripstoneThickness::Tip,
            } => Ok(25780u32),
            DripstoneBlock {
                waterlogged: false,
                vertical_direction: Direction::Up,
                thickness: DripstoneThickness::Tip,
            } => Ok(25781u32),
            DripstoneBlock {
                vertical_direction: Direction::Down,
                thickness: DripstoneThickness::Tip,
                waterlogged: true,
            } => Ok(25782u32),
            DripstoneBlock {
                thickness: DripstoneThickness::Tip,
                vertical_direction: Direction::Down,
                waterlogged: false,
            } => Ok(25783u32),
            DripstoneBlock {
                vertical_direction: Direction::Up,
                waterlogged: true,
                thickness: DripstoneThickness::Frustum,
            } => Ok(25784u32),
            DripstoneBlock {
                thickness: DripstoneThickness::Frustum,
                waterlogged: false,
                vertical_direction: Direction::Up,
            } => Ok(25785u32),
            DripstoneBlock {
                waterlogged: true,
                vertical_direction: Direction::Down,
                thickness: DripstoneThickness::Frustum,
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
                waterlogged: false,
                vertical_direction: Direction::Up,
            } => Ok(25789u32),
            DripstoneBlock {
                waterlogged: true,
                thickness: DripstoneThickness::Middle,
                vertical_direction: Direction::Down,
            } => Ok(25790u32),
            DripstoneBlock {
                thickness: DripstoneThickness::Middle,
                vertical_direction: Direction::Down,
                waterlogged: false,
            } => Ok(25791u32),
            DripstoneBlock {
                waterlogged: true,
                thickness: DripstoneThickness::Base,
                vertical_direction: Direction::Up,
            } => Ok(25792u32),
            DripstoneBlock {
                vertical_direction: Direction::Up,
                thickness: DripstoneThickness::Base,
                waterlogged: false,
            } => Ok(25793u32),
            DripstoneBlock {
                waterlogged: true,
                vertical_direction: Direction::Down,
                thickness: DripstoneThickness::Base,
            } => Ok(25794u32),
            DripstoneBlock {
                vertical_direction: Direction::Down,
                waterlogged: false,
                thickness: DripstoneThickness::Base,
            } => Ok(25795u32),
            _ => Err(()),
        }
    }
}

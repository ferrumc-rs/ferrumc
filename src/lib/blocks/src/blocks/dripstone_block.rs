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
                vertical_direction: Direction::Up,
                thickness: DripstoneThickness::TipMerge,
                waterlogged: true,
            }),
            25777u32 => Ok(DripstoneBlock {
                thickness: DripstoneThickness::TipMerge,
                waterlogged: false,
                vertical_direction: Direction::Up,
            }),
            25778u32 => Ok(DripstoneBlock {
                vertical_direction: Direction::Down,
                thickness: DripstoneThickness::TipMerge,
                waterlogged: true,
            }),
            25779u32 => Ok(DripstoneBlock {
                vertical_direction: Direction::Down,
                thickness: DripstoneThickness::TipMerge,
                waterlogged: false,
            }),
            25780u32 => Ok(DripstoneBlock {
                thickness: DripstoneThickness::Tip,
                waterlogged: true,
                vertical_direction: Direction::Up,
            }),
            25781u32 => Ok(DripstoneBlock {
                vertical_direction: Direction::Up,
                waterlogged: false,
                thickness: DripstoneThickness::Tip,
            }),
            25782u32 => Ok(DripstoneBlock {
                thickness: DripstoneThickness::Tip,
                vertical_direction: Direction::Down,
                waterlogged: true,
            }),
            25783u32 => Ok(DripstoneBlock {
                vertical_direction: Direction::Down,
                thickness: DripstoneThickness::Tip,
                waterlogged: false,
            }),
            25784u32 => Ok(DripstoneBlock {
                thickness: DripstoneThickness::Frustum,
                waterlogged: true,
                vertical_direction: Direction::Up,
            }),
            25785u32 => Ok(DripstoneBlock {
                thickness: DripstoneThickness::Frustum,
                vertical_direction: Direction::Up,
                waterlogged: false,
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
                vertical_direction: Direction::Up,
                waterlogged: true,
                thickness: DripstoneThickness::Middle,
            }),
            25789u32 => Ok(DripstoneBlock {
                vertical_direction: Direction::Up,
                thickness: DripstoneThickness::Middle,
                waterlogged: false,
            }),
            25790u32 => Ok(DripstoneBlock {
                vertical_direction: Direction::Down,
                waterlogged: true,
                thickness: DripstoneThickness::Middle,
            }),
            25791u32 => Ok(DripstoneBlock {
                vertical_direction: Direction::Down,
                waterlogged: false,
                thickness: DripstoneThickness::Middle,
            }),
            25792u32 => Ok(DripstoneBlock {
                vertical_direction: Direction::Up,
                waterlogged: true,
                thickness: DripstoneThickness::Base,
            }),
            25793u32 => Ok(DripstoneBlock {
                vertical_direction: Direction::Up,
                waterlogged: false,
                thickness: DripstoneThickness::Base,
            }),
            25794u32 => Ok(DripstoneBlock {
                vertical_direction: Direction::Down,
                thickness: DripstoneThickness::Base,
                waterlogged: true,
            }),
            25795u32 => Ok(DripstoneBlock {
                waterlogged: false,
                vertical_direction: Direction::Down,
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
                vertical_direction: Direction::Up,
                thickness: DripstoneThickness::TipMerge,
                waterlogged: true,
            } => Ok(25776u32),
            DripstoneBlock {
                thickness: DripstoneThickness::TipMerge,
                waterlogged: false,
                vertical_direction: Direction::Up,
            } => Ok(25777u32),
            DripstoneBlock {
                vertical_direction: Direction::Down,
                thickness: DripstoneThickness::TipMerge,
                waterlogged: true,
            } => Ok(25778u32),
            DripstoneBlock {
                vertical_direction: Direction::Down,
                thickness: DripstoneThickness::TipMerge,
                waterlogged: false,
            } => Ok(25779u32),
            DripstoneBlock {
                thickness: DripstoneThickness::Tip,
                waterlogged: true,
                vertical_direction: Direction::Up,
            } => Ok(25780u32),
            DripstoneBlock {
                vertical_direction: Direction::Up,
                waterlogged: false,
                thickness: DripstoneThickness::Tip,
            } => Ok(25781u32),
            DripstoneBlock {
                thickness: DripstoneThickness::Tip,
                vertical_direction: Direction::Down,
                waterlogged: true,
            } => Ok(25782u32),
            DripstoneBlock {
                vertical_direction: Direction::Down,
                thickness: DripstoneThickness::Tip,
                waterlogged: false,
            } => Ok(25783u32),
            DripstoneBlock {
                thickness: DripstoneThickness::Frustum,
                waterlogged: true,
                vertical_direction: Direction::Up,
            } => Ok(25784u32),
            DripstoneBlock {
                thickness: DripstoneThickness::Frustum,
                vertical_direction: Direction::Up,
                waterlogged: false,
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
                vertical_direction: Direction::Up,
                waterlogged: true,
                thickness: DripstoneThickness::Middle,
            } => Ok(25788u32),
            DripstoneBlock {
                vertical_direction: Direction::Up,
                thickness: DripstoneThickness::Middle,
                waterlogged: false,
            } => Ok(25789u32),
            DripstoneBlock {
                vertical_direction: Direction::Down,
                waterlogged: true,
                thickness: DripstoneThickness::Middle,
            } => Ok(25790u32),
            DripstoneBlock {
                vertical_direction: Direction::Down,
                waterlogged: false,
                thickness: DripstoneThickness::Middle,
            } => Ok(25791u32),
            DripstoneBlock {
                vertical_direction: Direction::Up,
                waterlogged: true,
                thickness: DripstoneThickness::Base,
            } => Ok(25792u32),
            DripstoneBlock {
                vertical_direction: Direction::Up,
                waterlogged: false,
                thickness: DripstoneThickness::Base,
            } => Ok(25793u32),
            DripstoneBlock {
                vertical_direction: Direction::Down,
                thickness: DripstoneThickness::Base,
                waterlogged: true,
            } => Ok(25794u32),
            DripstoneBlock {
                waterlogged: false,
                vertical_direction: Direction::Down,
                thickness: DripstoneThickness::Base,
            } => Ok(25795u32),
            _ => Err(()),
        }
    }
}

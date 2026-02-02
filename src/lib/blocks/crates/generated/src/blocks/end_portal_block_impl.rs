use crate::EndPortalBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for EndPortalBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            8191u32 => Ok(EndPortalBlock {
                eye: true,
                facing: Direction::North,
            }),
            8192u32 => Ok(EndPortalBlock {
                eye: true,
                facing: Direction::South,
            }),
            8193u32 => Ok(EndPortalBlock {
                eye: true,
                facing: Direction::West,
            }),
            8194u32 => Ok(EndPortalBlock {
                eye: true,
                facing: Direction::East,
            }),
            8195u32 => Ok(EndPortalBlock {
                eye: false,
                facing: Direction::North,
            }),
            8196u32 => Ok(EndPortalBlock {
                eye: false,
                facing: Direction::South,
            }),
            8197u32 => Ok(EndPortalBlock {
                eye: false,
                facing: Direction::West,
            }),
            8198u32 => Ok(EndPortalBlock {
                eye: false,
                facing: Direction::East,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for EndPortalBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            EndPortalBlock {
                eye: true,
                facing: Direction::North,
            } => Ok(8191u32),
            EndPortalBlock {
                eye: true,
                facing: Direction::South,
            } => Ok(8192u32),
            EndPortalBlock {
                eye: true,
                facing: Direction::West,
            } => Ok(8193u32),
            EndPortalBlock {
                eye: true,
                facing: Direction::East,
            } => Ok(8194u32),
            EndPortalBlock {
                eye: false,
                facing: Direction::North,
            } => Ok(8195u32),
            EndPortalBlock {
                eye: false,
                facing: Direction::South,
            } => Ok(8196u32),
            EndPortalBlock {
                eye: false,
                facing: Direction::West,
            } => Ok(8197u32),
            EndPortalBlock {
                eye: false,
                facing: Direction::East,
            } => Ok(8198u32),
            _ => Err(()),
        }
    }
}

use crate::TripwireHookBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for TripwireHookBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            8305u32 => Ok(TripwireHookBlock {
                attached: true,
                facing: Direction::North,
                powered: true,
            }),
            8306u32 => Ok(TripwireHookBlock {
                attached: true,
                facing: Direction::North,
                powered: false,
            }),
            8307u32 => Ok(TripwireHookBlock {
                attached: true,
                facing: Direction::South,
                powered: true,
            }),
            8308u32 => Ok(TripwireHookBlock {
                attached: true,
                facing: Direction::South,
                powered: false,
            }),
            8309u32 => Ok(TripwireHookBlock {
                attached: true,
                facing: Direction::West,
                powered: true,
            }),
            8310u32 => Ok(TripwireHookBlock {
                attached: true,
                facing: Direction::West,
                powered: false,
            }),
            8311u32 => Ok(TripwireHookBlock {
                attached: true,
                facing: Direction::East,
                powered: true,
            }),
            8312u32 => Ok(TripwireHookBlock {
                attached: true,
                facing: Direction::East,
                powered: false,
            }),
            8313u32 => Ok(TripwireHookBlock {
                attached: false,
                facing: Direction::North,
                powered: true,
            }),
            8314u32 => Ok(TripwireHookBlock {
                attached: false,
                facing: Direction::North,
                powered: false,
            }),
            8315u32 => Ok(TripwireHookBlock {
                attached: false,
                facing: Direction::South,
                powered: true,
            }),
            8316u32 => Ok(TripwireHookBlock {
                attached: false,
                facing: Direction::South,
                powered: false,
            }),
            8317u32 => Ok(TripwireHookBlock {
                attached: false,
                facing: Direction::West,
                powered: true,
            }),
            8318u32 => Ok(TripwireHookBlock {
                attached: false,
                facing: Direction::West,
                powered: false,
            }),
            8319u32 => Ok(TripwireHookBlock {
                attached: false,
                facing: Direction::East,
                powered: true,
            }),
            8320u32 => Ok(TripwireHookBlock {
                attached: false,
                facing: Direction::East,
                powered: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for TripwireHookBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            TripwireHookBlock {
                attached: true,
                facing: Direction::North,
                powered: true,
            } => Ok(8305u32),
            TripwireHookBlock {
                attached: true,
                facing: Direction::North,
                powered: false,
            } => Ok(8306u32),
            TripwireHookBlock {
                attached: true,
                facing: Direction::South,
                powered: true,
            } => Ok(8307u32),
            TripwireHookBlock {
                attached: true,
                facing: Direction::South,
                powered: false,
            } => Ok(8308u32),
            TripwireHookBlock {
                attached: true,
                facing: Direction::West,
                powered: true,
            } => Ok(8309u32),
            TripwireHookBlock {
                attached: true,
                facing: Direction::West,
                powered: false,
            } => Ok(8310u32),
            TripwireHookBlock {
                attached: true,
                facing: Direction::East,
                powered: true,
            } => Ok(8311u32),
            TripwireHookBlock {
                attached: true,
                facing: Direction::East,
                powered: false,
            } => Ok(8312u32),
            TripwireHookBlock {
                attached: false,
                facing: Direction::North,
                powered: true,
            } => Ok(8313u32),
            TripwireHookBlock {
                attached: false,
                facing: Direction::North,
                powered: false,
            } => Ok(8314u32),
            TripwireHookBlock {
                attached: false,
                facing: Direction::South,
                powered: true,
            } => Ok(8315u32),
            TripwireHookBlock {
                attached: false,
                facing: Direction::South,
                powered: false,
            } => Ok(8316u32),
            TripwireHookBlock {
                attached: false,
                facing: Direction::West,
                powered: true,
            } => Ok(8317u32),
            TripwireHookBlock {
                attached: false,
                facing: Direction::West,
                powered: false,
            } => Ok(8318u32),
            TripwireHookBlock {
                attached: false,
                facing: Direction::East,
                powered: true,
            } => Ok(8319u32),
            TripwireHookBlock {
                attached: false,
                facing: Direction::East,
                powered: false,
            } => Ok(8320u32),
            _ => Err(()),
        }
    }
}

use crate::HopperBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for HopperBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            10034u32 => Ok(HopperBlock {
                enabled: true,
                facing: Direction::Down,
            }),
            10035u32 => Ok(HopperBlock {
                enabled: true,
                facing: Direction::North,
            }),
            10036u32 => Ok(HopperBlock {
                enabled: true,
                facing: Direction::South,
            }),
            10037u32 => Ok(HopperBlock {
                enabled: true,
                facing: Direction::West,
            }),
            10038u32 => Ok(HopperBlock {
                enabled: true,
                facing: Direction::East,
            }),
            10039u32 => Ok(HopperBlock {
                enabled: false,
                facing: Direction::Down,
            }),
            10040u32 => Ok(HopperBlock {
                enabled: false,
                facing: Direction::North,
            }),
            10041u32 => Ok(HopperBlock {
                enabled: false,
                facing: Direction::South,
            }),
            10042u32 => Ok(HopperBlock {
                enabled: false,
                facing: Direction::West,
            }),
            10043u32 => Ok(HopperBlock {
                enabled: false,
                facing: Direction::East,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for HopperBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            HopperBlock {
                enabled: true,
                facing: Direction::Down,
            } => Ok(10034u32),
            HopperBlock {
                enabled: true,
                facing: Direction::North,
            } => Ok(10035u32),
            HopperBlock {
                enabled: true,
                facing: Direction::South,
            } => Ok(10036u32),
            HopperBlock {
                enabled: true,
                facing: Direction::West,
            } => Ok(10037u32),
            HopperBlock {
                enabled: true,
                facing: Direction::East,
            } => Ok(10038u32),
            HopperBlock {
                enabled: false,
                facing: Direction::Down,
            } => Ok(10039u32),
            HopperBlock {
                enabled: false,
                facing: Direction::North,
            } => Ok(10040u32),
            HopperBlock {
                enabled: false,
                facing: Direction::South,
            } => Ok(10041u32),
            HopperBlock {
                enabled: false,
                facing: Direction::West,
            } => Ok(10042u32),
            HopperBlock {
                enabled: false,
                facing: Direction::East,
            } => Ok(10043u32),
            _ => Err(()),
        }
    }
}

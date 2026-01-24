use crate::StemBlock;
use crate::StemBlockType;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for StemBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            7060u32 => Ok(StemBlock {
                block_type: StemBlockType::AttachedMelonStem,
                facing: Direction::North,
            }),
            7061u32 => Ok(StemBlock {
                block_type: StemBlockType::AttachedMelonStem,
                facing: Direction::South,
            }),
            7062u32 => Ok(StemBlock {
                block_type: StemBlockType::AttachedMelonStem,
                facing: Direction::West,
            }),
            7063u32 => Ok(StemBlock {
                block_type: StemBlockType::AttachedMelonStem,
                facing: Direction::East,
            }),
            7056u32 => Ok(StemBlock {
                block_type: StemBlockType::AttachedPumpkinStem,
                facing: Direction::North,
            }),
            7057u32 => Ok(StemBlock {
                block_type: StemBlockType::AttachedPumpkinStem,
                facing: Direction::South,
            }),
            7058u32 => Ok(StemBlock {
                block_type: StemBlockType::AttachedPumpkinStem,
                facing: Direction::West,
            }),
            7059u32 => Ok(StemBlock {
                block_type: StemBlockType::AttachedPumpkinStem,
                facing: Direction::East,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for StemBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            StemBlock {
                block_type: StemBlockType::AttachedMelonStem,
                facing: Direction::North,
            } => Ok(7060u32),
            StemBlock {
                block_type: StemBlockType::AttachedMelonStem,
                facing: Direction::South,
            } => Ok(7061u32),
            StemBlock {
                block_type: StemBlockType::AttachedMelonStem,
                facing: Direction::West,
            } => Ok(7062u32),
            StemBlock {
                block_type: StemBlockType::AttachedMelonStem,
                facing: Direction::East,
            } => Ok(7063u32),
            StemBlock {
                block_type: StemBlockType::AttachedPumpkinStem,
                facing: Direction::North,
            } => Ok(7056u32),
            StemBlock {
                block_type: StemBlockType::AttachedPumpkinStem,
                facing: Direction::South,
            } => Ok(7057u32),
            StemBlock {
                block_type: StemBlockType::AttachedPumpkinStem,
                facing: Direction::West,
            } => Ok(7058u32),
            StemBlock {
                block_type: StemBlockType::AttachedPumpkinStem,
                facing: Direction::East,
            } => Ok(7059u32),
            _ => Err(()),
        }
    }
}

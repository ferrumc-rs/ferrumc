use crate::SculkShriekerBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for SculkShriekerBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            23958u32 => Ok(SculkShriekerBlock {
                can_summon: true,
                shrieking: true,
                waterlogged: true,
            }),
            23959u32 => Ok(SculkShriekerBlock {
                can_summon: true,
                shrieking: true,
                waterlogged: false,
            }),
            23960u32 => Ok(SculkShriekerBlock {
                can_summon: true,
                shrieking: false,
                waterlogged: true,
            }),
            23961u32 => Ok(SculkShriekerBlock {
                can_summon: true,
                shrieking: false,
                waterlogged: false,
            }),
            23962u32 => Ok(SculkShriekerBlock {
                can_summon: false,
                shrieking: true,
                waterlogged: true,
            }),
            23963u32 => Ok(SculkShriekerBlock {
                can_summon: false,
                shrieking: true,
                waterlogged: false,
            }),
            23964u32 => Ok(SculkShriekerBlock {
                can_summon: false,
                shrieking: false,
                waterlogged: true,
            }),
            23965u32 => Ok(SculkShriekerBlock {
                can_summon: false,
                shrieking: false,
                waterlogged: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for SculkShriekerBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            SculkShriekerBlock {
                can_summon: true,
                shrieking: true,
                waterlogged: true,
            } => Ok(23958u32),
            SculkShriekerBlock {
                can_summon: true,
                shrieking: true,
                waterlogged: false,
            } => Ok(23959u32),
            SculkShriekerBlock {
                can_summon: true,
                shrieking: false,
                waterlogged: true,
            } => Ok(23960u32),
            SculkShriekerBlock {
                can_summon: true,
                shrieking: false,
                waterlogged: false,
            } => Ok(23961u32),
            SculkShriekerBlock {
                can_summon: false,
                shrieking: true,
                waterlogged: true,
            } => Ok(23962u32),
            SculkShriekerBlock {
                can_summon: false,
                shrieking: true,
                waterlogged: false,
            } => Ok(23963u32),
            SculkShriekerBlock {
                can_summon: false,
                shrieking: false,
                waterlogged: true,
            } => Ok(23964u32),
            SculkShriekerBlock {
                can_summon: false,
                shrieking: false,
                waterlogged: false,
            } => Ok(23965u32),
            _ => Err(()),
        }
    }
}

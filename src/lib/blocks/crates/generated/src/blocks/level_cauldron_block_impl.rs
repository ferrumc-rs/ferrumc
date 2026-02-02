use crate::LevelCauldronBlock;
use crate::LevelCauldronBlockType;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for LevelCauldronBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            8187u32 => Ok(LevelCauldronBlock {
                block_type: LevelCauldronBlockType::PowderSnowCauldron,
                level: 1i32,
            }),
            8188u32 => Ok(LevelCauldronBlock {
                block_type: LevelCauldronBlockType::PowderSnowCauldron,
                level: 2i32,
            }),
            8189u32 => Ok(LevelCauldronBlock {
                block_type: LevelCauldronBlockType::PowderSnowCauldron,
                level: 3i32,
            }),
            8183u32 => Ok(LevelCauldronBlock {
                block_type: LevelCauldronBlockType::WaterCauldron,
                level: 1i32,
            }),
            8184u32 => Ok(LevelCauldronBlock {
                block_type: LevelCauldronBlockType::WaterCauldron,
                level: 2i32,
            }),
            8185u32 => Ok(LevelCauldronBlock {
                block_type: LevelCauldronBlockType::WaterCauldron,
                level: 3i32,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for LevelCauldronBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            LevelCauldronBlock {
                block_type: LevelCauldronBlockType::PowderSnowCauldron,
                level: 1i32,
            } => Ok(8187u32),
            LevelCauldronBlock {
                block_type: LevelCauldronBlockType::PowderSnowCauldron,
                level: 2i32,
            } => Ok(8188u32),
            LevelCauldronBlock {
                block_type: LevelCauldronBlockType::PowderSnowCauldron,
                level: 3i32,
            } => Ok(8189u32),
            LevelCauldronBlock {
                block_type: LevelCauldronBlockType::WaterCauldron,
                level: 1i32,
            } => Ok(8183u32),
            LevelCauldronBlock {
                block_type: LevelCauldronBlockType::WaterCauldron,
                level: 2i32,
            } => Ok(8184u32),
            LevelCauldronBlock {
                block_type: LevelCauldronBlockType::WaterCauldron,
                level: 3i32,
            } => Ok(8185u32),
            _ => Err(()),
        }
    }
}

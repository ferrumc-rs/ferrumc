use crate::RedstoneOreBlock;
use crate::RedstoneOreBlockType;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for RedstoneOreBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            5914u32 => Ok(RedstoneOreBlock {
                block_type: RedstoneOreBlockType::DeepslateRedstoneOre,
                lit: true,
            }),
            5915u32 => Ok(RedstoneOreBlock {
                block_type: RedstoneOreBlockType::DeepslateRedstoneOre,
                lit: false,
            }),
            5912u32 => Ok(RedstoneOreBlock {
                block_type: RedstoneOreBlockType::RedstoneOre,
                lit: true,
            }),
            5913u32 => Ok(RedstoneOreBlock {
                block_type: RedstoneOreBlockType::RedstoneOre,
                lit: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for RedstoneOreBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            RedstoneOreBlock {
                block_type: RedstoneOreBlockType::DeepslateRedstoneOre,
                lit: true,
            } => Ok(5914u32),
            RedstoneOreBlock {
                block_type: RedstoneOreBlockType::DeepslateRedstoneOre,
                lit: false,
            } => Ok(5915u32),
            RedstoneOreBlock {
                block_type: RedstoneOreBlockType::RedstoneOre,
                lit: true,
            } => Ok(5912u32),
            RedstoneOreBlock {
                block_type: RedstoneOreBlockType::RedstoneOre,
                lit: false,
            } => Ok(5913u32),
            _ => Err(()),
        }
    }
}

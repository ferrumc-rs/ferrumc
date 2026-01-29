use crate::StructureBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for StructureBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            20379u32 => Ok(StructureBlock {
                mode: StructureMode::Save,
            }),
            20380u32 => Ok(StructureBlock {
                mode: StructureMode::Load,
            }),
            20381u32 => Ok(StructureBlock {
                mode: StructureMode::Corner,
            }),
            20382u32 => Ok(StructureBlock {
                mode: StructureMode::Data,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for StructureBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            StructureBlock {
                mode: StructureMode::Save,
            } => Ok(20379u32),
            StructureBlock {
                mode: StructureMode::Load,
            } => Ok(20380u32),
            StructureBlock {
                mode: StructureMode::Corner,
            } => Ok(20381u32),
            StructureBlock {
                mode: StructureMode::Data,
            } => Ok(20382u32),
            _ => Err(()),
        }
    }
}

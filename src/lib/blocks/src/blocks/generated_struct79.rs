#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GeneratedStruct79 {
    pub mode: StructureMode,
}
impl GeneratedStruct79 {
    pub(crate) const VTABLE: crate::BlockBehaviorTable =
        crate::BlockBehaviorTable::from::<GeneratedStruct79>();
}
impl TryFrom<u32> for GeneratedStruct79 {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            20379u32 => Ok(GeneratedStruct79 {
                mode: StructureMode::Save,
            }),
            20380u32 => Ok(GeneratedStruct79 {
                mode: StructureMode::Load,
            }),
            20381u32 => Ok(GeneratedStruct79 {
                mode: StructureMode::Corner,
            }),
            20382u32 => Ok(GeneratedStruct79 {
                mode: StructureMode::Data,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct79 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct79 {
                mode: StructureMode::Save,
            } => Ok(20379u32),
            GeneratedStruct79 {
                mode: StructureMode::Load,
            } => Ok(20380u32),
            GeneratedStruct79 {
                mode: StructureMode::Corner,
            } => Ok(20381u32),
            GeneratedStruct79 {
                mode: StructureMode::Data,
            } => Ok(20382u32),
            _ => Err(()),
        }
    }
}

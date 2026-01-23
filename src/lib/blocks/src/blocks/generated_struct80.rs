#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GeneratedStruct80 {
    pub mode: TestBlockMode,
}
impl GeneratedStruct80 {
    pub(crate) const VTABLE: crate::BlockBehaviorTable =
        crate::BlockBehaviorTable::from::<GeneratedStruct80>();
}
impl TryFrom<u32> for GeneratedStruct80 {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            20395u32 => Ok(GeneratedStruct80 {
                mode: TestBlockMode::Start,
            }),
            20396u32 => Ok(GeneratedStruct80 {
                mode: TestBlockMode::Log,
            }),
            20397u32 => Ok(GeneratedStruct80 {
                mode: TestBlockMode::Fail,
            }),
            20398u32 => Ok(GeneratedStruct80 {
                mode: TestBlockMode::Accept,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct80 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct80 {
                mode: TestBlockMode::Start,
            } => Ok(20395u32),
            GeneratedStruct80 {
                mode: TestBlockMode::Log,
            } => Ok(20396u32),
            GeneratedStruct80 {
                mode: TestBlockMode::Fail,
            } => Ok(20397u32),
            GeneratedStruct80 {
                mode: TestBlockMode::Accept,
            } => Ok(20398u32),
            _ => Err(()),
        }
    }
}

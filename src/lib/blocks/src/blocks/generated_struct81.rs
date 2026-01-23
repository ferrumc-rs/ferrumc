#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GeneratedStruct81 {
    pub moisture: i32,
}
impl GeneratedStruct81 {
    pub(crate) const VTABLE: crate::BlockBehaviorTable =
        crate::BlockBehaviorTable::from::<GeneratedStruct81>();
}
impl TryFrom<u32> for GeneratedStruct81 {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            4350u32 => Ok(GeneratedStruct81 { moisture: 0i32 }),
            4351u32 => Ok(GeneratedStruct81 { moisture: 1i32 }),
            4352u32 => Ok(GeneratedStruct81 { moisture: 2i32 }),
            4353u32 => Ok(GeneratedStruct81 { moisture: 3i32 }),
            4354u32 => Ok(GeneratedStruct81 { moisture: 4i32 }),
            4355u32 => Ok(GeneratedStruct81 { moisture: 5i32 }),
            4356u32 => Ok(GeneratedStruct81 { moisture: 6i32 }),
            4357u32 => Ok(GeneratedStruct81 { moisture: 7i32 }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct81 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct81 { moisture: 0i32 } => Ok(4350u32),
            GeneratedStruct81 { moisture: 1i32 } => Ok(4351u32),
            GeneratedStruct81 { moisture: 2i32 } => Ok(4352u32),
            GeneratedStruct81 { moisture: 3i32 } => Ok(4353u32),
            GeneratedStruct81 { moisture: 4i32 } => Ok(4354u32),
            GeneratedStruct81 { moisture: 5i32 } => Ok(4355u32),
            GeneratedStruct81 { moisture: 6i32 } => Ok(4356u32),
            GeneratedStruct81 { moisture: 7i32 } => Ok(4357u32),
            _ => Err(()),
        }
    }
}

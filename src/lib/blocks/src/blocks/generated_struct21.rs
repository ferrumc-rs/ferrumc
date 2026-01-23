#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GeneratedStruct21 {
    pub charges: i32,
}
impl GeneratedStruct21 {
    pub(crate) const VTABLE: crate::BlockBehaviorTable =
        crate::BlockBehaviorTable::from::<GeneratedStruct21>();
}
impl TryFrom<u32> for GeneratedStruct21 {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            20478u32 => Ok(GeneratedStruct21 { charges: 0i32 }),
            20479u32 => Ok(GeneratedStruct21 { charges: 1i32 }),
            20480u32 => Ok(GeneratedStruct21 { charges: 2i32 }),
            20481u32 => Ok(GeneratedStruct21 { charges: 3i32 }),
            20482u32 => Ok(GeneratedStruct21 { charges: 4i32 }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct21 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct21 { charges: 0i32 } => Ok(20478u32),
            GeneratedStruct21 { charges: 1i32 } => Ok(20479u32),
            GeneratedStruct21 { charges: 2i32 } => Ok(20480u32),
            GeneratedStruct21 { charges: 3i32 } => Ok(20481u32),
            GeneratedStruct21 { charges: 4i32 } => Ok(20482u32),
            _ => Err(()),
        }
    }
}

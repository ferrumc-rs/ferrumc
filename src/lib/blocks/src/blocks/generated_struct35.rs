#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GeneratedStruct35 {
    pub eggs: i32,
    pub hatch: i32,
}
impl GeneratedStruct35 {
    pub(crate) const VTABLE: crate::BlockBehaviorTable =
        crate::BlockBehaviorTable::from::<GeneratedStruct35>();
}
impl TryFrom<u32> for GeneratedStruct35 {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            13811u32 => Ok(GeneratedStruct35 {
                eggs: 1i32,
                hatch: 0i32,
            }),
            13812u32 => Ok(GeneratedStruct35 {
                eggs: 1i32,
                hatch: 1i32,
            }),
            13813u32 => Ok(GeneratedStruct35 {
                eggs: 1i32,
                hatch: 2i32,
            }),
            13814u32 => Ok(GeneratedStruct35 {
                eggs: 2i32,
                hatch: 0i32,
            }),
            13815u32 => Ok(GeneratedStruct35 {
                eggs: 2i32,
                hatch: 1i32,
            }),
            13816u32 => Ok(GeneratedStruct35 {
                eggs: 2i32,
                hatch: 2i32,
            }),
            13817u32 => Ok(GeneratedStruct35 {
                eggs: 3i32,
                hatch: 0i32,
            }),
            13818u32 => Ok(GeneratedStruct35 {
                eggs: 3i32,
                hatch: 1i32,
            }),
            13819u32 => Ok(GeneratedStruct35 {
                eggs: 3i32,
                hatch: 2i32,
            }),
            13820u32 => Ok(GeneratedStruct35 {
                eggs: 4i32,
                hatch: 0i32,
            }),
            13821u32 => Ok(GeneratedStruct35 {
                eggs: 4i32,
                hatch: 1i32,
            }),
            13822u32 => Ok(GeneratedStruct35 {
                eggs: 4i32,
                hatch: 2i32,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct35 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct35 {
                eggs: 1i32,
                hatch: 0i32,
            } => Ok(13811u32),
            GeneratedStruct35 {
                eggs: 1i32,
                hatch: 1i32,
            } => Ok(13812u32),
            GeneratedStruct35 {
                eggs: 1i32,
                hatch: 2i32,
            } => Ok(13813u32),
            GeneratedStruct35 {
                eggs: 2i32,
                hatch: 0i32,
            } => Ok(13814u32),
            GeneratedStruct35 {
                eggs: 2i32,
                hatch: 1i32,
            } => Ok(13815u32),
            GeneratedStruct35 {
                eggs: 2i32,
                hatch: 2i32,
            } => Ok(13816u32),
            GeneratedStruct35 {
                eggs: 3i32,
                hatch: 0i32,
            } => Ok(13817u32),
            GeneratedStruct35 {
                eggs: 3i32,
                hatch: 1i32,
            } => Ok(13818u32),
            GeneratedStruct35 {
                eggs: 3i32,
                hatch: 2i32,
            } => Ok(13819u32),
            GeneratedStruct35 {
                eggs: 4i32,
                hatch: 0i32,
            } => Ok(13820u32),
            GeneratedStruct35 {
                eggs: 4i32,
                hatch: 1i32,
            } => Ok(13821u32),
            GeneratedStruct35 {
                eggs: 4i32,
                hatch: 2i32,
            } => Ok(13822u32),
            _ => Err(()),
        }
    }
}

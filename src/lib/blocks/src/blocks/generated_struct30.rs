#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub enum GeneratedStruct30Type {
    SuspiciousGravel,
    SuspiciousSand,
}
#[allow(dead_code)]
pub struct GeneratedStruct30 {
    pub block_type: GeneratedStruct30Type,
    pub dusted: i32,
}
impl TryFrom<u32> for GeneratedStruct30 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            125u32 => Ok(GeneratedStruct30 {
                block_type: GeneratedStruct30Type::SuspiciousGravel,
                dusted: 0i32,
            }),
            126u32 => Ok(GeneratedStruct30 {
                block_type: GeneratedStruct30Type::SuspiciousGravel,
                dusted: 1i32,
            }),
            127u32 => Ok(GeneratedStruct30 {
                block_type: GeneratedStruct30Type::SuspiciousGravel,
                dusted: 2i32,
            }),
            128u32 => Ok(GeneratedStruct30 {
                block_type: GeneratedStruct30Type::SuspiciousGravel,
                dusted: 3i32,
            }),
            119u32 => Ok(GeneratedStruct30 {
                block_type: GeneratedStruct30Type::SuspiciousSand,
                dusted: 0i32,
            }),
            120u32 => Ok(GeneratedStruct30 {
                block_type: GeneratedStruct30Type::SuspiciousSand,
                dusted: 1i32,
            }),
            121u32 => Ok(GeneratedStruct30 {
                block_type: GeneratedStruct30Type::SuspiciousSand,
                dusted: 2i32,
            }),
            122u32 => Ok(GeneratedStruct30 {
                block_type: GeneratedStruct30Type::SuspiciousSand,
                dusted: 3i32,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct30 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct30 {
                block_type: GeneratedStruct30Type::SuspiciousGravel,
                dusted: 0i32,
            } => Ok(125u32),
            GeneratedStruct30 {
                block_type: GeneratedStruct30Type::SuspiciousGravel,
                dusted: 1i32,
            } => Ok(126u32),
            GeneratedStruct30 {
                block_type: GeneratedStruct30Type::SuspiciousGravel,
                dusted: 2i32,
            } => Ok(127u32),
            GeneratedStruct30 {
                block_type: GeneratedStruct30Type::SuspiciousGravel,
                dusted: 3i32,
            } => Ok(128u32),
            GeneratedStruct30 {
                block_type: GeneratedStruct30Type::SuspiciousSand,
                dusted: 0i32,
            } => Ok(119u32),
            GeneratedStruct30 {
                block_type: GeneratedStruct30Type::SuspiciousSand,
                dusted: 1i32,
            } => Ok(120u32),
            GeneratedStruct30 {
                block_type: GeneratedStruct30Type::SuspiciousSand,
                dusted: 2i32,
            } => Ok(121u32),
            GeneratedStruct30 {
                block_type: GeneratedStruct30Type::SuspiciousSand,
                dusted: 3i32,
            } => Ok(122u32),
            _ => Err(()),
        }
    }
}

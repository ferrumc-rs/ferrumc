#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub struct GeneratedStruct6 {
    pub age: i32,
    pub leaves: BambooLeaves,
    pub stage: i32,
}
impl TryFrom<u32> for GeneratedStruct6 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            13968u32 => Ok(GeneratedStruct6 {
                leaves: BambooLeaves::None,
                stage: 0i32,
                age: 0i32,
            }),
            13969u32 => Ok(GeneratedStruct6 {
                leaves: BambooLeaves::None,
                stage: 1i32,
                age: 0i32,
            }),
            13970u32 => Ok(GeneratedStruct6 {
                age: 0i32,
                stage: 0i32,
                leaves: BambooLeaves::Small,
            }),
            13971u32 => Ok(GeneratedStruct6 {
                age: 0i32,
                leaves: BambooLeaves::Small,
                stage: 1i32,
            }),
            13972u32 => Ok(GeneratedStruct6 {
                stage: 0i32,
                age: 0i32,
                leaves: BambooLeaves::Large,
            }),
            13973u32 => Ok(GeneratedStruct6 {
                age: 0i32,
                leaves: BambooLeaves::Large,
                stage: 1i32,
            }),
            13974u32 => Ok(GeneratedStruct6 {
                leaves: BambooLeaves::None,
                stage: 0i32,
                age: 1i32,
            }),
            13975u32 => Ok(GeneratedStruct6 {
                age: 1i32,
                leaves: BambooLeaves::None,
                stage: 1i32,
            }),
            13976u32 => Ok(GeneratedStruct6 {
                age: 1i32,
                stage: 0i32,
                leaves: BambooLeaves::Small,
            }),
            13977u32 => Ok(GeneratedStruct6 {
                stage: 1i32,
                age: 1i32,
                leaves: BambooLeaves::Small,
            }),
            13978u32 => Ok(GeneratedStruct6 {
                leaves: BambooLeaves::Large,
                age: 1i32,
                stage: 0i32,
            }),
            13979u32 => Ok(GeneratedStruct6 {
                leaves: BambooLeaves::Large,
                stage: 1i32,
                age: 1i32,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct6 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct6 {
                leaves: BambooLeaves::None,
                stage: 0i32,
                age: 0i32,
            } => Ok(13968u32),
            GeneratedStruct6 {
                leaves: BambooLeaves::None,
                stage: 1i32,
                age: 0i32,
            } => Ok(13969u32),
            GeneratedStruct6 {
                age: 0i32,
                stage: 0i32,
                leaves: BambooLeaves::Small,
            } => Ok(13970u32),
            GeneratedStruct6 {
                age: 0i32,
                leaves: BambooLeaves::Small,
                stage: 1i32,
            } => Ok(13971u32),
            GeneratedStruct6 {
                stage: 0i32,
                age: 0i32,
                leaves: BambooLeaves::Large,
            } => Ok(13972u32),
            GeneratedStruct6 {
                age: 0i32,
                leaves: BambooLeaves::Large,
                stage: 1i32,
            } => Ok(13973u32),
            GeneratedStruct6 {
                leaves: BambooLeaves::None,
                stage: 0i32,
                age: 1i32,
            } => Ok(13974u32),
            GeneratedStruct6 {
                age: 1i32,
                leaves: BambooLeaves::None,
                stage: 1i32,
            } => Ok(13975u32),
            GeneratedStruct6 {
                age: 1i32,
                stage: 0i32,
                leaves: BambooLeaves::Small,
            } => Ok(13976u32),
            GeneratedStruct6 {
                stage: 1i32,
                age: 1i32,
                leaves: BambooLeaves::Small,
            } => Ok(13977u32),
            GeneratedStruct6 {
                leaves: BambooLeaves::Large,
                age: 1i32,
                stage: 0i32,
            } => Ok(13978u32),
            GeneratedStruct6 {
                leaves: BambooLeaves::Large,
                stage: 1i32,
                age: 1i32,
            } => Ok(13979u32),
            _ => Err(()),
        }
    }
}

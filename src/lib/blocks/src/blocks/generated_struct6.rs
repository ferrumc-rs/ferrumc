#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GeneratedStruct6 {
    pub age: i32,
    pub leaves: BambooLeaves,
    pub stage: i32,
}
impl TryInto<u32> for GeneratedStruct6 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct6 {
                age: 0i32,
                leaves: BambooLeaves::None,
                stage: 0i32,
            } => Ok(13968u32),
            GeneratedStruct6 {
                age: 0i32,
                leaves: BambooLeaves::None,
                stage: 1i32,
            } => Ok(13969u32),
            GeneratedStruct6 {
                age: 0i32,
                leaves: BambooLeaves::Small,
                stage: 0i32,
            } => Ok(13970u32),
            GeneratedStruct6 {
                age: 0i32,
                leaves: BambooLeaves::Small,
                stage: 1i32,
            } => Ok(13971u32),
            GeneratedStruct6 {
                age: 0i32,
                leaves: BambooLeaves::Large,
                stage: 0i32,
            } => Ok(13972u32),
            GeneratedStruct6 {
                age: 0i32,
                leaves: BambooLeaves::Large,
                stage: 1i32,
            } => Ok(13973u32),
            GeneratedStruct6 {
                age: 1i32,
                leaves: BambooLeaves::None,
                stage: 0i32,
            } => Ok(13974u32),
            GeneratedStruct6 {
                age: 1i32,
                leaves: BambooLeaves::None,
                stage: 1i32,
            } => Ok(13975u32),
            GeneratedStruct6 {
                age: 1i32,
                leaves: BambooLeaves::Small,
                stage: 0i32,
            } => Ok(13976u32),
            GeneratedStruct6 {
                age: 1i32,
                leaves: BambooLeaves::Small,
                stage: 1i32,
            } => Ok(13977u32),
            GeneratedStruct6 {
                age: 1i32,
                leaves: BambooLeaves::Large,
                stage: 0i32,
            } => Ok(13978u32),
            GeneratedStruct6 {
                age: 1i32,
                leaves: BambooLeaves::Large,
                stage: 1i32,
            } => Ok(13979u32),
            _ => Err(()),
        }
    }
}

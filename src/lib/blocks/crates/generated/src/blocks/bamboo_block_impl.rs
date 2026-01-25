use crate::BambooBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for BambooBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            13968u32 => Ok(BambooBlock {
                age: 0i32,
                leaves: BambooLeaves::None,
                stage: 0i32,
            }),
            13969u32 => Ok(BambooBlock {
                age: 0i32,
                leaves: BambooLeaves::None,
                stage: 1i32,
            }),
            13970u32 => Ok(BambooBlock {
                age: 0i32,
                leaves: BambooLeaves::Small,
                stage: 0i32,
            }),
            13971u32 => Ok(BambooBlock {
                age: 0i32,
                leaves: BambooLeaves::Small,
                stage: 1i32,
            }),
            13972u32 => Ok(BambooBlock {
                age: 0i32,
                leaves: BambooLeaves::Large,
                stage: 0i32,
            }),
            13973u32 => Ok(BambooBlock {
                age: 0i32,
                leaves: BambooLeaves::Large,
                stage: 1i32,
            }),
            13974u32 => Ok(BambooBlock {
                age: 1i32,
                leaves: BambooLeaves::None,
                stage: 0i32,
            }),
            13975u32 => Ok(BambooBlock {
                age: 1i32,
                leaves: BambooLeaves::None,
                stage: 1i32,
            }),
            13976u32 => Ok(BambooBlock {
                age: 1i32,
                leaves: BambooLeaves::Small,
                stage: 0i32,
            }),
            13977u32 => Ok(BambooBlock {
                age: 1i32,
                leaves: BambooLeaves::Small,
                stage: 1i32,
            }),
            13978u32 => Ok(BambooBlock {
                age: 1i32,
                leaves: BambooLeaves::Large,
                stage: 0i32,
            }),
            13979u32 => Ok(BambooBlock {
                age: 1i32,
                leaves: BambooLeaves::Large,
                stage: 1i32,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for BambooBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            BambooBlock {
                age: 0i32,
                leaves: BambooLeaves::None,
                stage: 0i32,
            } => Ok(13968u32),
            BambooBlock {
                age: 0i32,
                leaves: BambooLeaves::None,
                stage: 1i32,
            } => Ok(13969u32),
            BambooBlock {
                age: 0i32,
                leaves: BambooLeaves::Small,
                stage: 0i32,
            } => Ok(13970u32),
            BambooBlock {
                age: 0i32,
                leaves: BambooLeaves::Small,
                stage: 1i32,
            } => Ok(13971u32),
            BambooBlock {
                age: 0i32,
                leaves: BambooLeaves::Large,
                stage: 0i32,
            } => Ok(13972u32),
            BambooBlock {
                age: 0i32,
                leaves: BambooLeaves::Large,
                stage: 1i32,
            } => Ok(13973u32),
            BambooBlock {
                age: 1i32,
                leaves: BambooLeaves::None,
                stage: 0i32,
            } => Ok(13974u32),
            BambooBlock {
                age: 1i32,
                leaves: BambooLeaves::None,
                stage: 1i32,
            } => Ok(13975u32),
            BambooBlock {
                age: 1i32,
                leaves: BambooLeaves::Small,
                stage: 0i32,
            } => Ok(13976u32),
            BambooBlock {
                age: 1i32,
                leaves: BambooLeaves::Small,
                stage: 1i32,
            } => Ok(13977u32),
            BambooBlock {
                age: 1i32,
                leaves: BambooLeaves::Large,
                stage: 0i32,
            } => Ok(13978u32),
            BambooBlock {
                age: 1i32,
                leaves: BambooLeaves::Large,
                stage: 1i32,
            } => Ok(13979u32),
            _ => Err(()),
        }
    }
}

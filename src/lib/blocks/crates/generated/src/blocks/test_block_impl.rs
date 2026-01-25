use crate::TestBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for TestBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            20395u32 => Ok(TestBlock {
                mode: TestBlockMode::Start,
            }),
            20396u32 => Ok(TestBlock {
                mode: TestBlockMode::Log,
            }),
            20397u32 => Ok(TestBlock {
                mode: TestBlockMode::Fail,
            }),
            20398u32 => Ok(TestBlock {
                mode: TestBlockMode::Accept,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for TestBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            TestBlock {
                mode: TestBlockMode::Start,
            } => Ok(20395u32),
            TestBlock {
                mode: TestBlockMode::Log,
            } => Ok(20396u32),
            TestBlock {
                mode: TestBlockMode::Fail,
            } => Ok(20397u32),
            TestBlock {
                mode: TestBlockMode::Accept,
            } => Ok(20398u32),
            _ => Err(()),
        }
    }
}

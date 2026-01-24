#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum SuspiciousBlockType {
    SuspiciousGravel,
    SuspiciousSand,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SuspiciousBlock {
    pub block_type: SuspiciousBlockType,
    pub dusted: i32,
}

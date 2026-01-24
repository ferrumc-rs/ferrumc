#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum CommandBlockType {
    ChainCommandBlock,
    CommandBlock,
    RepeatingCommandBlock,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CommandBlock {
    pub block_type: CommandBlockType,
    pub conditional: bool,
    pub facing: Direction,
}

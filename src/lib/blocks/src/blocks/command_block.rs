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
impl TryInto<u32> for CommandBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            CommandBlock {
                block_type: CommandBlockType::ChainCommandBlock,
                conditional: true,
                facing: Direction::North,
            } => Ok(13550u32),
            CommandBlock {
                block_type: CommandBlockType::ChainCommandBlock,
                conditional: true,
                facing: Direction::East,
            } => Ok(13551u32),
            CommandBlock {
                block_type: CommandBlockType::ChainCommandBlock,
                conditional: true,
                facing: Direction::South,
            } => Ok(13552u32),
            CommandBlock {
                block_type: CommandBlockType::ChainCommandBlock,
                conditional: true,
                facing: Direction::West,
            } => Ok(13553u32),
            CommandBlock {
                block_type: CommandBlockType::ChainCommandBlock,
                conditional: true,
                facing: Direction::Up,
            } => Ok(13554u32),
            CommandBlock {
                block_type: CommandBlockType::ChainCommandBlock,
                conditional: true,
                facing: Direction::Down,
            } => Ok(13555u32),
            CommandBlock {
                block_type: CommandBlockType::ChainCommandBlock,
                conditional: false,
                facing: Direction::North,
            } => Ok(13556u32),
            CommandBlock {
                block_type: CommandBlockType::ChainCommandBlock,
                conditional: false,
                facing: Direction::East,
            } => Ok(13557u32),
            CommandBlock {
                block_type: CommandBlockType::ChainCommandBlock,
                conditional: false,
                facing: Direction::South,
            } => Ok(13558u32),
            CommandBlock {
                block_type: CommandBlockType::ChainCommandBlock,
                conditional: false,
                facing: Direction::West,
            } => Ok(13559u32),
            CommandBlock {
                block_type: CommandBlockType::ChainCommandBlock,
                conditional: false,
                facing: Direction::Up,
            } => Ok(13560u32),
            CommandBlock {
                block_type: CommandBlockType::ChainCommandBlock,
                conditional: false,
                facing: Direction::Down,
            } => Ok(13561u32),
            CommandBlock {
                block_type: CommandBlockType::CommandBlock,
                conditional: true,
                facing: Direction::North,
            } => Ok(8690u32),
            CommandBlock {
                block_type: CommandBlockType::CommandBlock,
                conditional: true,
                facing: Direction::East,
            } => Ok(8691u32),
            CommandBlock {
                block_type: CommandBlockType::CommandBlock,
                conditional: true,
                facing: Direction::South,
            } => Ok(8692u32),
            CommandBlock {
                block_type: CommandBlockType::CommandBlock,
                conditional: true,
                facing: Direction::West,
            } => Ok(8693u32),
            CommandBlock {
                block_type: CommandBlockType::CommandBlock,
                conditional: true,
                facing: Direction::Up,
            } => Ok(8694u32),
            CommandBlock {
                block_type: CommandBlockType::CommandBlock,
                conditional: true,
                facing: Direction::Down,
            } => Ok(8695u32),
            CommandBlock {
                block_type: CommandBlockType::CommandBlock,
                conditional: false,
                facing: Direction::North,
            } => Ok(8696u32),
            CommandBlock {
                block_type: CommandBlockType::CommandBlock,
                conditional: false,
                facing: Direction::East,
            } => Ok(8697u32),
            CommandBlock {
                block_type: CommandBlockType::CommandBlock,
                conditional: false,
                facing: Direction::South,
            } => Ok(8698u32),
            CommandBlock {
                block_type: CommandBlockType::CommandBlock,
                conditional: false,
                facing: Direction::West,
            } => Ok(8699u32),
            CommandBlock {
                block_type: CommandBlockType::CommandBlock,
                conditional: false,
                facing: Direction::Up,
            } => Ok(8700u32),
            CommandBlock {
                block_type: CommandBlockType::CommandBlock,
                conditional: false,
                facing: Direction::Down,
            } => Ok(8701u32),
            CommandBlock {
                block_type: CommandBlockType::RepeatingCommandBlock,
                conditional: true,
                facing: Direction::North,
            } => Ok(13538u32),
            CommandBlock {
                block_type: CommandBlockType::RepeatingCommandBlock,
                conditional: true,
                facing: Direction::East,
            } => Ok(13539u32),
            CommandBlock {
                block_type: CommandBlockType::RepeatingCommandBlock,
                conditional: true,
                facing: Direction::South,
            } => Ok(13540u32),
            CommandBlock {
                block_type: CommandBlockType::RepeatingCommandBlock,
                conditional: true,
                facing: Direction::West,
            } => Ok(13541u32),
            CommandBlock {
                block_type: CommandBlockType::RepeatingCommandBlock,
                conditional: true,
                facing: Direction::Up,
            } => Ok(13542u32),
            CommandBlock {
                block_type: CommandBlockType::RepeatingCommandBlock,
                conditional: true,
                facing: Direction::Down,
            } => Ok(13543u32),
            CommandBlock {
                block_type: CommandBlockType::RepeatingCommandBlock,
                conditional: false,
                facing: Direction::North,
            } => Ok(13544u32),
            CommandBlock {
                block_type: CommandBlockType::RepeatingCommandBlock,
                conditional: false,
                facing: Direction::East,
            } => Ok(13545u32),
            CommandBlock {
                block_type: CommandBlockType::RepeatingCommandBlock,
                conditional: false,
                facing: Direction::South,
            } => Ok(13546u32),
            CommandBlock {
                block_type: CommandBlockType::RepeatingCommandBlock,
                conditional: false,
                facing: Direction::West,
            } => Ok(13547u32),
            CommandBlock {
                block_type: CommandBlockType::RepeatingCommandBlock,
                conditional: false,
                facing: Direction::Up,
            } => Ok(13548u32),
            CommandBlock {
                block_type: CommandBlockType::RepeatingCommandBlock,
                conditional: false,
                facing: Direction::Down,
            } => Ok(13549u32),
            _ => Err(()),
        }
    }
}

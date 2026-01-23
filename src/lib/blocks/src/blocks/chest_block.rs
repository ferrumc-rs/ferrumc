#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum ChestBlockType {
    Chest,
    TrappedChest,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ChestBlock {
    pub block_type: ChestBlockType,
    pub facing: Direction,
    pub ty: ChestType,
    pub waterlogged: bool,
}
impl TryInto<u32> for ChestBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            ChestBlock {
                block_type: ChestBlockType::Chest,
                facing: Direction::North,
                ty: ChestType::Single,
                waterlogged: true,
            } => Ok(3018u32),
            ChestBlock {
                block_type: ChestBlockType::Chest,
                facing: Direction::North,
                ty: ChestType::Single,
                waterlogged: false,
            } => Ok(3019u32),
            ChestBlock {
                block_type: ChestBlockType::Chest,
                facing: Direction::North,
                ty: ChestType::Left,
                waterlogged: true,
            } => Ok(3020u32),
            ChestBlock {
                block_type: ChestBlockType::Chest,
                facing: Direction::North,
                ty: ChestType::Left,
                waterlogged: false,
            } => Ok(3021u32),
            ChestBlock {
                block_type: ChestBlockType::Chest,
                facing: Direction::North,
                ty: ChestType::Right,
                waterlogged: true,
            } => Ok(3022u32),
            ChestBlock {
                block_type: ChestBlockType::Chest,
                facing: Direction::North,
                ty: ChestType::Right,
                waterlogged: false,
            } => Ok(3023u32),
            ChestBlock {
                block_type: ChestBlockType::Chest,
                facing: Direction::South,
                ty: ChestType::Single,
                waterlogged: true,
            } => Ok(3024u32),
            ChestBlock {
                block_type: ChestBlockType::Chest,
                facing: Direction::South,
                ty: ChestType::Single,
                waterlogged: false,
            } => Ok(3025u32),
            ChestBlock {
                block_type: ChestBlockType::Chest,
                facing: Direction::South,
                ty: ChestType::Left,
                waterlogged: true,
            } => Ok(3026u32),
            ChestBlock {
                block_type: ChestBlockType::Chest,
                facing: Direction::South,
                ty: ChestType::Left,
                waterlogged: false,
            } => Ok(3027u32),
            ChestBlock {
                block_type: ChestBlockType::Chest,
                facing: Direction::South,
                ty: ChestType::Right,
                waterlogged: true,
            } => Ok(3028u32),
            ChestBlock {
                block_type: ChestBlockType::Chest,
                facing: Direction::South,
                ty: ChestType::Right,
                waterlogged: false,
            } => Ok(3029u32),
            ChestBlock {
                block_type: ChestBlockType::Chest,
                facing: Direction::West,
                ty: ChestType::Single,
                waterlogged: true,
            } => Ok(3030u32),
            ChestBlock {
                block_type: ChestBlockType::Chest,
                facing: Direction::West,
                ty: ChestType::Single,
                waterlogged: false,
            } => Ok(3031u32),
            ChestBlock {
                block_type: ChestBlockType::Chest,
                facing: Direction::West,
                ty: ChestType::Left,
                waterlogged: true,
            } => Ok(3032u32),
            ChestBlock {
                block_type: ChestBlockType::Chest,
                facing: Direction::West,
                ty: ChestType::Left,
                waterlogged: false,
            } => Ok(3033u32),
            ChestBlock {
                block_type: ChestBlockType::Chest,
                facing: Direction::West,
                ty: ChestType::Right,
                waterlogged: true,
            } => Ok(3034u32),
            ChestBlock {
                block_type: ChestBlockType::Chest,
                facing: Direction::West,
                ty: ChestType::Right,
                waterlogged: false,
            } => Ok(3035u32),
            ChestBlock {
                block_type: ChestBlockType::Chest,
                facing: Direction::East,
                ty: ChestType::Single,
                waterlogged: true,
            } => Ok(3036u32),
            ChestBlock {
                block_type: ChestBlockType::Chest,
                facing: Direction::East,
                ty: ChestType::Single,
                waterlogged: false,
            } => Ok(3037u32),
            ChestBlock {
                block_type: ChestBlockType::Chest,
                facing: Direction::East,
                ty: ChestType::Left,
                waterlogged: true,
            } => Ok(3038u32),
            ChestBlock {
                block_type: ChestBlockType::Chest,
                facing: Direction::East,
                ty: ChestType::Left,
                waterlogged: false,
            } => Ok(3039u32),
            ChestBlock {
                block_type: ChestBlockType::Chest,
                facing: Direction::East,
                ty: ChestType::Right,
                waterlogged: true,
            } => Ok(3040u32),
            ChestBlock {
                block_type: ChestBlockType::Chest,
                facing: Direction::East,
                ty: ChestType::Right,
                waterlogged: false,
            } => Ok(3041u32),
            ChestBlock {
                block_type: ChestBlockType::TrappedChest,
                facing: Direction::North,
                ty: ChestType::Single,
                waterlogged: true,
            } => Ok(9928u32),
            ChestBlock {
                block_type: ChestBlockType::TrappedChest,
                facing: Direction::North,
                ty: ChestType::Single,
                waterlogged: false,
            } => Ok(9929u32),
            ChestBlock {
                block_type: ChestBlockType::TrappedChest,
                facing: Direction::North,
                ty: ChestType::Left,
                waterlogged: true,
            } => Ok(9930u32),
            ChestBlock {
                block_type: ChestBlockType::TrappedChest,
                facing: Direction::North,
                ty: ChestType::Left,
                waterlogged: false,
            } => Ok(9931u32),
            ChestBlock {
                block_type: ChestBlockType::TrappedChest,
                facing: Direction::North,
                ty: ChestType::Right,
                waterlogged: true,
            } => Ok(9932u32),
            ChestBlock {
                block_type: ChestBlockType::TrappedChest,
                facing: Direction::North,
                ty: ChestType::Right,
                waterlogged: false,
            } => Ok(9933u32),
            ChestBlock {
                block_type: ChestBlockType::TrappedChest,
                facing: Direction::South,
                ty: ChestType::Single,
                waterlogged: true,
            } => Ok(9934u32),
            ChestBlock {
                block_type: ChestBlockType::TrappedChest,
                facing: Direction::South,
                ty: ChestType::Single,
                waterlogged: false,
            } => Ok(9935u32),
            ChestBlock {
                block_type: ChestBlockType::TrappedChest,
                facing: Direction::South,
                ty: ChestType::Left,
                waterlogged: true,
            } => Ok(9936u32),
            ChestBlock {
                block_type: ChestBlockType::TrappedChest,
                facing: Direction::South,
                ty: ChestType::Left,
                waterlogged: false,
            } => Ok(9937u32),
            ChestBlock {
                block_type: ChestBlockType::TrappedChest,
                facing: Direction::South,
                ty: ChestType::Right,
                waterlogged: true,
            } => Ok(9938u32),
            ChestBlock {
                block_type: ChestBlockType::TrappedChest,
                facing: Direction::South,
                ty: ChestType::Right,
                waterlogged: false,
            } => Ok(9939u32),
            ChestBlock {
                block_type: ChestBlockType::TrappedChest,
                facing: Direction::West,
                ty: ChestType::Single,
                waterlogged: true,
            } => Ok(9940u32),
            ChestBlock {
                block_type: ChestBlockType::TrappedChest,
                facing: Direction::West,
                ty: ChestType::Single,
                waterlogged: false,
            } => Ok(9941u32),
            ChestBlock {
                block_type: ChestBlockType::TrappedChest,
                facing: Direction::West,
                ty: ChestType::Left,
                waterlogged: true,
            } => Ok(9942u32),
            ChestBlock {
                block_type: ChestBlockType::TrappedChest,
                facing: Direction::West,
                ty: ChestType::Left,
                waterlogged: false,
            } => Ok(9943u32),
            ChestBlock {
                block_type: ChestBlockType::TrappedChest,
                facing: Direction::West,
                ty: ChestType::Right,
                waterlogged: true,
            } => Ok(9944u32),
            ChestBlock {
                block_type: ChestBlockType::TrappedChest,
                facing: Direction::West,
                ty: ChestType::Right,
                waterlogged: false,
            } => Ok(9945u32),
            ChestBlock {
                block_type: ChestBlockType::TrappedChest,
                facing: Direction::East,
                ty: ChestType::Single,
                waterlogged: true,
            } => Ok(9946u32),
            ChestBlock {
                block_type: ChestBlockType::TrappedChest,
                facing: Direction::East,
                ty: ChestType::Single,
                waterlogged: false,
            } => Ok(9947u32),
            ChestBlock {
                block_type: ChestBlockType::TrappedChest,
                facing: Direction::East,
                ty: ChestType::Left,
                waterlogged: true,
            } => Ok(9948u32),
            ChestBlock {
                block_type: ChestBlockType::TrappedChest,
                facing: Direction::East,
                ty: ChestType::Left,
                waterlogged: false,
            } => Ok(9949u32),
            ChestBlock {
                block_type: ChestBlockType::TrappedChest,
                facing: Direction::East,
                ty: ChestType::Right,
                waterlogged: true,
            } => Ok(9950u32),
            ChestBlock {
                block_type: ChestBlockType::TrappedChest,
                facing: Direction::East,
                ty: ChestType::Right,
                waterlogged: false,
            } => Ok(9951u32),
            _ => Err(()),
        }
    }
}

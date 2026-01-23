#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum FurnaceBlockType {
    BlastFurnace,
    Furnace,
    RedstoneWallTorch,
    Smoker,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct FurnaceBlock {
    pub block_type: FurnaceBlockType,
    pub facing: Direction,
    pub lit: bool,
}
impl TryInto<u32> for FurnaceBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            FurnaceBlock {
                block_type: FurnaceBlockType::BlastFurnace,
                facing: Direction::North,
                lit: true,
            } => Ok(19451u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::BlastFurnace,
                facing: Direction::North,
                lit: false,
            } => Ok(19452u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::BlastFurnace,
                facing: Direction::South,
                lit: true,
            } => Ok(19453u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::BlastFurnace,
                facing: Direction::South,
                lit: false,
            } => Ok(19454u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::BlastFurnace,
                facing: Direction::West,
                lit: true,
            } => Ok(19455u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::BlastFurnace,
                facing: Direction::West,
                lit: false,
            } => Ok(19456u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::BlastFurnace,
                facing: Direction::East,
                lit: true,
            } => Ok(19457u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::BlastFurnace,
                facing: Direction::East,
                lit: false,
            } => Ok(19458u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::Furnace,
                facing: Direction::North,
                lit: true,
            } => Ok(4358u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::Furnace,
                facing: Direction::North,
                lit: false,
            } => Ok(4359u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::Furnace,
                facing: Direction::South,
                lit: true,
            } => Ok(4360u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::Furnace,
                facing: Direction::South,
                lit: false,
            } => Ok(4361u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::Furnace,
                facing: Direction::West,
                lit: true,
            } => Ok(4362u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::Furnace,
                facing: Direction::West,
                lit: false,
            } => Ok(4363u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::Furnace,
                facing: Direction::East,
                lit: true,
            } => Ok(4364u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::Furnace,
                facing: Direction::East,
                lit: false,
            } => Ok(4365u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::RedstoneWallTorch,
                facing: Direction::North,
                lit: true,
            } => Ok(5918u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::RedstoneWallTorch,
                facing: Direction::North,
                lit: false,
            } => Ok(5919u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::RedstoneWallTorch,
                facing: Direction::South,
                lit: true,
            } => Ok(5920u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::RedstoneWallTorch,
                facing: Direction::South,
                lit: false,
            } => Ok(5921u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::RedstoneWallTorch,
                facing: Direction::West,
                lit: true,
            } => Ok(5922u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::RedstoneWallTorch,
                facing: Direction::West,
                lit: false,
            } => Ok(5923u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::RedstoneWallTorch,
                facing: Direction::East,
                lit: true,
            } => Ok(5924u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::RedstoneWallTorch,
                facing: Direction::East,
                lit: false,
            } => Ok(5925u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::Smoker,
                facing: Direction::North,
                lit: true,
            } => Ok(19443u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::Smoker,
                facing: Direction::North,
                lit: false,
            } => Ok(19444u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::Smoker,
                facing: Direction::South,
                lit: true,
            } => Ok(19445u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::Smoker,
                facing: Direction::South,
                lit: false,
            } => Ok(19446u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::Smoker,
                facing: Direction::West,
                lit: true,
            } => Ok(19447u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::Smoker,
                facing: Direction::West,
                lit: false,
            } => Ok(19448u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::Smoker,
                facing: Direction::East,
                lit: true,
            } => Ok(19449u32),
            FurnaceBlock {
                block_type: FurnaceBlockType::Smoker,
                facing: Direction::East,
                lit: false,
            } => Ok(19450u32),
            _ => Err(()),
        }
    }
}

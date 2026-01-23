#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum DispenserBlockType {
    Dispenser,
    Dropper,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct DispenserBlock {
    pub block_type: DispenserBlockType,
    pub facing: Direction,
    pub triggered: bool,
}
impl TryInto<u32> for DispenserBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            DispenserBlock {
                block_type: DispenserBlockType::Dispenser,
                facing: Direction::North,
                triggered: true,
            } => Ok(566u32),
            DispenserBlock {
                block_type: DispenserBlockType::Dispenser,
                facing: Direction::North,
                triggered: false,
            } => Ok(567u32),
            DispenserBlock {
                block_type: DispenserBlockType::Dispenser,
                facing: Direction::East,
                triggered: true,
            } => Ok(568u32),
            DispenserBlock {
                block_type: DispenserBlockType::Dispenser,
                facing: Direction::East,
                triggered: false,
            } => Ok(569u32),
            DispenserBlock {
                block_type: DispenserBlockType::Dispenser,
                facing: Direction::South,
                triggered: true,
            } => Ok(570u32),
            DispenserBlock {
                block_type: DispenserBlockType::Dispenser,
                facing: Direction::South,
                triggered: false,
            } => Ok(571u32),
            DispenserBlock {
                block_type: DispenserBlockType::Dispenser,
                facing: Direction::West,
                triggered: true,
            } => Ok(572u32),
            DispenserBlock {
                block_type: DispenserBlockType::Dispenser,
                facing: Direction::West,
                triggered: false,
            } => Ok(573u32),
            DispenserBlock {
                block_type: DispenserBlockType::Dispenser,
                facing: Direction::Up,
                triggered: true,
            } => Ok(574u32),
            DispenserBlock {
                block_type: DispenserBlockType::Dispenser,
                facing: Direction::Up,
                triggered: false,
            } => Ok(575u32),
            DispenserBlock {
                block_type: DispenserBlockType::Dispenser,
                facing: Direction::Down,
                triggered: true,
            } => Ok(576u32),
            DispenserBlock {
                block_type: DispenserBlockType::Dispenser,
                facing: Direction::Down,
                triggered: false,
            } => Ok(577u32),
            DispenserBlock {
                block_type: DispenserBlockType::Dropper,
                facing: Direction::North,
                triggered: true,
            } => Ok(10153u32),
            DispenserBlock {
                block_type: DispenserBlockType::Dropper,
                facing: Direction::North,
                triggered: false,
            } => Ok(10154u32),
            DispenserBlock {
                block_type: DispenserBlockType::Dropper,
                facing: Direction::East,
                triggered: true,
            } => Ok(10155u32),
            DispenserBlock {
                block_type: DispenserBlockType::Dropper,
                facing: Direction::East,
                triggered: false,
            } => Ok(10156u32),
            DispenserBlock {
                block_type: DispenserBlockType::Dropper,
                facing: Direction::South,
                triggered: true,
            } => Ok(10157u32),
            DispenserBlock {
                block_type: DispenserBlockType::Dropper,
                facing: Direction::South,
                triggered: false,
            } => Ok(10158u32),
            DispenserBlock {
                block_type: DispenserBlockType::Dropper,
                facing: Direction::West,
                triggered: true,
            } => Ok(10159u32),
            DispenserBlock {
                block_type: DispenserBlockType::Dropper,
                facing: Direction::West,
                triggered: false,
            } => Ok(10160u32),
            DispenserBlock {
                block_type: DispenserBlockType::Dropper,
                facing: Direction::Up,
                triggered: true,
            } => Ok(10161u32),
            DispenserBlock {
                block_type: DispenserBlockType::Dropper,
                facing: Direction::Up,
                triggered: false,
            } => Ok(10162u32),
            DispenserBlock {
                block_type: DispenserBlockType::Dropper,
                facing: Direction::Down,
                triggered: true,
            } => Ok(10163u32),
            DispenserBlock {
                block_type: DispenserBlockType::Dropper,
                facing: Direction::Down,
                triggered: false,
            } => Ok(10164u32),
            _ => Err(()),
        }
    }
}

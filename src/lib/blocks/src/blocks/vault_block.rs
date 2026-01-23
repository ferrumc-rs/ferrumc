#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct VaultBlock {
    pub facing: Direction,
    pub ominous: bool,
    pub vault_state: VaultState,
}
impl TryInto<u32> for VaultBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            VaultBlock {
                facing: Direction::North,
                ominous: true,
                vault_state: VaultState::Inactive,
            } => Ok(27710u32),
            VaultBlock {
                facing: Direction::North,
                ominous: true,
                vault_state: VaultState::Active,
            } => Ok(27711u32),
            VaultBlock {
                facing: Direction::North,
                ominous: true,
                vault_state: VaultState::Unlocking,
            } => Ok(27712u32),
            VaultBlock {
                facing: Direction::North,
                ominous: true,
                vault_state: VaultState::Ejecting,
            } => Ok(27713u32),
            VaultBlock {
                facing: Direction::North,
                ominous: false,
                vault_state: VaultState::Inactive,
            } => Ok(27714u32),
            VaultBlock {
                facing: Direction::North,
                ominous: false,
                vault_state: VaultState::Active,
            } => Ok(27715u32),
            VaultBlock {
                facing: Direction::North,
                ominous: false,
                vault_state: VaultState::Unlocking,
            } => Ok(27716u32),
            VaultBlock {
                facing: Direction::North,
                ominous: false,
                vault_state: VaultState::Ejecting,
            } => Ok(27717u32),
            VaultBlock {
                facing: Direction::South,
                ominous: true,
                vault_state: VaultState::Inactive,
            } => Ok(27718u32),
            VaultBlock {
                facing: Direction::South,
                ominous: true,
                vault_state: VaultState::Active,
            } => Ok(27719u32),
            VaultBlock {
                facing: Direction::South,
                ominous: true,
                vault_state: VaultState::Unlocking,
            } => Ok(27720u32),
            VaultBlock {
                facing: Direction::South,
                ominous: true,
                vault_state: VaultState::Ejecting,
            } => Ok(27721u32),
            VaultBlock {
                facing: Direction::South,
                ominous: false,
                vault_state: VaultState::Inactive,
            } => Ok(27722u32),
            VaultBlock {
                facing: Direction::South,
                ominous: false,
                vault_state: VaultState::Active,
            } => Ok(27723u32),
            VaultBlock {
                facing: Direction::South,
                ominous: false,
                vault_state: VaultState::Unlocking,
            } => Ok(27724u32),
            VaultBlock {
                facing: Direction::South,
                ominous: false,
                vault_state: VaultState::Ejecting,
            } => Ok(27725u32),
            VaultBlock {
                facing: Direction::West,
                ominous: true,
                vault_state: VaultState::Inactive,
            } => Ok(27726u32),
            VaultBlock {
                facing: Direction::West,
                ominous: true,
                vault_state: VaultState::Active,
            } => Ok(27727u32),
            VaultBlock {
                facing: Direction::West,
                ominous: true,
                vault_state: VaultState::Unlocking,
            } => Ok(27728u32),
            VaultBlock {
                facing: Direction::West,
                ominous: true,
                vault_state: VaultState::Ejecting,
            } => Ok(27729u32),
            VaultBlock {
                facing: Direction::West,
                ominous: false,
                vault_state: VaultState::Inactive,
            } => Ok(27730u32),
            VaultBlock {
                facing: Direction::West,
                ominous: false,
                vault_state: VaultState::Active,
            } => Ok(27731u32),
            VaultBlock {
                facing: Direction::West,
                ominous: false,
                vault_state: VaultState::Unlocking,
            } => Ok(27732u32),
            VaultBlock {
                facing: Direction::West,
                ominous: false,
                vault_state: VaultState::Ejecting,
            } => Ok(27733u32),
            VaultBlock {
                facing: Direction::East,
                ominous: true,
                vault_state: VaultState::Inactive,
            } => Ok(27734u32),
            VaultBlock {
                facing: Direction::East,
                ominous: true,
                vault_state: VaultState::Active,
            } => Ok(27735u32),
            VaultBlock {
                facing: Direction::East,
                ominous: true,
                vault_state: VaultState::Unlocking,
            } => Ok(27736u32),
            VaultBlock {
                facing: Direction::East,
                ominous: true,
                vault_state: VaultState::Ejecting,
            } => Ok(27737u32),
            VaultBlock {
                facing: Direction::East,
                ominous: false,
                vault_state: VaultState::Inactive,
            } => Ok(27738u32),
            VaultBlock {
                facing: Direction::East,
                ominous: false,
                vault_state: VaultState::Active,
            } => Ok(27739u32),
            VaultBlock {
                facing: Direction::East,
                ominous: false,
                vault_state: VaultState::Unlocking,
            } => Ok(27740u32),
            VaultBlock {
                facing: Direction::East,
                ominous: false,
                vault_state: VaultState::Ejecting,
            } => Ok(27741u32),
            _ => Err(()),
        }
    }
}

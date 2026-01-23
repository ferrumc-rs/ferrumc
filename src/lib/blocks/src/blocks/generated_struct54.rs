#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub struct GeneratedStruct54 {
    pub facing: Direction,
    pub ominous: bool,
    pub vault_state: VaultState,
}
impl TryFrom<u32> for GeneratedStruct54 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            27710u32 => Ok(GeneratedStruct54 {
                vault_state: VaultState::Inactive,
                facing: Direction::North,
                ominous: true,
            }),
            27711u32 => Ok(GeneratedStruct54 {
                vault_state: VaultState::Active,
                facing: Direction::North,
                ominous: true,
            }),
            27712u32 => Ok(GeneratedStruct54 {
                facing: Direction::North,
                vault_state: VaultState::Unlocking,
                ominous: true,
            }),
            27713u32 => Ok(GeneratedStruct54 {
                vault_state: VaultState::Ejecting,
                facing: Direction::North,
                ominous: true,
            }),
            27714u32 => Ok(GeneratedStruct54 {
                ominous: false,
                vault_state: VaultState::Inactive,
                facing: Direction::North,
            }),
            27715u32 => Ok(GeneratedStruct54 {
                ominous: false,
                vault_state: VaultState::Active,
                facing: Direction::North,
            }),
            27716u32 => Ok(GeneratedStruct54 {
                ominous: false,
                vault_state: VaultState::Unlocking,
                facing: Direction::North,
            }),
            27717u32 => Ok(GeneratedStruct54 {
                vault_state: VaultState::Ejecting,
                facing: Direction::North,
                ominous: false,
            }),
            27718u32 => Ok(GeneratedStruct54 {
                vault_state: VaultState::Inactive,
                facing: Direction::South,
                ominous: true,
            }),
            27719u32 => Ok(GeneratedStruct54 {
                facing: Direction::South,
                ominous: true,
                vault_state: VaultState::Active,
            }),
            27720u32 => Ok(GeneratedStruct54 {
                facing: Direction::South,
                vault_state: VaultState::Unlocking,
                ominous: true,
            }),
            27721u32 => Ok(GeneratedStruct54 {
                vault_state: VaultState::Ejecting,
                facing: Direction::South,
                ominous: true,
            }),
            27722u32 => Ok(GeneratedStruct54 {
                vault_state: VaultState::Inactive,
                ominous: false,
                facing: Direction::South,
            }),
            27723u32 => Ok(GeneratedStruct54 {
                facing: Direction::South,
                vault_state: VaultState::Active,
                ominous: false,
            }),
            27724u32 => Ok(GeneratedStruct54 {
                vault_state: VaultState::Unlocking,
                facing: Direction::South,
                ominous: false,
            }),
            27725u32 => Ok(GeneratedStruct54 {
                vault_state: VaultState::Ejecting,
                facing: Direction::South,
                ominous: false,
            }),
            27726u32 => Ok(GeneratedStruct54 {
                vault_state: VaultState::Inactive,
                facing: Direction::West,
                ominous: true,
            }),
            27727u32 => Ok(GeneratedStruct54 {
                ominous: true,
                vault_state: VaultState::Active,
                facing: Direction::West,
            }),
            27728u32 => Ok(GeneratedStruct54 {
                vault_state: VaultState::Unlocking,
                ominous: true,
                facing: Direction::West,
            }),
            27729u32 => Ok(GeneratedStruct54 {
                vault_state: VaultState::Ejecting,
                facing: Direction::West,
                ominous: true,
            }),
            27730u32 => Ok(GeneratedStruct54 {
                vault_state: VaultState::Inactive,
                facing: Direction::West,
                ominous: false,
            }),
            27731u32 => Ok(GeneratedStruct54 {
                ominous: false,
                facing: Direction::West,
                vault_state: VaultState::Active,
            }),
            27732u32 => Ok(GeneratedStruct54 {
                vault_state: VaultState::Unlocking,
                facing: Direction::West,
                ominous: false,
            }),
            27733u32 => Ok(GeneratedStruct54 {
                ominous: false,
                facing: Direction::West,
                vault_state: VaultState::Ejecting,
            }),
            27734u32 => Ok(GeneratedStruct54 {
                facing: Direction::East,
                ominous: true,
                vault_state: VaultState::Inactive,
            }),
            27735u32 => Ok(GeneratedStruct54 {
                ominous: true,
                facing: Direction::East,
                vault_state: VaultState::Active,
            }),
            27736u32 => Ok(GeneratedStruct54 {
                facing: Direction::East,
                ominous: true,
                vault_state: VaultState::Unlocking,
            }),
            27737u32 => Ok(GeneratedStruct54 {
                ominous: true,
                facing: Direction::East,
                vault_state: VaultState::Ejecting,
            }),
            27738u32 => Ok(GeneratedStruct54 {
                ominous: false,
                facing: Direction::East,
                vault_state: VaultState::Inactive,
            }),
            27739u32 => Ok(GeneratedStruct54 {
                ominous: false,
                vault_state: VaultState::Active,
                facing: Direction::East,
            }),
            27740u32 => Ok(GeneratedStruct54 {
                ominous: false,
                vault_state: VaultState::Unlocking,
                facing: Direction::East,
            }),
            27741u32 => Ok(GeneratedStruct54 {
                vault_state: VaultState::Ejecting,
                facing: Direction::East,
                ominous: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct54 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct54 {
                vault_state: VaultState::Inactive,
                facing: Direction::North,
                ominous: true,
            } => Ok(27710u32),
            GeneratedStruct54 {
                vault_state: VaultState::Active,
                facing: Direction::North,
                ominous: true,
            } => Ok(27711u32),
            GeneratedStruct54 {
                facing: Direction::North,
                vault_state: VaultState::Unlocking,
                ominous: true,
            } => Ok(27712u32),
            GeneratedStruct54 {
                vault_state: VaultState::Ejecting,
                facing: Direction::North,
                ominous: true,
            } => Ok(27713u32),
            GeneratedStruct54 {
                ominous: false,
                vault_state: VaultState::Inactive,
                facing: Direction::North,
            } => Ok(27714u32),
            GeneratedStruct54 {
                ominous: false,
                vault_state: VaultState::Active,
                facing: Direction::North,
            } => Ok(27715u32),
            GeneratedStruct54 {
                ominous: false,
                vault_state: VaultState::Unlocking,
                facing: Direction::North,
            } => Ok(27716u32),
            GeneratedStruct54 {
                vault_state: VaultState::Ejecting,
                facing: Direction::North,
                ominous: false,
            } => Ok(27717u32),
            GeneratedStruct54 {
                vault_state: VaultState::Inactive,
                facing: Direction::South,
                ominous: true,
            } => Ok(27718u32),
            GeneratedStruct54 {
                facing: Direction::South,
                ominous: true,
                vault_state: VaultState::Active,
            } => Ok(27719u32),
            GeneratedStruct54 {
                facing: Direction::South,
                vault_state: VaultState::Unlocking,
                ominous: true,
            } => Ok(27720u32),
            GeneratedStruct54 {
                vault_state: VaultState::Ejecting,
                facing: Direction::South,
                ominous: true,
            } => Ok(27721u32),
            GeneratedStruct54 {
                vault_state: VaultState::Inactive,
                ominous: false,
                facing: Direction::South,
            } => Ok(27722u32),
            GeneratedStruct54 {
                facing: Direction::South,
                vault_state: VaultState::Active,
                ominous: false,
            } => Ok(27723u32),
            GeneratedStruct54 {
                vault_state: VaultState::Unlocking,
                facing: Direction::South,
                ominous: false,
            } => Ok(27724u32),
            GeneratedStruct54 {
                vault_state: VaultState::Ejecting,
                facing: Direction::South,
                ominous: false,
            } => Ok(27725u32),
            GeneratedStruct54 {
                vault_state: VaultState::Inactive,
                facing: Direction::West,
                ominous: true,
            } => Ok(27726u32),
            GeneratedStruct54 {
                ominous: true,
                vault_state: VaultState::Active,
                facing: Direction::West,
            } => Ok(27727u32),
            GeneratedStruct54 {
                vault_state: VaultState::Unlocking,
                ominous: true,
                facing: Direction::West,
            } => Ok(27728u32),
            GeneratedStruct54 {
                vault_state: VaultState::Ejecting,
                facing: Direction::West,
                ominous: true,
            } => Ok(27729u32),
            GeneratedStruct54 {
                vault_state: VaultState::Inactive,
                facing: Direction::West,
                ominous: false,
            } => Ok(27730u32),
            GeneratedStruct54 {
                ominous: false,
                facing: Direction::West,
                vault_state: VaultState::Active,
            } => Ok(27731u32),
            GeneratedStruct54 {
                vault_state: VaultState::Unlocking,
                facing: Direction::West,
                ominous: false,
            } => Ok(27732u32),
            GeneratedStruct54 {
                ominous: false,
                facing: Direction::West,
                vault_state: VaultState::Ejecting,
            } => Ok(27733u32),
            GeneratedStruct54 {
                facing: Direction::East,
                ominous: true,
                vault_state: VaultState::Inactive,
            } => Ok(27734u32),
            GeneratedStruct54 {
                ominous: true,
                facing: Direction::East,
                vault_state: VaultState::Active,
            } => Ok(27735u32),
            GeneratedStruct54 {
                facing: Direction::East,
                ominous: true,
                vault_state: VaultState::Unlocking,
            } => Ok(27736u32),
            GeneratedStruct54 {
                ominous: true,
                facing: Direction::East,
                vault_state: VaultState::Ejecting,
            } => Ok(27737u32),
            GeneratedStruct54 {
                ominous: false,
                facing: Direction::East,
                vault_state: VaultState::Inactive,
            } => Ok(27738u32),
            GeneratedStruct54 {
                ominous: false,
                vault_state: VaultState::Active,
                facing: Direction::East,
            } => Ok(27739u32),
            GeneratedStruct54 {
                ominous: false,
                vault_state: VaultState::Unlocking,
                facing: Direction::East,
            } => Ok(27740u32),
            GeneratedStruct54 {
                vault_state: VaultState::Ejecting,
                facing: Direction::East,
                ominous: false,
            } => Ok(27741u32),
            _ => Err(()),
        }
    }
}

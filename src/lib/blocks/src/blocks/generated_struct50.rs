#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub enum GeneratedStruct50Type {
    BlastFurnace,
    Furnace,
    RedstoneWallTorch,
    Smoker,
}
#[allow(dead_code)]
pub struct GeneratedStruct50 {
    pub block_type: GeneratedStruct50Type,
    pub facing: Direction,
    pub lit: bool,
}
impl TryFrom<u32> for GeneratedStruct50 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            19451u32 => Ok(GeneratedStruct50 {
                block_type: GeneratedStruct50Type::BlastFurnace,
                facing: Direction::North,
                lit: true,
            }),
            19452u32 => Ok(GeneratedStruct50 {
                block_type: GeneratedStruct50Type::BlastFurnace,
                facing: Direction::North,
                lit: false,
            }),
            19453u32 => Ok(GeneratedStruct50 {
                block_type: GeneratedStruct50Type::BlastFurnace,
                facing: Direction::South,
                lit: true,
            }),
            19454u32 => Ok(GeneratedStruct50 {
                block_type: GeneratedStruct50Type::BlastFurnace,
                lit: false,
                facing: Direction::South,
            }),
            19455u32 => Ok(GeneratedStruct50 {
                block_type: GeneratedStruct50Type::BlastFurnace,
                facing: Direction::West,
                lit: true,
            }),
            19456u32 => Ok(GeneratedStruct50 {
                block_type: GeneratedStruct50Type::BlastFurnace,
                facing: Direction::West,
                lit: false,
            }),
            19457u32 => Ok(GeneratedStruct50 {
                block_type: GeneratedStruct50Type::BlastFurnace,
                facing: Direction::East,
                lit: true,
            }),
            19458u32 => Ok(GeneratedStruct50 {
                block_type: GeneratedStruct50Type::BlastFurnace,
                lit: false,
                facing: Direction::East,
            }),
            4358u32 => Ok(GeneratedStruct50 {
                block_type: GeneratedStruct50Type::Furnace,
                facing: Direction::North,
                lit: true,
            }),
            4359u32 => Ok(GeneratedStruct50 {
                block_type: GeneratedStruct50Type::Furnace,
                facing: Direction::North,
                lit: false,
            }),
            4360u32 => Ok(GeneratedStruct50 {
                block_type: GeneratedStruct50Type::Furnace,
                facing: Direction::South,
                lit: true,
            }),
            4361u32 => Ok(GeneratedStruct50 {
                block_type: GeneratedStruct50Type::Furnace,
                facing: Direction::South,
                lit: false,
            }),
            4362u32 => Ok(GeneratedStruct50 {
                block_type: GeneratedStruct50Type::Furnace,
                lit: true,
                facing: Direction::West,
            }),
            4363u32 => Ok(GeneratedStruct50 {
                block_type: GeneratedStruct50Type::Furnace,
                facing: Direction::West,
                lit: false,
            }),
            4364u32 => Ok(GeneratedStruct50 {
                block_type: GeneratedStruct50Type::Furnace,
                lit: true,
                facing: Direction::East,
            }),
            4365u32 => Ok(GeneratedStruct50 {
                block_type: GeneratedStruct50Type::Furnace,
                facing: Direction::East,
                lit: false,
            }),
            5918u32 => Ok(GeneratedStruct50 {
                block_type: GeneratedStruct50Type::RedstoneWallTorch,
                lit: true,
                facing: Direction::North,
            }),
            5919u32 => Ok(GeneratedStruct50 {
                block_type: GeneratedStruct50Type::RedstoneWallTorch,
                lit: false,
                facing: Direction::North,
            }),
            5920u32 => Ok(GeneratedStruct50 {
                block_type: GeneratedStruct50Type::RedstoneWallTorch,
                facing: Direction::South,
                lit: true,
            }),
            5921u32 => Ok(GeneratedStruct50 {
                block_type: GeneratedStruct50Type::RedstoneWallTorch,
                facing: Direction::South,
                lit: false,
            }),
            5922u32 => Ok(GeneratedStruct50 {
                block_type: GeneratedStruct50Type::RedstoneWallTorch,
                lit: true,
                facing: Direction::West,
            }),
            5923u32 => Ok(GeneratedStruct50 {
                block_type: GeneratedStruct50Type::RedstoneWallTorch,
                facing: Direction::West,
                lit: false,
            }),
            5924u32 => Ok(GeneratedStruct50 {
                block_type: GeneratedStruct50Type::RedstoneWallTorch,
                lit: true,
                facing: Direction::East,
            }),
            5925u32 => Ok(GeneratedStruct50 {
                block_type: GeneratedStruct50Type::RedstoneWallTorch,
                facing: Direction::East,
                lit: false,
            }),
            19443u32 => Ok(GeneratedStruct50 {
                block_type: GeneratedStruct50Type::Smoker,
                facing: Direction::North,
                lit: true,
            }),
            19444u32 => Ok(GeneratedStruct50 {
                block_type: GeneratedStruct50Type::Smoker,
                facing: Direction::North,
                lit: false,
            }),
            19445u32 => Ok(GeneratedStruct50 {
                block_type: GeneratedStruct50Type::Smoker,
                facing: Direction::South,
                lit: true,
            }),
            19446u32 => Ok(GeneratedStruct50 {
                block_type: GeneratedStruct50Type::Smoker,
                facing: Direction::South,
                lit: false,
            }),
            19447u32 => Ok(GeneratedStruct50 {
                block_type: GeneratedStruct50Type::Smoker,
                lit: true,
                facing: Direction::West,
            }),
            19448u32 => Ok(GeneratedStruct50 {
                block_type: GeneratedStruct50Type::Smoker,
                facing: Direction::West,
                lit: false,
            }),
            19449u32 => Ok(GeneratedStruct50 {
                block_type: GeneratedStruct50Type::Smoker,
                lit: true,
                facing: Direction::East,
            }),
            19450u32 => Ok(GeneratedStruct50 {
                block_type: GeneratedStruct50Type::Smoker,
                facing: Direction::East,
                lit: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct50 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct50 {
                block_type: GeneratedStruct50Type::BlastFurnace,
                facing: Direction::North,
                lit: true,
            } => Ok(19451u32),
            GeneratedStruct50 {
                block_type: GeneratedStruct50Type::BlastFurnace,
                facing: Direction::North,
                lit: false,
            } => Ok(19452u32),
            GeneratedStruct50 {
                block_type: GeneratedStruct50Type::BlastFurnace,
                facing: Direction::South,
                lit: true,
            } => Ok(19453u32),
            GeneratedStruct50 {
                block_type: GeneratedStruct50Type::BlastFurnace,
                lit: false,
                facing: Direction::South,
            } => Ok(19454u32),
            GeneratedStruct50 {
                block_type: GeneratedStruct50Type::BlastFurnace,
                facing: Direction::West,
                lit: true,
            } => Ok(19455u32),
            GeneratedStruct50 {
                block_type: GeneratedStruct50Type::BlastFurnace,
                facing: Direction::West,
                lit: false,
            } => Ok(19456u32),
            GeneratedStruct50 {
                block_type: GeneratedStruct50Type::BlastFurnace,
                facing: Direction::East,
                lit: true,
            } => Ok(19457u32),
            GeneratedStruct50 {
                block_type: GeneratedStruct50Type::BlastFurnace,
                lit: false,
                facing: Direction::East,
            } => Ok(19458u32),
            GeneratedStruct50 {
                block_type: GeneratedStruct50Type::Furnace,
                facing: Direction::North,
                lit: true,
            } => Ok(4358u32),
            GeneratedStruct50 {
                block_type: GeneratedStruct50Type::Furnace,
                facing: Direction::North,
                lit: false,
            } => Ok(4359u32),
            GeneratedStruct50 {
                block_type: GeneratedStruct50Type::Furnace,
                facing: Direction::South,
                lit: true,
            } => Ok(4360u32),
            GeneratedStruct50 {
                block_type: GeneratedStruct50Type::Furnace,
                facing: Direction::South,
                lit: false,
            } => Ok(4361u32),
            GeneratedStruct50 {
                block_type: GeneratedStruct50Type::Furnace,
                lit: true,
                facing: Direction::West,
            } => Ok(4362u32),
            GeneratedStruct50 {
                block_type: GeneratedStruct50Type::Furnace,
                facing: Direction::West,
                lit: false,
            } => Ok(4363u32),
            GeneratedStruct50 {
                block_type: GeneratedStruct50Type::Furnace,
                lit: true,
                facing: Direction::East,
            } => Ok(4364u32),
            GeneratedStruct50 {
                block_type: GeneratedStruct50Type::Furnace,
                facing: Direction::East,
                lit: false,
            } => Ok(4365u32),
            GeneratedStruct50 {
                block_type: GeneratedStruct50Type::RedstoneWallTorch,
                lit: true,
                facing: Direction::North,
            } => Ok(5918u32),
            GeneratedStruct50 {
                block_type: GeneratedStruct50Type::RedstoneWallTorch,
                lit: false,
                facing: Direction::North,
            } => Ok(5919u32),
            GeneratedStruct50 {
                block_type: GeneratedStruct50Type::RedstoneWallTorch,
                facing: Direction::South,
                lit: true,
            } => Ok(5920u32),
            GeneratedStruct50 {
                block_type: GeneratedStruct50Type::RedstoneWallTorch,
                facing: Direction::South,
                lit: false,
            } => Ok(5921u32),
            GeneratedStruct50 {
                block_type: GeneratedStruct50Type::RedstoneWallTorch,
                lit: true,
                facing: Direction::West,
            } => Ok(5922u32),
            GeneratedStruct50 {
                block_type: GeneratedStruct50Type::RedstoneWallTorch,
                facing: Direction::West,
                lit: false,
            } => Ok(5923u32),
            GeneratedStruct50 {
                block_type: GeneratedStruct50Type::RedstoneWallTorch,
                lit: true,
                facing: Direction::East,
            } => Ok(5924u32),
            GeneratedStruct50 {
                block_type: GeneratedStruct50Type::RedstoneWallTorch,
                facing: Direction::East,
                lit: false,
            } => Ok(5925u32),
            GeneratedStruct50 {
                block_type: GeneratedStruct50Type::Smoker,
                facing: Direction::North,
                lit: true,
            } => Ok(19443u32),
            GeneratedStruct50 {
                block_type: GeneratedStruct50Type::Smoker,
                facing: Direction::North,
                lit: false,
            } => Ok(19444u32),
            GeneratedStruct50 {
                block_type: GeneratedStruct50Type::Smoker,
                facing: Direction::South,
                lit: true,
            } => Ok(19445u32),
            GeneratedStruct50 {
                block_type: GeneratedStruct50Type::Smoker,
                facing: Direction::South,
                lit: false,
            } => Ok(19446u32),
            GeneratedStruct50 {
                block_type: GeneratedStruct50Type::Smoker,
                lit: true,
                facing: Direction::West,
            } => Ok(19447u32),
            GeneratedStruct50 {
                block_type: GeneratedStruct50Type::Smoker,
                facing: Direction::West,
                lit: false,
            } => Ok(19448u32),
            GeneratedStruct50 {
                block_type: GeneratedStruct50Type::Smoker,
                lit: true,
                facing: Direction::East,
            } => Ok(19449u32),
            GeneratedStruct50 {
                block_type: GeneratedStruct50Type::Smoker,
                facing: Direction::East,
                lit: false,
            } => Ok(19450u32),
            _ => Err(()),
        }
    }
}

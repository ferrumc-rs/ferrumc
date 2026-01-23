#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub struct GeneratedStruct8 {
    pub attached: bool,
    pub facing: Direction,
    pub powered: bool,
}
impl TryFrom<u32> for GeneratedStruct8 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            8305u32 => Ok(GeneratedStruct8 {
                powered: true,
                facing: Direction::North,
                attached: true,
            }),
            8306u32 => Ok(GeneratedStruct8 {
                attached: true,
                facing: Direction::North,
                powered: false,
            }),
            8307u32 => Ok(GeneratedStruct8 {
                facing: Direction::South,
                powered: true,
                attached: true,
            }),
            8308u32 => Ok(GeneratedStruct8 {
                powered: false,
                attached: true,
                facing: Direction::South,
            }),
            8309u32 => Ok(GeneratedStruct8 {
                facing: Direction::West,
                powered: true,
                attached: true,
            }),
            8310u32 => Ok(GeneratedStruct8 {
                facing: Direction::West,
                attached: true,
                powered: false,
            }),
            8311u32 => Ok(GeneratedStruct8 {
                facing: Direction::East,
                attached: true,
                powered: true,
            }),
            8312u32 => Ok(GeneratedStruct8 {
                facing: Direction::East,
                powered: false,
                attached: true,
            }),
            8313u32 => Ok(GeneratedStruct8 {
                attached: false,
                facing: Direction::North,
                powered: true,
            }),
            8314u32 => Ok(GeneratedStruct8 {
                powered: false,
                facing: Direction::North,
                attached: false,
            }),
            8315u32 => Ok(GeneratedStruct8 {
                attached: false,
                facing: Direction::South,
                powered: true,
            }),
            8316u32 => Ok(GeneratedStruct8 {
                powered: false,
                attached: false,
                facing: Direction::South,
            }),
            8317u32 => Ok(GeneratedStruct8 {
                facing: Direction::West,
                attached: false,
                powered: true,
            }),
            8318u32 => Ok(GeneratedStruct8 {
                powered: false,
                attached: false,
                facing: Direction::West,
            }),
            8319u32 => Ok(GeneratedStruct8 {
                facing: Direction::East,
                powered: true,
                attached: false,
            }),
            8320u32 => Ok(GeneratedStruct8 {
                attached: false,
                facing: Direction::East,
                powered: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct8 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct8 {
                powered: true,
                facing: Direction::North,
                attached: true,
            } => Ok(8305u32),
            GeneratedStruct8 {
                attached: true,
                facing: Direction::North,
                powered: false,
            } => Ok(8306u32),
            GeneratedStruct8 {
                facing: Direction::South,
                powered: true,
                attached: true,
            } => Ok(8307u32),
            GeneratedStruct8 {
                powered: false,
                attached: true,
                facing: Direction::South,
            } => Ok(8308u32),
            GeneratedStruct8 {
                facing: Direction::West,
                powered: true,
                attached: true,
            } => Ok(8309u32),
            GeneratedStruct8 {
                facing: Direction::West,
                attached: true,
                powered: false,
            } => Ok(8310u32),
            GeneratedStruct8 {
                facing: Direction::East,
                attached: true,
                powered: true,
            } => Ok(8311u32),
            GeneratedStruct8 {
                facing: Direction::East,
                powered: false,
                attached: true,
            } => Ok(8312u32),
            GeneratedStruct8 {
                attached: false,
                facing: Direction::North,
                powered: true,
            } => Ok(8313u32),
            GeneratedStruct8 {
                powered: false,
                facing: Direction::North,
                attached: false,
            } => Ok(8314u32),
            GeneratedStruct8 {
                attached: false,
                facing: Direction::South,
                powered: true,
            } => Ok(8315u32),
            GeneratedStruct8 {
                powered: false,
                attached: false,
                facing: Direction::South,
            } => Ok(8316u32),
            GeneratedStruct8 {
                facing: Direction::West,
                attached: false,
                powered: true,
            } => Ok(8317u32),
            GeneratedStruct8 {
                powered: false,
                attached: false,
                facing: Direction::West,
            } => Ok(8318u32),
            GeneratedStruct8 {
                facing: Direction::East,
                powered: true,
                attached: false,
            } => Ok(8319u32),
            GeneratedStruct8 {
                attached: false,
                facing: Direction::East,
                powered: false,
            } => Ok(8320u32),
            _ => Err(()),
        }
    }
}

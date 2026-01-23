#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub struct GeneratedStruct52 {
    pub facing: Direction,
    pub mode: ComparatorMode,
    pub powered: bool,
}
impl TryFrom<u32> for GeneratedStruct52 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            9984u32 => Ok(GeneratedStruct52 {
                mode: ComparatorMode::Compare,
                powered: true,
                facing: Direction::North,
            }),
            9985u32 => Ok(GeneratedStruct52 {
                facing: Direction::North,
                mode: ComparatorMode::Compare,
                powered: false,
            }),
            9986u32 => Ok(GeneratedStruct52 {
                facing: Direction::North,
                mode: ComparatorMode::Subtract,
                powered: true,
            }),
            9987u32 => Ok(GeneratedStruct52 {
                powered: false,
                mode: ComparatorMode::Subtract,
                facing: Direction::North,
            }),
            9988u32 => Ok(GeneratedStruct52 {
                powered: true,
                facing: Direction::South,
                mode: ComparatorMode::Compare,
            }),
            9989u32 => Ok(GeneratedStruct52 {
                facing: Direction::South,
                powered: false,
                mode: ComparatorMode::Compare,
            }),
            9990u32 => Ok(GeneratedStruct52 {
                mode: ComparatorMode::Subtract,
                powered: true,
                facing: Direction::South,
            }),
            9991u32 => Ok(GeneratedStruct52 {
                facing: Direction::South,
                powered: false,
                mode: ComparatorMode::Subtract,
            }),
            9992u32 => Ok(GeneratedStruct52 {
                facing: Direction::West,
                mode: ComparatorMode::Compare,
                powered: true,
            }),
            9993u32 => Ok(GeneratedStruct52 {
                powered: false,
                facing: Direction::West,
                mode: ComparatorMode::Compare,
            }),
            9994u32 => Ok(GeneratedStruct52 {
                facing: Direction::West,
                mode: ComparatorMode::Subtract,
                powered: true,
            }),
            9995u32 => Ok(GeneratedStruct52 {
                mode: ComparatorMode::Subtract,
                facing: Direction::West,
                powered: false,
            }),
            9996u32 => Ok(GeneratedStruct52 {
                mode: ComparatorMode::Compare,
                facing: Direction::East,
                powered: true,
            }),
            9997u32 => Ok(GeneratedStruct52 {
                facing: Direction::East,
                mode: ComparatorMode::Compare,
                powered: false,
            }),
            9998u32 => Ok(GeneratedStruct52 {
                powered: true,
                mode: ComparatorMode::Subtract,
                facing: Direction::East,
            }),
            9999u32 => Ok(GeneratedStruct52 {
                facing: Direction::East,
                powered: false,
                mode: ComparatorMode::Subtract,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct52 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct52 {
                mode: ComparatorMode::Compare,
                powered: true,
                facing: Direction::North,
            } => Ok(9984u32),
            GeneratedStruct52 {
                facing: Direction::North,
                mode: ComparatorMode::Compare,
                powered: false,
            } => Ok(9985u32),
            GeneratedStruct52 {
                facing: Direction::North,
                mode: ComparatorMode::Subtract,
                powered: true,
            } => Ok(9986u32),
            GeneratedStruct52 {
                powered: false,
                mode: ComparatorMode::Subtract,
                facing: Direction::North,
            } => Ok(9987u32),
            GeneratedStruct52 {
                powered: true,
                facing: Direction::South,
                mode: ComparatorMode::Compare,
            } => Ok(9988u32),
            GeneratedStruct52 {
                facing: Direction::South,
                powered: false,
                mode: ComparatorMode::Compare,
            } => Ok(9989u32),
            GeneratedStruct52 {
                mode: ComparatorMode::Subtract,
                powered: true,
                facing: Direction::South,
            } => Ok(9990u32),
            GeneratedStruct52 {
                facing: Direction::South,
                powered: false,
                mode: ComparatorMode::Subtract,
            } => Ok(9991u32),
            GeneratedStruct52 {
                facing: Direction::West,
                mode: ComparatorMode::Compare,
                powered: true,
            } => Ok(9992u32),
            GeneratedStruct52 {
                powered: false,
                facing: Direction::West,
                mode: ComparatorMode::Compare,
            } => Ok(9993u32),
            GeneratedStruct52 {
                facing: Direction::West,
                mode: ComparatorMode::Subtract,
                powered: true,
            } => Ok(9994u32),
            GeneratedStruct52 {
                mode: ComparatorMode::Subtract,
                facing: Direction::West,
                powered: false,
            } => Ok(9995u32),
            GeneratedStruct52 {
                mode: ComparatorMode::Compare,
                facing: Direction::East,
                powered: true,
            } => Ok(9996u32),
            GeneratedStruct52 {
                facing: Direction::East,
                mode: ComparatorMode::Compare,
                powered: false,
            } => Ok(9997u32),
            GeneratedStruct52 {
                powered: true,
                mode: ComparatorMode::Subtract,
                facing: Direction::East,
            } => Ok(9998u32),
            GeneratedStruct52 {
                facing: Direction::East,
                powered: false,
                mode: ComparatorMode::Subtract,
            } => Ok(9999u32),
            _ => Err(()),
        }
    }
}

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
                facing: Direction::North,
                mode: ComparatorMode::Compare,
                powered: true,
            }),
            9985u32 => Ok(GeneratedStruct52 {
                powered: false,
                mode: ComparatorMode::Compare,
                facing: Direction::North,
            }),
            9986u32 => Ok(GeneratedStruct52 {
                facing: Direction::North,
                mode: ComparatorMode::Subtract,
                powered: true,
            }),
            9987u32 => Ok(GeneratedStruct52 {
                mode: ComparatorMode::Subtract,
                powered: false,
                facing: Direction::North,
            }),
            9988u32 => Ok(GeneratedStruct52 {
                powered: true,
                mode: ComparatorMode::Compare,
                facing: Direction::South,
            }),
            9989u32 => Ok(GeneratedStruct52 {
                facing: Direction::South,
                powered: false,
                mode: ComparatorMode::Compare,
            }),
            9990u32 => Ok(GeneratedStruct52 {
                powered: true,
                mode: ComparatorMode::Subtract,
                facing: Direction::South,
            }),
            9991u32 => Ok(GeneratedStruct52 {
                mode: ComparatorMode::Subtract,
                powered: false,
                facing: Direction::South,
            }),
            9992u32 => Ok(GeneratedStruct52 {
                facing: Direction::West,
                powered: true,
                mode: ComparatorMode::Compare,
            }),
            9993u32 => Ok(GeneratedStruct52 {
                facing: Direction::West,
                mode: ComparatorMode::Compare,
                powered: false,
            }),
            9994u32 => Ok(GeneratedStruct52 {
                mode: ComparatorMode::Subtract,
                facing: Direction::West,
                powered: true,
            }),
            9995u32 => Ok(GeneratedStruct52 {
                facing: Direction::West,
                mode: ComparatorMode::Subtract,
                powered: false,
            }),
            9996u32 => Ok(GeneratedStruct52 {
                powered: true,
                facing: Direction::East,
                mode: ComparatorMode::Compare,
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
                mode: ComparatorMode::Subtract,
                powered: false,
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
                facing: Direction::North,
                mode: ComparatorMode::Compare,
                powered: true,
            } => Ok(9984u32),
            GeneratedStruct52 {
                powered: false,
                mode: ComparatorMode::Compare,
                facing: Direction::North,
            } => Ok(9985u32),
            GeneratedStruct52 {
                facing: Direction::North,
                mode: ComparatorMode::Subtract,
                powered: true,
            } => Ok(9986u32),
            GeneratedStruct52 {
                mode: ComparatorMode::Subtract,
                powered: false,
                facing: Direction::North,
            } => Ok(9987u32),
            GeneratedStruct52 {
                powered: true,
                mode: ComparatorMode::Compare,
                facing: Direction::South,
            } => Ok(9988u32),
            GeneratedStruct52 {
                facing: Direction::South,
                powered: false,
                mode: ComparatorMode::Compare,
            } => Ok(9989u32),
            GeneratedStruct52 {
                powered: true,
                mode: ComparatorMode::Subtract,
                facing: Direction::South,
            } => Ok(9990u32),
            GeneratedStruct52 {
                mode: ComparatorMode::Subtract,
                powered: false,
                facing: Direction::South,
            } => Ok(9991u32),
            GeneratedStruct52 {
                facing: Direction::West,
                powered: true,
                mode: ComparatorMode::Compare,
            } => Ok(9992u32),
            GeneratedStruct52 {
                facing: Direction::West,
                mode: ComparatorMode::Compare,
                powered: false,
            } => Ok(9993u32),
            GeneratedStruct52 {
                mode: ComparatorMode::Subtract,
                facing: Direction::West,
                powered: true,
            } => Ok(9994u32),
            GeneratedStruct52 {
                facing: Direction::West,
                mode: ComparatorMode::Subtract,
                powered: false,
            } => Ok(9995u32),
            GeneratedStruct52 {
                powered: true,
                facing: Direction::East,
                mode: ComparatorMode::Compare,
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
                mode: ComparatorMode::Subtract,
                powered: false,
            } => Ok(9999u32),
            _ => Err(()),
        }
    }
}

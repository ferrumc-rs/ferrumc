#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub struct GeneratedStruct36 {
    pub enabled: bool,
    pub facing: Direction,
}
impl TryFrom<u32> for GeneratedStruct36 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            10034u32 => Ok(GeneratedStruct36 {
                enabled: true,
                facing: Direction::Down,
            }),
            10035u32 => Ok(GeneratedStruct36 {
                enabled: true,
                facing: Direction::North,
            }),
            10036u32 => Ok(GeneratedStruct36 {
                enabled: true,
                facing: Direction::South,
            }),
            10037u32 => Ok(GeneratedStruct36 {
                facing: Direction::West,
                enabled: true,
            }),
            10038u32 => Ok(GeneratedStruct36 {
                enabled: true,
                facing: Direction::East,
            }),
            10039u32 => Ok(GeneratedStruct36 {
                facing: Direction::Down,
                enabled: false,
            }),
            10040u32 => Ok(GeneratedStruct36 {
                facing: Direction::North,
                enabled: false,
            }),
            10041u32 => Ok(GeneratedStruct36 {
                enabled: false,
                facing: Direction::South,
            }),
            10042u32 => Ok(GeneratedStruct36 {
                facing: Direction::West,
                enabled: false,
            }),
            10043u32 => Ok(GeneratedStruct36 {
                enabled: false,
                facing: Direction::East,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct36 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct36 {
                enabled: true,
                facing: Direction::Down,
            } => Ok(10034u32),
            GeneratedStruct36 {
                enabled: true,
                facing: Direction::North,
            } => Ok(10035u32),
            GeneratedStruct36 {
                enabled: true,
                facing: Direction::South,
            } => Ok(10036u32),
            GeneratedStruct36 {
                facing: Direction::West,
                enabled: true,
            } => Ok(10037u32),
            GeneratedStruct36 {
                enabled: true,
                facing: Direction::East,
            } => Ok(10038u32),
            GeneratedStruct36 {
                facing: Direction::Down,
                enabled: false,
            } => Ok(10039u32),
            GeneratedStruct36 {
                facing: Direction::North,
                enabled: false,
            } => Ok(10040u32),
            GeneratedStruct36 {
                enabled: false,
                facing: Direction::South,
            } => Ok(10041u32),
            GeneratedStruct36 {
                facing: Direction::West,
                enabled: false,
            } => Ok(10042u32),
            GeneratedStruct36 {
                enabled: false,
                facing: Direction::East,
            } => Ok(10043u32),
            _ => Err(()),
        }
    }
}

#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub struct GeneratedStruct38 {
    pub eye: bool,
    pub facing: Direction,
}
impl TryFrom<u32> for GeneratedStruct38 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            8191u32 => Ok(GeneratedStruct38 {
                eye: true,
                facing: Direction::North,
            }),
            8192u32 => Ok(GeneratedStruct38 {
                eye: true,
                facing: Direction::South,
            }),
            8193u32 => Ok(GeneratedStruct38 {
                facing: Direction::West,
                eye: true,
            }),
            8194u32 => Ok(GeneratedStruct38 {
                eye: true,
                facing: Direction::East,
            }),
            8195u32 => Ok(GeneratedStruct38 {
                facing: Direction::North,
                eye: false,
            }),
            8196u32 => Ok(GeneratedStruct38 {
                facing: Direction::South,
                eye: false,
            }),
            8197u32 => Ok(GeneratedStruct38 {
                facing: Direction::West,
                eye: false,
            }),
            8198u32 => Ok(GeneratedStruct38 {
                facing: Direction::East,
                eye: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct38 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct38 {
                eye: true,
                facing: Direction::North,
            } => Ok(8191u32),
            GeneratedStruct38 {
                eye: true,
                facing: Direction::South,
            } => Ok(8192u32),
            GeneratedStruct38 {
                facing: Direction::West,
                eye: true,
            } => Ok(8193u32),
            GeneratedStruct38 {
                eye: true,
                facing: Direction::East,
            } => Ok(8194u32),
            GeneratedStruct38 {
                facing: Direction::North,
                eye: false,
            } => Ok(8195u32),
            GeneratedStruct38 {
                facing: Direction::South,
                eye: false,
            } => Ok(8196u32),
            GeneratedStruct38 {
                facing: Direction::West,
                eye: false,
            } => Ok(8197u32),
            GeneratedStruct38 {
                facing: Direction::East,
                eye: false,
            } => Ok(8198u32),
            _ => Err(()),
        }
    }
}

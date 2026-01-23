#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub struct GeneratedStruct65 {
    pub facing: Direction,
    pub ty: PistonType,
}
impl TryFrom<u32> for GeneratedStruct65 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            2109u32 => Ok(GeneratedStruct65 {
                ty: PistonType::Default,
                facing: Direction::North,
            }),
            2110u32 => Ok(GeneratedStruct65 {
                facing: Direction::North,
                ty: PistonType::Sticky,
            }),
            2111u32 => Ok(GeneratedStruct65 {
                ty: PistonType::Default,
                facing: Direction::East,
            }),
            2112u32 => Ok(GeneratedStruct65 {
                facing: Direction::East,
                ty: PistonType::Sticky,
            }),
            2113u32 => Ok(GeneratedStruct65 {
                facing: Direction::South,
                ty: PistonType::Default,
            }),
            2114u32 => Ok(GeneratedStruct65 {
                ty: PistonType::Sticky,
                facing: Direction::South,
            }),
            2115u32 => Ok(GeneratedStruct65 {
                facing: Direction::West,
                ty: PistonType::Default,
            }),
            2116u32 => Ok(GeneratedStruct65 {
                ty: PistonType::Sticky,
                facing: Direction::West,
            }),
            2117u32 => Ok(GeneratedStruct65 {
                facing: Direction::Up,
                ty: PistonType::Default,
            }),
            2118u32 => Ok(GeneratedStruct65 {
                ty: PistonType::Sticky,
                facing: Direction::Up,
            }),
            2119u32 => Ok(GeneratedStruct65 {
                ty: PistonType::Default,
                facing: Direction::Down,
            }),
            2120u32 => Ok(GeneratedStruct65 {
                facing: Direction::Down,
                ty: PistonType::Sticky,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct65 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct65 {
                ty: PistonType::Default,
                facing: Direction::North,
            } => Ok(2109u32),
            GeneratedStruct65 {
                facing: Direction::North,
                ty: PistonType::Sticky,
            } => Ok(2110u32),
            GeneratedStruct65 {
                ty: PistonType::Default,
                facing: Direction::East,
            } => Ok(2111u32),
            GeneratedStruct65 {
                facing: Direction::East,
                ty: PistonType::Sticky,
            } => Ok(2112u32),
            GeneratedStruct65 {
                facing: Direction::South,
                ty: PistonType::Default,
            } => Ok(2113u32),
            GeneratedStruct65 {
                ty: PistonType::Sticky,
                facing: Direction::South,
            } => Ok(2114u32),
            GeneratedStruct65 {
                facing: Direction::West,
                ty: PistonType::Default,
            } => Ok(2115u32),
            GeneratedStruct65 {
                ty: PistonType::Sticky,
                facing: Direction::West,
            } => Ok(2116u32),
            GeneratedStruct65 {
                facing: Direction::Up,
                ty: PistonType::Default,
            } => Ok(2117u32),
            GeneratedStruct65 {
                ty: PistonType::Sticky,
                facing: Direction::Up,
            } => Ok(2118u32),
            GeneratedStruct65 {
                ty: PistonType::Default,
                facing: Direction::Down,
            } => Ok(2119u32),
            GeneratedStruct65 {
                facing: Direction::Down,
                ty: PistonType::Sticky,
            } => Ok(2120u32),
            _ => Err(()),
        }
    }
}

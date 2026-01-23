#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GeneratedStruct65 {
    pub facing: Direction,
    pub ty: PistonType,
}
impl TryInto<u32> for GeneratedStruct65 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct65 {
                facing: Direction::North,
                ty: PistonType::Default,
            } => Ok(2109u32),
            GeneratedStruct65 {
                facing: Direction::North,
                ty: PistonType::Sticky,
            } => Ok(2110u32),
            GeneratedStruct65 {
                facing: Direction::East,
                ty: PistonType::Default,
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
                facing: Direction::South,
                ty: PistonType::Sticky,
            } => Ok(2114u32),
            GeneratedStruct65 {
                facing: Direction::West,
                ty: PistonType::Default,
            } => Ok(2115u32),
            GeneratedStruct65 {
                facing: Direction::West,
                ty: PistonType::Sticky,
            } => Ok(2116u32),
            GeneratedStruct65 {
                facing: Direction::Up,
                ty: PistonType::Default,
            } => Ok(2117u32),
            GeneratedStruct65 {
                facing: Direction::Up,
                ty: PistonType::Sticky,
            } => Ok(2118u32),
            GeneratedStruct65 {
                facing: Direction::Down,
                ty: PistonType::Default,
            } => Ok(2119u32),
            GeneratedStruct65 {
                facing: Direction::Down,
                ty: PistonType::Sticky,
            } => Ok(2120u32),
            _ => Err(()),
        }
    }
}

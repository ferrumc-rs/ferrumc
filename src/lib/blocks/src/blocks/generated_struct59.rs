#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GeneratedStruct59 {
    pub facing: Direction,
    pub segment_amount: i32,
}
impl TryInto<u32> for GeneratedStruct59 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct59 {
                facing: Direction::North,
                segment_amount: 1i32,
            } => Ok(25887u32),
            GeneratedStruct59 {
                facing: Direction::North,
                segment_amount: 2i32,
            } => Ok(25888u32),
            GeneratedStruct59 {
                facing: Direction::North,
                segment_amount: 3i32,
            } => Ok(25889u32),
            GeneratedStruct59 {
                facing: Direction::North,
                segment_amount: 4i32,
            } => Ok(25890u32),
            GeneratedStruct59 {
                facing: Direction::South,
                segment_amount: 1i32,
            } => Ok(25891u32),
            GeneratedStruct59 {
                facing: Direction::South,
                segment_amount: 2i32,
            } => Ok(25892u32),
            GeneratedStruct59 {
                facing: Direction::South,
                segment_amount: 3i32,
            } => Ok(25893u32),
            GeneratedStruct59 {
                facing: Direction::South,
                segment_amount: 4i32,
            } => Ok(25894u32),
            GeneratedStruct59 {
                facing: Direction::West,
                segment_amount: 1i32,
            } => Ok(25895u32),
            GeneratedStruct59 {
                facing: Direction::West,
                segment_amount: 2i32,
            } => Ok(25896u32),
            GeneratedStruct59 {
                facing: Direction::West,
                segment_amount: 3i32,
            } => Ok(25897u32),
            GeneratedStruct59 {
                facing: Direction::West,
                segment_amount: 4i32,
            } => Ok(25898u32),
            GeneratedStruct59 {
                facing: Direction::East,
                segment_amount: 1i32,
            } => Ok(25899u32),
            GeneratedStruct59 {
                facing: Direction::East,
                segment_amount: 2i32,
            } => Ok(25900u32),
            GeneratedStruct59 {
                facing: Direction::East,
                segment_amount: 3i32,
            } => Ok(25901u32),
            GeneratedStruct59 {
                facing: Direction::East,
                segment_amount: 4i32,
            } => Ok(25902u32),
            _ => Err(()),
        }
    }
}

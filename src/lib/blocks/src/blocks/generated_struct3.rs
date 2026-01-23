#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GeneratedStruct3 {
    pub age: i32,
    pub facing: Direction,
}
impl TryInto<u32> for GeneratedStruct3 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct3 {
                age: 0i32,
                facing: Direction::North,
            } => Ok(8203u32),
            GeneratedStruct3 {
                age: 0i32,
                facing: Direction::South,
            } => Ok(8204u32),
            GeneratedStruct3 {
                age: 0i32,
                facing: Direction::West,
            } => Ok(8205u32),
            GeneratedStruct3 {
                age: 0i32,
                facing: Direction::East,
            } => Ok(8206u32),
            GeneratedStruct3 {
                age: 1i32,
                facing: Direction::North,
            } => Ok(8207u32),
            GeneratedStruct3 {
                age: 1i32,
                facing: Direction::South,
            } => Ok(8208u32),
            GeneratedStruct3 {
                age: 1i32,
                facing: Direction::West,
            } => Ok(8209u32),
            GeneratedStruct3 {
                age: 1i32,
                facing: Direction::East,
            } => Ok(8210u32),
            GeneratedStruct3 {
                age: 2i32,
                facing: Direction::North,
            } => Ok(8211u32),
            GeneratedStruct3 {
                age: 2i32,
                facing: Direction::South,
            } => Ok(8212u32),
            GeneratedStruct3 {
                age: 2i32,
                facing: Direction::West,
            } => Ok(8213u32),
            GeneratedStruct3 {
                age: 2i32,
                facing: Direction::East,
            } => Ok(8214u32),
            _ => Err(()),
        }
    }
}

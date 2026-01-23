#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GeneratedStruct44 {
    pub facing: Direction,
    pub half: DoubleBlockHalf,
    pub waterlogged: bool,
}
impl TryInto<u32> for GeneratedStruct44 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct44 {
                facing: Direction::North,
                half: DoubleBlockHalf::Upper,
                waterlogged: true,
            } => Ok(25944u32),
            GeneratedStruct44 {
                facing: Direction::North,
                half: DoubleBlockHalf::Upper,
                waterlogged: false,
            } => Ok(25945u32),
            GeneratedStruct44 {
                facing: Direction::North,
                half: DoubleBlockHalf::Lower,
                waterlogged: true,
            } => Ok(25946u32),
            GeneratedStruct44 {
                facing: Direction::North,
                half: DoubleBlockHalf::Lower,
                waterlogged: false,
            } => Ok(25947u32),
            GeneratedStruct44 {
                facing: Direction::South,
                half: DoubleBlockHalf::Upper,
                waterlogged: true,
            } => Ok(25948u32),
            GeneratedStruct44 {
                facing: Direction::South,
                half: DoubleBlockHalf::Upper,
                waterlogged: false,
            } => Ok(25949u32),
            GeneratedStruct44 {
                facing: Direction::South,
                half: DoubleBlockHalf::Lower,
                waterlogged: true,
            } => Ok(25950u32),
            GeneratedStruct44 {
                facing: Direction::South,
                half: DoubleBlockHalf::Lower,
                waterlogged: false,
            } => Ok(25951u32),
            GeneratedStruct44 {
                facing: Direction::West,
                half: DoubleBlockHalf::Upper,
                waterlogged: true,
            } => Ok(25952u32),
            GeneratedStruct44 {
                facing: Direction::West,
                half: DoubleBlockHalf::Upper,
                waterlogged: false,
            } => Ok(25953u32),
            GeneratedStruct44 {
                facing: Direction::West,
                half: DoubleBlockHalf::Lower,
                waterlogged: true,
            } => Ok(25954u32),
            GeneratedStruct44 {
                facing: Direction::West,
                half: DoubleBlockHalf::Lower,
                waterlogged: false,
            } => Ok(25955u32),
            GeneratedStruct44 {
                facing: Direction::East,
                half: DoubleBlockHalf::Upper,
                waterlogged: true,
            } => Ok(25956u32),
            GeneratedStruct44 {
                facing: Direction::East,
                half: DoubleBlockHalf::Upper,
                waterlogged: false,
            } => Ok(25957u32),
            GeneratedStruct44 {
                facing: Direction::East,
                half: DoubleBlockHalf::Lower,
                waterlogged: true,
            } => Ok(25958u32),
            GeneratedStruct44 {
                facing: Direction::East,
                half: DoubleBlockHalf::Lower,
                waterlogged: false,
            } => Ok(25959u32),
            _ => Err(()),
        }
    }
}

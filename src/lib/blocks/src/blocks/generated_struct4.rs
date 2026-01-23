#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GeneratedStruct4 {
    pub age: i32,
    pub half: DoubleBlockHalf,
}
impl TryInto<u32> for GeneratedStruct4 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct4 {
                age: 0i32,
                half: DoubleBlockHalf::Upper,
            } => Ok(13520u32),
            GeneratedStruct4 {
                age: 0i32,
                half: DoubleBlockHalf::Lower,
            } => Ok(13521u32),
            GeneratedStruct4 {
                age: 1i32,
                half: DoubleBlockHalf::Upper,
            } => Ok(13522u32),
            GeneratedStruct4 {
                age: 1i32,
                half: DoubleBlockHalf::Lower,
            } => Ok(13523u32),
            GeneratedStruct4 {
                age: 2i32,
                half: DoubleBlockHalf::Upper,
            } => Ok(13524u32),
            GeneratedStruct4 {
                age: 2i32,
                half: DoubleBlockHalf::Lower,
            } => Ok(13525u32),
            GeneratedStruct4 {
                age: 3i32,
                half: DoubleBlockHalf::Upper,
            } => Ok(13526u32),
            GeneratedStruct4 {
                age: 3i32,
                half: DoubleBlockHalf::Lower,
            } => Ok(13527u32),
            GeneratedStruct4 {
                age: 4i32,
                half: DoubleBlockHalf::Upper,
            } => Ok(13528u32),
            GeneratedStruct4 {
                age: 4i32,
                half: DoubleBlockHalf::Lower,
            } => Ok(13529u32),
            _ => Err(()),
        }
    }
}

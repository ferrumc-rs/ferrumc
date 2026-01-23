#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub struct GeneratedStruct4 {
    pub age: i32,
    pub half: DoubleBlockHalf,
}
impl TryFrom<u32> for GeneratedStruct4 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            13520u32 => Ok(GeneratedStruct4 {
                age: 0i32,
                half: DoubleBlockHalf::Upper,
            }),
            13521u32 => Ok(GeneratedStruct4 {
                half: DoubleBlockHalf::Lower,
                age: 0i32,
            }),
            13522u32 => Ok(GeneratedStruct4 {
                half: DoubleBlockHalf::Upper,
                age: 1i32,
            }),
            13523u32 => Ok(GeneratedStruct4 {
                age: 1i32,
                half: DoubleBlockHalf::Lower,
            }),
            13524u32 => Ok(GeneratedStruct4 {
                half: DoubleBlockHalf::Upper,
                age: 2i32,
            }),
            13525u32 => Ok(GeneratedStruct4 {
                half: DoubleBlockHalf::Lower,
                age: 2i32,
            }),
            13526u32 => Ok(GeneratedStruct4 {
                half: DoubleBlockHalf::Upper,
                age: 3i32,
            }),
            13527u32 => Ok(GeneratedStruct4 {
                half: DoubleBlockHalf::Lower,
                age: 3i32,
            }),
            13528u32 => Ok(GeneratedStruct4 {
                half: DoubleBlockHalf::Upper,
                age: 4i32,
            }),
            13529u32 => Ok(GeneratedStruct4 {
                age: 4i32,
                half: DoubleBlockHalf::Lower,
            }),
            _ => Err(()),
        }
    }
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
                half: DoubleBlockHalf::Lower,
                age: 0i32,
            } => Ok(13521u32),
            GeneratedStruct4 {
                half: DoubleBlockHalf::Upper,
                age: 1i32,
            } => Ok(13522u32),
            GeneratedStruct4 {
                age: 1i32,
                half: DoubleBlockHalf::Lower,
            } => Ok(13523u32),
            GeneratedStruct4 {
                half: DoubleBlockHalf::Upper,
                age: 2i32,
            } => Ok(13524u32),
            GeneratedStruct4 {
                half: DoubleBlockHalf::Lower,
                age: 2i32,
            } => Ok(13525u32),
            GeneratedStruct4 {
                half: DoubleBlockHalf::Upper,
                age: 3i32,
            } => Ok(13526u32),
            GeneratedStruct4 {
                half: DoubleBlockHalf::Lower,
                age: 3i32,
            } => Ok(13527u32),
            GeneratedStruct4 {
                half: DoubleBlockHalf::Upper,
                age: 4i32,
            } => Ok(13528u32),
            GeneratedStruct4 {
                age: 4i32,
                half: DoubleBlockHalf::Lower,
            } => Ok(13529u32),
            _ => Err(()),
        }
    }
}

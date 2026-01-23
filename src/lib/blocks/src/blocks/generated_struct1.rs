#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub struct GeneratedStruct1 {
    pub age: i32,
    pub berries: bool,
}
impl TryFrom<u32> for GeneratedStruct1 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            25797u32 => Ok(GeneratedStruct1 {
                age: 0i32,
                berries: true,
            }),
            25798u32 => Ok(GeneratedStruct1 {
                berries: false,
                age: 0i32,
            }),
            25799u32 => Ok(GeneratedStruct1 {
                age: 1i32,
                berries: true,
            }),
            25800u32 => Ok(GeneratedStruct1 {
                berries: false,
                age: 1i32,
            }),
            25801u32 => Ok(GeneratedStruct1 {
                berries: true,
                age: 2i32,
            }),
            25802u32 => Ok(GeneratedStruct1 {
                age: 2i32,
                berries: false,
            }),
            25803u32 => Ok(GeneratedStruct1 {
                berries: true,
                age: 3i32,
            }),
            25804u32 => Ok(GeneratedStruct1 {
                age: 3i32,
                berries: false,
            }),
            25805u32 => Ok(GeneratedStruct1 {
                age: 4i32,
                berries: true,
            }),
            25806u32 => Ok(GeneratedStruct1 {
                berries: false,
                age: 4i32,
            }),
            25807u32 => Ok(GeneratedStruct1 {
                age: 5i32,
                berries: true,
            }),
            25808u32 => Ok(GeneratedStruct1 {
                berries: false,
                age: 5i32,
            }),
            25809u32 => Ok(GeneratedStruct1 {
                age: 6i32,
                berries: true,
            }),
            25810u32 => Ok(GeneratedStruct1 {
                age: 6i32,
                berries: false,
            }),
            25811u32 => Ok(GeneratedStruct1 {
                age: 7i32,
                berries: true,
            }),
            25812u32 => Ok(GeneratedStruct1 {
                age: 7i32,
                berries: false,
            }),
            25813u32 => Ok(GeneratedStruct1 {
                berries: true,
                age: 8i32,
            }),
            25814u32 => Ok(GeneratedStruct1 {
                age: 8i32,
                berries: false,
            }),
            25815u32 => Ok(GeneratedStruct1 {
                berries: true,
                age: 9i32,
            }),
            25816u32 => Ok(GeneratedStruct1 {
                berries: false,
                age: 9i32,
            }),
            25817u32 => Ok(GeneratedStruct1 {
                berries: true,
                age: 10i32,
            }),
            25818u32 => Ok(GeneratedStruct1 {
                age: 10i32,
                berries: false,
            }),
            25819u32 => Ok(GeneratedStruct1 {
                berries: true,
                age: 11i32,
            }),
            25820u32 => Ok(GeneratedStruct1 {
                age: 11i32,
                berries: false,
            }),
            25821u32 => Ok(GeneratedStruct1 {
                age: 12i32,
                berries: true,
            }),
            25822u32 => Ok(GeneratedStruct1 {
                age: 12i32,
                berries: false,
            }),
            25823u32 => Ok(GeneratedStruct1 {
                age: 13i32,
                berries: true,
            }),
            25824u32 => Ok(GeneratedStruct1 {
                age: 13i32,
                berries: false,
            }),
            25825u32 => Ok(GeneratedStruct1 {
                berries: true,
                age: 14i32,
            }),
            25826u32 => Ok(GeneratedStruct1 {
                berries: false,
                age: 14i32,
            }),
            25827u32 => Ok(GeneratedStruct1 {
                berries: true,
                age: 15i32,
            }),
            25828u32 => Ok(GeneratedStruct1 {
                age: 15i32,
                berries: false,
            }),
            25829u32 => Ok(GeneratedStruct1 {
                age: 16i32,
                berries: true,
            }),
            25830u32 => Ok(GeneratedStruct1 {
                age: 16i32,
                berries: false,
            }),
            25831u32 => Ok(GeneratedStruct1 {
                berries: true,
                age: 17i32,
            }),
            25832u32 => Ok(GeneratedStruct1 {
                age: 17i32,
                berries: false,
            }),
            25833u32 => Ok(GeneratedStruct1 {
                berries: true,
                age: 18i32,
            }),
            25834u32 => Ok(GeneratedStruct1 {
                berries: false,
                age: 18i32,
            }),
            25835u32 => Ok(GeneratedStruct1 {
                age: 19i32,
                berries: true,
            }),
            25836u32 => Ok(GeneratedStruct1 {
                age: 19i32,
                berries: false,
            }),
            25837u32 => Ok(GeneratedStruct1 {
                age: 20i32,
                berries: true,
            }),
            25838u32 => Ok(GeneratedStruct1 {
                berries: false,
                age: 20i32,
            }),
            25839u32 => Ok(GeneratedStruct1 {
                age: 21i32,
                berries: true,
            }),
            25840u32 => Ok(GeneratedStruct1 {
                berries: false,
                age: 21i32,
            }),
            25841u32 => Ok(GeneratedStruct1 {
                age: 22i32,
                berries: true,
            }),
            25842u32 => Ok(GeneratedStruct1 {
                age: 22i32,
                berries: false,
            }),
            25843u32 => Ok(GeneratedStruct1 {
                age: 23i32,
                berries: true,
            }),
            25844u32 => Ok(GeneratedStruct1 {
                berries: false,
                age: 23i32,
            }),
            25845u32 => Ok(GeneratedStruct1 {
                berries: true,
                age: 24i32,
            }),
            25846u32 => Ok(GeneratedStruct1 {
                berries: false,
                age: 24i32,
            }),
            25847u32 => Ok(GeneratedStruct1 {
                age: 25i32,
                berries: true,
            }),
            25848u32 => Ok(GeneratedStruct1 {
                berries: false,
                age: 25i32,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct1 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct1 {
                age: 0i32,
                berries: true,
            } => Ok(25797u32),
            GeneratedStruct1 {
                berries: false,
                age: 0i32,
            } => Ok(25798u32),
            GeneratedStruct1 {
                age: 1i32,
                berries: true,
            } => Ok(25799u32),
            GeneratedStruct1 {
                berries: false,
                age: 1i32,
            } => Ok(25800u32),
            GeneratedStruct1 {
                berries: true,
                age: 2i32,
            } => Ok(25801u32),
            GeneratedStruct1 {
                age: 2i32,
                berries: false,
            } => Ok(25802u32),
            GeneratedStruct1 {
                berries: true,
                age: 3i32,
            } => Ok(25803u32),
            GeneratedStruct1 {
                age: 3i32,
                berries: false,
            } => Ok(25804u32),
            GeneratedStruct1 {
                age: 4i32,
                berries: true,
            } => Ok(25805u32),
            GeneratedStruct1 {
                berries: false,
                age: 4i32,
            } => Ok(25806u32),
            GeneratedStruct1 {
                age: 5i32,
                berries: true,
            } => Ok(25807u32),
            GeneratedStruct1 {
                berries: false,
                age: 5i32,
            } => Ok(25808u32),
            GeneratedStruct1 {
                age: 6i32,
                berries: true,
            } => Ok(25809u32),
            GeneratedStruct1 {
                age: 6i32,
                berries: false,
            } => Ok(25810u32),
            GeneratedStruct1 {
                age: 7i32,
                berries: true,
            } => Ok(25811u32),
            GeneratedStruct1 {
                age: 7i32,
                berries: false,
            } => Ok(25812u32),
            GeneratedStruct1 {
                berries: true,
                age: 8i32,
            } => Ok(25813u32),
            GeneratedStruct1 {
                age: 8i32,
                berries: false,
            } => Ok(25814u32),
            GeneratedStruct1 {
                berries: true,
                age: 9i32,
            } => Ok(25815u32),
            GeneratedStruct1 {
                berries: false,
                age: 9i32,
            } => Ok(25816u32),
            GeneratedStruct1 {
                berries: true,
                age: 10i32,
            } => Ok(25817u32),
            GeneratedStruct1 {
                age: 10i32,
                berries: false,
            } => Ok(25818u32),
            GeneratedStruct1 {
                berries: true,
                age: 11i32,
            } => Ok(25819u32),
            GeneratedStruct1 {
                age: 11i32,
                berries: false,
            } => Ok(25820u32),
            GeneratedStruct1 {
                age: 12i32,
                berries: true,
            } => Ok(25821u32),
            GeneratedStruct1 {
                age: 12i32,
                berries: false,
            } => Ok(25822u32),
            GeneratedStruct1 {
                age: 13i32,
                berries: true,
            } => Ok(25823u32),
            GeneratedStruct1 {
                age: 13i32,
                berries: false,
            } => Ok(25824u32),
            GeneratedStruct1 {
                berries: true,
                age: 14i32,
            } => Ok(25825u32),
            GeneratedStruct1 {
                berries: false,
                age: 14i32,
            } => Ok(25826u32),
            GeneratedStruct1 {
                berries: true,
                age: 15i32,
            } => Ok(25827u32),
            GeneratedStruct1 {
                age: 15i32,
                berries: false,
            } => Ok(25828u32),
            GeneratedStruct1 {
                age: 16i32,
                berries: true,
            } => Ok(25829u32),
            GeneratedStruct1 {
                age: 16i32,
                berries: false,
            } => Ok(25830u32),
            GeneratedStruct1 {
                berries: true,
                age: 17i32,
            } => Ok(25831u32),
            GeneratedStruct1 {
                age: 17i32,
                berries: false,
            } => Ok(25832u32),
            GeneratedStruct1 {
                berries: true,
                age: 18i32,
            } => Ok(25833u32),
            GeneratedStruct1 {
                berries: false,
                age: 18i32,
            } => Ok(25834u32),
            GeneratedStruct1 {
                age: 19i32,
                berries: true,
            } => Ok(25835u32),
            GeneratedStruct1 {
                age: 19i32,
                berries: false,
            } => Ok(25836u32),
            GeneratedStruct1 {
                age: 20i32,
                berries: true,
            } => Ok(25837u32),
            GeneratedStruct1 {
                berries: false,
                age: 20i32,
            } => Ok(25838u32),
            GeneratedStruct1 {
                age: 21i32,
                berries: true,
            } => Ok(25839u32),
            GeneratedStruct1 {
                berries: false,
                age: 21i32,
            } => Ok(25840u32),
            GeneratedStruct1 {
                age: 22i32,
                berries: true,
            } => Ok(25841u32),
            GeneratedStruct1 {
                age: 22i32,
                berries: false,
            } => Ok(25842u32),
            GeneratedStruct1 {
                age: 23i32,
                berries: true,
            } => Ok(25843u32),
            GeneratedStruct1 {
                berries: false,
                age: 23i32,
            } => Ok(25844u32),
            GeneratedStruct1 {
                berries: true,
                age: 24i32,
            } => Ok(25845u32),
            GeneratedStruct1 {
                berries: false,
                age: 24i32,
            } => Ok(25846u32),
            GeneratedStruct1 {
                age: 25i32,
                berries: true,
            } => Ok(25847u32),
            GeneratedStruct1 {
                berries: false,
                age: 25i32,
            } => Ok(25848u32),
            _ => Err(()),
        }
    }
}

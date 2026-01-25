use crate::GlowBerriesBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for GlowBerriesBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            25797u32 => Ok(GlowBerriesBlock {
                age: 0i32,
                berries: true,
            }),
            25798u32 => Ok(GlowBerriesBlock {
                age: 0i32,
                berries: false,
            }),
            25799u32 => Ok(GlowBerriesBlock {
                age: 1i32,
                berries: true,
            }),
            25800u32 => Ok(GlowBerriesBlock {
                age: 1i32,
                berries: false,
            }),
            25801u32 => Ok(GlowBerriesBlock {
                age: 2i32,
                berries: true,
            }),
            25802u32 => Ok(GlowBerriesBlock {
                age: 2i32,
                berries: false,
            }),
            25803u32 => Ok(GlowBerriesBlock {
                age: 3i32,
                berries: true,
            }),
            25804u32 => Ok(GlowBerriesBlock {
                age: 3i32,
                berries: false,
            }),
            25805u32 => Ok(GlowBerriesBlock {
                age: 4i32,
                berries: true,
            }),
            25806u32 => Ok(GlowBerriesBlock {
                age: 4i32,
                berries: false,
            }),
            25807u32 => Ok(GlowBerriesBlock {
                age: 5i32,
                berries: true,
            }),
            25808u32 => Ok(GlowBerriesBlock {
                age: 5i32,
                berries: false,
            }),
            25809u32 => Ok(GlowBerriesBlock {
                age: 6i32,
                berries: true,
            }),
            25810u32 => Ok(GlowBerriesBlock {
                age: 6i32,
                berries: false,
            }),
            25811u32 => Ok(GlowBerriesBlock {
                age: 7i32,
                berries: true,
            }),
            25812u32 => Ok(GlowBerriesBlock {
                age: 7i32,
                berries: false,
            }),
            25813u32 => Ok(GlowBerriesBlock {
                age: 8i32,
                berries: true,
            }),
            25814u32 => Ok(GlowBerriesBlock {
                age: 8i32,
                berries: false,
            }),
            25815u32 => Ok(GlowBerriesBlock {
                age: 9i32,
                berries: true,
            }),
            25816u32 => Ok(GlowBerriesBlock {
                age: 9i32,
                berries: false,
            }),
            25817u32 => Ok(GlowBerriesBlock {
                age: 10i32,
                berries: true,
            }),
            25818u32 => Ok(GlowBerriesBlock {
                age: 10i32,
                berries: false,
            }),
            25819u32 => Ok(GlowBerriesBlock {
                age: 11i32,
                berries: true,
            }),
            25820u32 => Ok(GlowBerriesBlock {
                age: 11i32,
                berries: false,
            }),
            25821u32 => Ok(GlowBerriesBlock {
                age: 12i32,
                berries: true,
            }),
            25822u32 => Ok(GlowBerriesBlock {
                age: 12i32,
                berries: false,
            }),
            25823u32 => Ok(GlowBerriesBlock {
                age: 13i32,
                berries: true,
            }),
            25824u32 => Ok(GlowBerriesBlock {
                age: 13i32,
                berries: false,
            }),
            25825u32 => Ok(GlowBerriesBlock {
                age: 14i32,
                berries: true,
            }),
            25826u32 => Ok(GlowBerriesBlock {
                age: 14i32,
                berries: false,
            }),
            25827u32 => Ok(GlowBerriesBlock {
                age: 15i32,
                berries: true,
            }),
            25828u32 => Ok(GlowBerriesBlock {
                age: 15i32,
                berries: false,
            }),
            25829u32 => Ok(GlowBerriesBlock {
                age: 16i32,
                berries: true,
            }),
            25830u32 => Ok(GlowBerriesBlock {
                age: 16i32,
                berries: false,
            }),
            25831u32 => Ok(GlowBerriesBlock {
                age: 17i32,
                berries: true,
            }),
            25832u32 => Ok(GlowBerriesBlock {
                age: 17i32,
                berries: false,
            }),
            25833u32 => Ok(GlowBerriesBlock {
                age: 18i32,
                berries: true,
            }),
            25834u32 => Ok(GlowBerriesBlock {
                age: 18i32,
                berries: false,
            }),
            25835u32 => Ok(GlowBerriesBlock {
                age: 19i32,
                berries: true,
            }),
            25836u32 => Ok(GlowBerriesBlock {
                age: 19i32,
                berries: false,
            }),
            25837u32 => Ok(GlowBerriesBlock {
                age: 20i32,
                berries: true,
            }),
            25838u32 => Ok(GlowBerriesBlock {
                age: 20i32,
                berries: false,
            }),
            25839u32 => Ok(GlowBerriesBlock {
                age: 21i32,
                berries: true,
            }),
            25840u32 => Ok(GlowBerriesBlock {
                age: 21i32,
                berries: false,
            }),
            25841u32 => Ok(GlowBerriesBlock {
                age: 22i32,
                berries: true,
            }),
            25842u32 => Ok(GlowBerriesBlock {
                age: 22i32,
                berries: false,
            }),
            25843u32 => Ok(GlowBerriesBlock {
                age: 23i32,
                berries: true,
            }),
            25844u32 => Ok(GlowBerriesBlock {
                age: 23i32,
                berries: false,
            }),
            25845u32 => Ok(GlowBerriesBlock {
                age: 24i32,
                berries: true,
            }),
            25846u32 => Ok(GlowBerriesBlock {
                age: 24i32,
                berries: false,
            }),
            25847u32 => Ok(GlowBerriesBlock {
                age: 25i32,
                berries: true,
            }),
            25848u32 => Ok(GlowBerriesBlock {
                age: 25i32,
                berries: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GlowBerriesBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GlowBerriesBlock {
                age: 0i32,
                berries: true,
            } => Ok(25797u32),
            GlowBerriesBlock {
                age: 0i32,
                berries: false,
            } => Ok(25798u32),
            GlowBerriesBlock {
                age: 1i32,
                berries: true,
            } => Ok(25799u32),
            GlowBerriesBlock {
                age: 1i32,
                berries: false,
            } => Ok(25800u32),
            GlowBerriesBlock {
                age: 2i32,
                berries: true,
            } => Ok(25801u32),
            GlowBerriesBlock {
                age: 2i32,
                berries: false,
            } => Ok(25802u32),
            GlowBerriesBlock {
                age: 3i32,
                berries: true,
            } => Ok(25803u32),
            GlowBerriesBlock {
                age: 3i32,
                berries: false,
            } => Ok(25804u32),
            GlowBerriesBlock {
                age: 4i32,
                berries: true,
            } => Ok(25805u32),
            GlowBerriesBlock {
                age: 4i32,
                berries: false,
            } => Ok(25806u32),
            GlowBerriesBlock {
                age: 5i32,
                berries: true,
            } => Ok(25807u32),
            GlowBerriesBlock {
                age: 5i32,
                berries: false,
            } => Ok(25808u32),
            GlowBerriesBlock {
                age: 6i32,
                berries: true,
            } => Ok(25809u32),
            GlowBerriesBlock {
                age: 6i32,
                berries: false,
            } => Ok(25810u32),
            GlowBerriesBlock {
                age: 7i32,
                berries: true,
            } => Ok(25811u32),
            GlowBerriesBlock {
                age: 7i32,
                berries: false,
            } => Ok(25812u32),
            GlowBerriesBlock {
                age: 8i32,
                berries: true,
            } => Ok(25813u32),
            GlowBerriesBlock {
                age: 8i32,
                berries: false,
            } => Ok(25814u32),
            GlowBerriesBlock {
                age: 9i32,
                berries: true,
            } => Ok(25815u32),
            GlowBerriesBlock {
                age: 9i32,
                berries: false,
            } => Ok(25816u32),
            GlowBerriesBlock {
                age: 10i32,
                berries: true,
            } => Ok(25817u32),
            GlowBerriesBlock {
                age: 10i32,
                berries: false,
            } => Ok(25818u32),
            GlowBerriesBlock {
                age: 11i32,
                berries: true,
            } => Ok(25819u32),
            GlowBerriesBlock {
                age: 11i32,
                berries: false,
            } => Ok(25820u32),
            GlowBerriesBlock {
                age: 12i32,
                berries: true,
            } => Ok(25821u32),
            GlowBerriesBlock {
                age: 12i32,
                berries: false,
            } => Ok(25822u32),
            GlowBerriesBlock {
                age: 13i32,
                berries: true,
            } => Ok(25823u32),
            GlowBerriesBlock {
                age: 13i32,
                berries: false,
            } => Ok(25824u32),
            GlowBerriesBlock {
                age: 14i32,
                berries: true,
            } => Ok(25825u32),
            GlowBerriesBlock {
                age: 14i32,
                berries: false,
            } => Ok(25826u32),
            GlowBerriesBlock {
                age: 15i32,
                berries: true,
            } => Ok(25827u32),
            GlowBerriesBlock {
                age: 15i32,
                berries: false,
            } => Ok(25828u32),
            GlowBerriesBlock {
                age: 16i32,
                berries: true,
            } => Ok(25829u32),
            GlowBerriesBlock {
                age: 16i32,
                berries: false,
            } => Ok(25830u32),
            GlowBerriesBlock {
                age: 17i32,
                berries: true,
            } => Ok(25831u32),
            GlowBerriesBlock {
                age: 17i32,
                berries: false,
            } => Ok(25832u32),
            GlowBerriesBlock {
                age: 18i32,
                berries: true,
            } => Ok(25833u32),
            GlowBerriesBlock {
                age: 18i32,
                berries: false,
            } => Ok(25834u32),
            GlowBerriesBlock {
                age: 19i32,
                berries: true,
            } => Ok(25835u32),
            GlowBerriesBlock {
                age: 19i32,
                berries: false,
            } => Ok(25836u32),
            GlowBerriesBlock {
                age: 20i32,
                berries: true,
            } => Ok(25837u32),
            GlowBerriesBlock {
                age: 20i32,
                berries: false,
            } => Ok(25838u32),
            GlowBerriesBlock {
                age: 21i32,
                berries: true,
            } => Ok(25839u32),
            GlowBerriesBlock {
                age: 21i32,
                berries: false,
            } => Ok(25840u32),
            GlowBerriesBlock {
                age: 22i32,
                berries: true,
            } => Ok(25841u32),
            GlowBerriesBlock {
                age: 22i32,
                berries: false,
            } => Ok(25842u32),
            GlowBerriesBlock {
                age: 23i32,
                berries: true,
            } => Ok(25843u32),
            GlowBerriesBlock {
                age: 23i32,
                berries: false,
            } => Ok(25844u32),
            GlowBerriesBlock {
                age: 24i32,
                berries: true,
            } => Ok(25845u32),
            GlowBerriesBlock {
                age: 24i32,
                berries: false,
            } => Ok(25846u32),
            GlowBerriesBlock {
                age: 25i32,
                berries: true,
            } => Ok(25847u32),
            GlowBerriesBlock {
                age: 25i32,
                berries: false,
            } => Ok(25848u32),
            _ => Err(()),
        }
    }
}

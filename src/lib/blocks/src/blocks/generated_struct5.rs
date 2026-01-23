#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GeneratedStruct5 {
    pub age: i32,
    pub hanging: bool,
    pub stage: i32,
    pub waterlogged: bool,
}
impl GeneratedStruct5 {
    pub(crate) const VTABLE: crate::BlockBehaviorTable =
        crate::BlockBehaviorTable::from::<GeneratedStruct5>();
}
impl TryFrom<u32> for GeneratedStruct5 {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            45u32 => Ok(GeneratedStruct5 {
                age: 0i32,
                hanging: true,
                stage: 0i32,
                waterlogged: true,
            }),
            46u32 => Ok(GeneratedStruct5 {
                age: 0i32,
                hanging: true,
                stage: 0i32,
                waterlogged: false,
            }),
            47u32 => Ok(GeneratedStruct5 {
                age: 0i32,
                hanging: true,
                stage: 1i32,
                waterlogged: true,
            }),
            48u32 => Ok(GeneratedStruct5 {
                age: 0i32,
                hanging: true,
                stage: 1i32,
                waterlogged: false,
            }),
            49u32 => Ok(GeneratedStruct5 {
                age: 0i32,
                hanging: false,
                stage: 0i32,
                waterlogged: true,
            }),
            50u32 => Ok(GeneratedStruct5 {
                age: 0i32,
                hanging: false,
                stage: 0i32,
                waterlogged: false,
            }),
            51u32 => Ok(GeneratedStruct5 {
                age: 0i32,
                hanging: false,
                stage: 1i32,
                waterlogged: true,
            }),
            52u32 => Ok(GeneratedStruct5 {
                age: 0i32,
                hanging: false,
                stage: 1i32,
                waterlogged: false,
            }),
            53u32 => Ok(GeneratedStruct5 {
                age: 1i32,
                hanging: true,
                stage: 0i32,
                waterlogged: true,
            }),
            54u32 => Ok(GeneratedStruct5 {
                age: 1i32,
                hanging: true,
                stage: 0i32,
                waterlogged: false,
            }),
            55u32 => Ok(GeneratedStruct5 {
                age: 1i32,
                hanging: true,
                stage: 1i32,
                waterlogged: true,
            }),
            56u32 => Ok(GeneratedStruct5 {
                age: 1i32,
                hanging: true,
                stage: 1i32,
                waterlogged: false,
            }),
            57u32 => Ok(GeneratedStruct5 {
                age: 1i32,
                hanging: false,
                stage: 0i32,
                waterlogged: true,
            }),
            58u32 => Ok(GeneratedStruct5 {
                age: 1i32,
                hanging: false,
                stage: 0i32,
                waterlogged: false,
            }),
            59u32 => Ok(GeneratedStruct5 {
                age: 1i32,
                hanging: false,
                stage: 1i32,
                waterlogged: true,
            }),
            60u32 => Ok(GeneratedStruct5 {
                age: 1i32,
                hanging: false,
                stage: 1i32,
                waterlogged: false,
            }),
            61u32 => Ok(GeneratedStruct5 {
                age: 2i32,
                hanging: true,
                stage: 0i32,
                waterlogged: true,
            }),
            62u32 => Ok(GeneratedStruct5 {
                age: 2i32,
                hanging: true,
                stage: 0i32,
                waterlogged: false,
            }),
            63u32 => Ok(GeneratedStruct5 {
                age: 2i32,
                hanging: true,
                stage: 1i32,
                waterlogged: true,
            }),
            64u32 => Ok(GeneratedStruct5 {
                age: 2i32,
                hanging: true,
                stage: 1i32,
                waterlogged: false,
            }),
            65u32 => Ok(GeneratedStruct5 {
                age: 2i32,
                hanging: false,
                stage: 0i32,
                waterlogged: true,
            }),
            66u32 => Ok(GeneratedStruct5 {
                age: 2i32,
                hanging: false,
                stage: 0i32,
                waterlogged: false,
            }),
            67u32 => Ok(GeneratedStruct5 {
                age: 2i32,
                hanging: false,
                stage: 1i32,
                waterlogged: true,
            }),
            68u32 => Ok(GeneratedStruct5 {
                age: 2i32,
                hanging: false,
                stage: 1i32,
                waterlogged: false,
            }),
            69u32 => Ok(GeneratedStruct5 {
                age: 3i32,
                hanging: true,
                stage: 0i32,
                waterlogged: true,
            }),
            70u32 => Ok(GeneratedStruct5 {
                age: 3i32,
                hanging: true,
                stage: 0i32,
                waterlogged: false,
            }),
            71u32 => Ok(GeneratedStruct5 {
                age: 3i32,
                hanging: true,
                stage: 1i32,
                waterlogged: true,
            }),
            72u32 => Ok(GeneratedStruct5 {
                age: 3i32,
                hanging: true,
                stage: 1i32,
                waterlogged: false,
            }),
            73u32 => Ok(GeneratedStruct5 {
                age: 3i32,
                hanging: false,
                stage: 0i32,
                waterlogged: true,
            }),
            74u32 => Ok(GeneratedStruct5 {
                age: 3i32,
                hanging: false,
                stage: 0i32,
                waterlogged: false,
            }),
            75u32 => Ok(GeneratedStruct5 {
                age: 3i32,
                hanging: false,
                stage: 1i32,
                waterlogged: true,
            }),
            76u32 => Ok(GeneratedStruct5 {
                age: 3i32,
                hanging: false,
                stage: 1i32,
                waterlogged: false,
            }),
            77u32 => Ok(GeneratedStruct5 {
                age: 4i32,
                hanging: true,
                stage: 0i32,
                waterlogged: true,
            }),
            78u32 => Ok(GeneratedStruct5 {
                age: 4i32,
                hanging: true,
                stage: 0i32,
                waterlogged: false,
            }),
            79u32 => Ok(GeneratedStruct5 {
                age: 4i32,
                hanging: true,
                stage: 1i32,
                waterlogged: true,
            }),
            80u32 => Ok(GeneratedStruct5 {
                age: 4i32,
                hanging: true,
                stage: 1i32,
                waterlogged: false,
            }),
            81u32 => Ok(GeneratedStruct5 {
                age: 4i32,
                hanging: false,
                stage: 0i32,
                waterlogged: true,
            }),
            82u32 => Ok(GeneratedStruct5 {
                age: 4i32,
                hanging: false,
                stage: 0i32,
                waterlogged: false,
            }),
            83u32 => Ok(GeneratedStruct5 {
                age: 4i32,
                hanging: false,
                stage: 1i32,
                waterlogged: true,
            }),
            84u32 => Ok(GeneratedStruct5 {
                age: 4i32,
                hanging: false,
                stage: 1i32,
                waterlogged: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct5 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct5 {
                age: 0i32,
                hanging: true,
                stage: 0i32,
                waterlogged: true,
            } => Ok(45u32),
            GeneratedStruct5 {
                age: 0i32,
                hanging: true,
                stage: 0i32,
                waterlogged: false,
            } => Ok(46u32),
            GeneratedStruct5 {
                age: 0i32,
                hanging: true,
                stage: 1i32,
                waterlogged: true,
            } => Ok(47u32),
            GeneratedStruct5 {
                age: 0i32,
                hanging: true,
                stage: 1i32,
                waterlogged: false,
            } => Ok(48u32),
            GeneratedStruct5 {
                age: 0i32,
                hanging: false,
                stage: 0i32,
                waterlogged: true,
            } => Ok(49u32),
            GeneratedStruct5 {
                age: 0i32,
                hanging: false,
                stage: 0i32,
                waterlogged: false,
            } => Ok(50u32),
            GeneratedStruct5 {
                age: 0i32,
                hanging: false,
                stage: 1i32,
                waterlogged: true,
            } => Ok(51u32),
            GeneratedStruct5 {
                age: 0i32,
                hanging: false,
                stage: 1i32,
                waterlogged: false,
            } => Ok(52u32),
            GeneratedStruct5 {
                age: 1i32,
                hanging: true,
                stage: 0i32,
                waterlogged: true,
            } => Ok(53u32),
            GeneratedStruct5 {
                age: 1i32,
                hanging: true,
                stage: 0i32,
                waterlogged: false,
            } => Ok(54u32),
            GeneratedStruct5 {
                age: 1i32,
                hanging: true,
                stage: 1i32,
                waterlogged: true,
            } => Ok(55u32),
            GeneratedStruct5 {
                age: 1i32,
                hanging: true,
                stage: 1i32,
                waterlogged: false,
            } => Ok(56u32),
            GeneratedStruct5 {
                age: 1i32,
                hanging: false,
                stage: 0i32,
                waterlogged: true,
            } => Ok(57u32),
            GeneratedStruct5 {
                age: 1i32,
                hanging: false,
                stage: 0i32,
                waterlogged: false,
            } => Ok(58u32),
            GeneratedStruct5 {
                age: 1i32,
                hanging: false,
                stage: 1i32,
                waterlogged: true,
            } => Ok(59u32),
            GeneratedStruct5 {
                age: 1i32,
                hanging: false,
                stage: 1i32,
                waterlogged: false,
            } => Ok(60u32),
            GeneratedStruct5 {
                age: 2i32,
                hanging: true,
                stage: 0i32,
                waterlogged: true,
            } => Ok(61u32),
            GeneratedStruct5 {
                age: 2i32,
                hanging: true,
                stage: 0i32,
                waterlogged: false,
            } => Ok(62u32),
            GeneratedStruct5 {
                age: 2i32,
                hanging: true,
                stage: 1i32,
                waterlogged: true,
            } => Ok(63u32),
            GeneratedStruct5 {
                age: 2i32,
                hanging: true,
                stage: 1i32,
                waterlogged: false,
            } => Ok(64u32),
            GeneratedStruct5 {
                age: 2i32,
                hanging: false,
                stage: 0i32,
                waterlogged: true,
            } => Ok(65u32),
            GeneratedStruct5 {
                age: 2i32,
                hanging: false,
                stage: 0i32,
                waterlogged: false,
            } => Ok(66u32),
            GeneratedStruct5 {
                age: 2i32,
                hanging: false,
                stage: 1i32,
                waterlogged: true,
            } => Ok(67u32),
            GeneratedStruct5 {
                age: 2i32,
                hanging: false,
                stage: 1i32,
                waterlogged: false,
            } => Ok(68u32),
            GeneratedStruct5 {
                age: 3i32,
                hanging: true,
                stage: 0i32,
                waterlogged: true,
            } => Ok(69u32),
            GeneratedStruct5 {
                age: 3i32,
                hanging: true,
                stage: 0i32,
                waterlogged: false,
            } => Ok(70u32),
            GeneratedStruct5 {
                age: 3i32,
                hanging: true,
                stage: 1i32,
                waterlogged: true,
            } => Ok(71u32),
            GeneratedStruct5 {
                age: 3i32,
                hanging: true,
                stage: 1i32,
                waterlogged: false,
            } => Ok(72u32),
            GeneratedStruct5 {
                age: 3i32,
                hanging: false,
                stage: 0i32,
                waterlogged: true,
            } => Ok(73u32),
            GeneratedStruct5 {
                age: 3i32,
                hanging: false,
                stage: 0i32,
                waterlogged: false,
            } => Ok(74u32),
            GeneratedStruct5 {
                age: 3i32,
                hanging: false,
                stage: 1i32,
                waterlogged: true,
            } => Ok(75u32),
            GeneratedStruct5 {
                age: 3i32,
                hanging: false,
                stage: 1i32,
                waterlogged: false,
            } => Ok(76u32),
            GeneratedStruct5 {
                age: 4i32,
                hanging: true,
                stage: 0i32,
                waterlogged: true,
            } => Ok(77u32),
            GeneratedStruct5 {
                age: 4i32,
                hanging: true,
                stage: 0i32,
                waterlogged: false,
            } => Ok(78u32),
            GeneratedStruct5 {
                age: 4i32,
                hanging: true,
                stage: 1i32,
                waterlogged: true,
            } => Ok(79u32),
            GeneratedStruct5 {
                age: 4i32,
                hanging: true,
                stage: 1i32,
                waterlogged: false,
            } => Ok(80u32),
            GeneratedStruct5 {
                age: 4i32,
                hanging: false,
                stage: 0i32,
                waterlogged: true,
            } => Ok(81u32),
            GeneratedStruct5 {
                age: 4i32,
                hanging: false,
                stage: 0i32,
                waterlogged: false,
            } => Ok(82u32),
            GeneratedStruct5 {
                age: 4i32,
                hanging: false,
                stage: 1i32,
                waterlogged: true,
            } => Ok(83u32),
            GeneratedStruct5 {
                age: 4i32,
                hanging: false,
                stage: 1i32,
                waterlogged: false,
            } => Ok(84u32),
            _ => Err(()),
        }
    }
}

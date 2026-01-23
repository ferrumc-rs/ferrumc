#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub struct GeneratedStruct76 {
    pub level: i32,
    pub waterlogged: bool,
}
impl TryFrom<u32> for GeneratedStruct76 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            11256u32 => Ok(GeneratedStruct76 {
                level: 0i32,
                waterlogged: true,
            }),
            11257u32 => Ok(GeneratedStruct76 {
                waterlogged: false,
                level: 0i32,
            }),
            11258u32 => Ok(GeneratedStruct76 {
                level: 1i32,
                waterlogged: true,
            }),
            11259u32 => Ok(GeneratedStruct76 {
                waterlogged: false,
                level: 1i32,
            }),
            11260u32 => Ok(GeneratedStruct76 {
                level: 2i32,
                waterlogged: true,
            }),
            11261u32 => Ok(GeneratedStruct76 {
                level: 2i32,
                waterlogged: false,
            }),
            11262u32 => Ok(GeneratedStruct76 {
                level: 3i32,
                waterlogged: true,
            }),
            11263u32 => Ok(GeneratedStruct76 {
                level: 3i32,
                waterlogged: false,
            }),
            11264u32 => Ok(GeneratedStruct76 {
                waterlogged: true,
                level: 4i32,
            }),
            11265u32 => Ok(GeneratedStruct76 {
                level: 4i32,
                waterlogged: false,
            }),
            11266u32 => Ok(GeneratedStruct76 {
                level: 5i32,
                waterlogged: true,
            }),
            11267u32 => Ok(GeneratedStruct76 {
                waterlogged: false,
                level: 5i32,
            }),
            11268u32 => Ok(GeneratedStruct76 {
                waterlogged: true,
                level: 6i32,
            }),
            11269u32 => Ok(GeneratedStruct76 {
                waterlogged: false,
                level: 6i32,
            }),
            11270u32 => Ok(GeneratedStruct76 {
                level: 7i32,
                waterlogged: true,
            }),
            11271u32 => Ok(GeneratedStruct76 {
                waterlogged: false,
                level: 7i32,
            }),
            11272u32 => Ok(GeneratedStruct76 {
                waterlogged: true,
                level: 8i32,
            }),
            11273u32 => Ok(GeneratedStruct76 {
                level: 8i32,
                waterlogged: false,
            }),
            11274u32 => Ok(GeneratedStruct76 {
                level: 9i32,
                waterlogged: true,
            }),
            11275u32 => Ok(GeneratedStruct76 {
                waterlogged: false,
                level: 9i32,
            }),
            11276u32 => Ok(GeneratedStruct76 {
                waterlogged: true,
                level: 10i32,
            }),
            11277u32 => Ok(GeneratedStruct76 {
                level: 10i32,
                waterlogged: false,
            }),
            11278u32 => Ok(GeneratedStruct76 {
                level: 11i32,
                waterlogged: true,
            }),
            11279u32 => Ok(GeneratedStruct76 {
                waterlogged: false,
                level: 11i32,
            }),
            11280u32 => Ok(GeneratedStruct76 {
                level: 12i32,
                waterlogged: true,
            }),
            11281u32 => Ok(GeneratedStruct76 {
                waterlogged: false,
                level: 12i32,
            }),
            11282u32 => Ok(GeneratedStruct76 {
                level: 13i32,
                waterlogged: true,
            }),
            11283u32 => Ok(GeneratedStruct76 {
                level: 13i32,
                waterlogged: false,
            }),
            11284u32 => Ok(GeneratedStruct76 {
                level: 14i32,
                waterlogged: true,
            }),
            11285u32 => Ok(GeneratedStruct76 {
                waterlogged: false,
                level: 14i32,
            }),
            11286u32 => Ok(GeneratedStruct76 {
                waterlogged: true,
                level: 15i32,
            }),
            11287u32 => Ok(GeneratedStruct76 {
                waterlogged: false,
                level: 15i32,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct76 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct76 {
                level: 0i32,
                waterlogged: true,
            } => Ok(11256u32),
            GeneratedStruct76 {
                waterlogged: false,
                level: 0i32,
            } => Ok(11257u32),
            GeneratedStruct76 {
                level: 1i32,
                waterlogged: true,
            } => Ok(11258u32),
            GeneratedStruct76 {
                waterlogged: false,
                level: 1i32,
            } => Ok(11259u32),
            GeneratedStruct76 {
                level: 2i32,
                waterlogged: true,
            } => Ok(11260u32),
            GeneratedStruct76 {
                level: 2i32,
                waterlogged: false,
            } => Ok(11261u32),
            GeneratedStruct76 {
                level: 3i32,
                waterlogged: true,
            } => Ok(11262u32),
            GeneratedStruct76 {
                level: 3i32,
                waterlogged: false,
            } => Ok(11263u32),
            GeneratedStruct76 {
                waterlogged: true,
                level: 4i32,
            } => Ok(11264u32),
            GeneratedStruct76 {
                level: 4i32,
                waterlogged: false,
            } => Ok(11265u32),
            GeneratedStruct76 {
                level: 5i32,
                waterlogged: true,
            } => Ok(11266u32),
            GeneratedStruct76 {
                waterlogged: false,
                level: 5i32,
            } => Ok(11267u32),
            GeneratedStruct76 {
                waterlogged: true,
                level: 6i32,
            } => Ok(11268u32),
            GeneratedStruct76 {
                waterlogged: false,
                level: 6i32,
            } => Ok(11269u32),
            GeneratedStruct76 {
                level: 7i32,
                waterlogged: true,
            } => Ok(11270u32),
            GeneratedStruct76 {
                waterlogged: false,
                level: 7i32,
            } => Ok(11271u32),
            GeneratedStruct76 {
                waterlogged: true,
                level: 8i32,
            } => Ok(11272u32),
            GeneratedStruct76 {
                level: 8i32,
                waterlogged: false,
            } => Ok(11273u32),
            GeneratedStruct76 {
                level: 9i32,
                waterlogged: true,
            } => Ok(11274u32),
            GeneratedStruct76 {
                waterlogged: false,
                level: 9i32,
            } => Ok(11275u32),
            GeneratedStruct76 {
                waterlogged: true,
                level: 10i32,
            } => Ok(11276u32),
            GeneratedStruct76 {
                level: 10i32,
                waterlogged: false,
            } => Ok(11277u32),
            GeneratedStruct76 {
                level: 11i32,
                waterlogged: true,
            } => Ok(11278u32),
            GeneratedStruct76 {
                waterlogged: false,
                level: 11i32,
            } => Ok(11279u32),
            GeneratedStruct76 {
                level: 12i32,
                waterlogged: true,
            } => Ok(11280u32),
            GeneratedStruct76 {
                waterlogged: false,
                level: 12i32,
            } => Ok(11281u32),
            GeneratedStruct76 {
                level: 13i32,
                waterlogged: true,
            } => Ok(11282u32),
            GeneratedStruct76 {
                level: 13i32,
                waterlogged: false,
            } => Ok(11283u32),
            GeneratedStruct76 {
                level: 14i32,
                waterlogged: true,
            } => Ok(11284u32),
            GeneratedStruct76 {
                waterlogged: false,
                level: 14i32,
            } => Ok(11285u32),
            GeneratedStruct76 {
                waterlogged: true,
                level: 15i32,
            } => Ok(11286u32),
            GeneratedStruct76 {
                waterlogged: false,
                level: 15i32,
            } => Ok(11287u32),
            _ => Err(()),
        }
    }
}

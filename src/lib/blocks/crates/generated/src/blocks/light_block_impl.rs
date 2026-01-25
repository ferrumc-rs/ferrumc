use crate::LightBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for LightBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            11256u32 => Ok(LightBlock {
                level: 0i32,
                waterlogged: true,
            }),
            11257u32 => Ok(LightBlock {
                level: 0i32,
                waterlogged: false,
            }),
            11258u32 => Ok(LightBlock {
                level: 1i32,
                waterlogged: true,
            }),
            11259u32 => Ok(LightBlock {
                level: 1i32,
                waterlogged: false,
            }),
            11260u32 => Ok(LightBlock {
                level: 2i32,
                waterlogged: true,
            }),
            11261u32 => Ok(LightBlock {
                level: 2i32,
                waterlogged: false,
            }),
            11262u32 => Ok(LightBlock {
                level: 3i32,
                waterlogged: true,
            }),
            11263u32 => Ok(LightBlock {
                level: 3i32,
                waterlogged: false,
            }),
            11264u32 => Ok(LightBlock {
                level: 4i32,
                waterlogged: true,
            }),
            11265u32 => Ok(LightBlock {
                level: 4i32,
                waterlogged: false,
            }),
            11266u32 => Ok(LightBlock {
                level: 5i32,
                waterlogged: true,
            }),
            11267u32 => Ok(LightBlock {
                level: 5i32,
                waterlogged: false,
            }),
            11268u32 => Ok(LightBlock {
                level: 6i32,
                waterlogged: true,
            }),
            11269u32 => Ok(LightBlock {
                level: 6i32,
                waterlogged: false,
            }),
            11270u32 => Ok(LightBlock {
                level: 7i32,
                waterlogged: true,
            }),
            11271u32 => Ok(LightBlock {
                level: 7i32,
                waterlogged: false,
            }),
            11272u32 => Ok(LightBlock {
                level: 8i32,
                waterlogged: true,
            }),
            11273u32 => Ok(LightBlock {
                level: 8i32,
                waterlogged: false,
            }),
            11274u32 => Ok(LightBlock {
                level: 9i32,
                waterlogged: true,
            }),
            11275u32 => Ok(LightBlock {
                level: 9i32,
                waterlogged: false,
            }),
            11276u32 => Ok(LightBlock {
                level: 10i32,
                waterlogged: true,
            }),
            11277u32 => Ok(LightBlock {
                level: 10i32,
                waterlogged: false,
            }),
            11278u32 => Ok(LightBlock {
                level: 11i32,
                waterlogged: true,
            }),
            11279u32 => Ok(LightBlock {
                level: 11i32,
                waterlogged: false,
            }),
            11280u32 => Ok(LightBlock {
                level: 12i32,
                waterlogged: true,
            }),
            11281u32 => Ok(LightBlock {
                level: 12i32,
                waterlogged: false,
            }),
            11282u32 => Ok(LightBlock {
                level: 13i32,
                waterlogged: true,
            }),
            11283u32 => Ok(LightBlock {
                level: 13i32,
                waterlogged: false,
            }),
            11284u32 => Ok(LightBlock {
                level: 14i32,
                waterlogged: true,
            }),
            11285u32 => Ok(LightBlock {
                level: 14i32,
                waterlogged: false,
            }),
            11286u32 => Ok(LightBlock {
                level: 15i32,
                waterlogged: true,
            }),
            11287u32 => Ok(LightBlock {
                level: 15i32,
                waterlogged: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for LightBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            LightBlock {
                level: 0i32,
                waterlogged: true,
            } => Ok(11256u32),
            LightBlock {
                level: 0i32,
                waterlogged: false,
            } => Ok(11257u32),
            LightBlock {
                level: 1i32,
                waterlogged: true,
            } => Ok(11258u32),
            LightBlock {
                level: 1i32,
                waterlogged: false,
            } => Ok(11259u32),
            LightBlock {
                level: 2i32,
                waterlogged: true,
            } => Ok(11260u32),
            LightBlock {
                level: 2i32,
                waterlogged: false,
            } => Ok(11261u32),
            LightBlock {
                level: 3i32,
                waterlogged: true,
            } => Ok(11262u32),
            LightBlock {
                level: 3i32,
                waterlogged: false,
            } => Ok(11263u32),
            LightBlock {
                level: 4i32,
                waterlogged: true,
            } => Ok(11264u32),
            LightBlock {
                level: 4i32,
                waterlogged: false,
            } => Ok(11265u32),
            LightBlock {
                level: 5i32,
                waterlogged: true,
            } => Ok(11266u32),
            LightBlock {
                level: 5i32,
                waterlogged: false,
            } => Ok(11267u32),
            LightBlock {
                level: 6i32,
                waterlogged: true,
            } => Ok(11268u32),
            LightBlock {
                level: 6i32,
                waterlogged: false,
            } => Ok(11269u32),
            LightBlock {
                level: 7i32,
                waterlogged: true,
            } => Ok(11270u32),
            LightBlock {
                level: 7i32,
                waterlogged: false,
            } => Ok(11271u32),
            LightBlock {
                level: 8i32,
                waterlogged: true,
            } => Ok(11272u32),
            LightBlock {
                level: 8i32,
                waterlogged: false,
            } => Ok(11273u32),
            LightBlock {
                level: 9i32,
                waterlogged: true,
            } => Ok(11274u32),
            LightBlock {
                level: 9i32,
                waterlogged: false,
            } => Ok(11275u32),
            LightBlock {
                level: 10i32,
                waterlogged: true,
            } => Ok(11276u32),
            LightBlock {
                level: 10i32,
                waterlogged: false,
            } => Ok(11277u32),
            LightBlock {
                level: 11i32,
                waterlogged: true,
            } => Ok(11278u32),
            LightBlock {
                level: 11i32,
                waterlogged: false,
            } => Ok(11279u32),
            LightBlock {
                level: 12i32,
                waterlogged: true,
            } => Ok(11280u32),
            LightBlock {
                level: 12i32,
                waterlogged: false,
            } => Ok(11281u32),
            LightBlock {
                level: 13i32,
                waterlogged: true,
            } => Ok(11282u32),
            LightBlock {
                level: 13i32,
                waterlogged: false,
            } => Ok(11283u32),
            LightBlock {
                level: 14i32,
                waterlogged: true,
            } => Ok(11284u32),
            LightBlock {
                level: 14i32,
                waterlogged: false,
            } => Ok(11285u32),
            LightBlock {
                level: 15i32,
                waterlogged: true,
            } => Ok(11286u32),
            LightBlock {
                level: 15i32,
                waterlogged: false,
            } => Ok(11287u32),
            _ => Err(()),
        }
    }
}

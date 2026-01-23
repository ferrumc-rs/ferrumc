#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub struct GeneratedStruct73 {
    pub inverted: bool,
    pub power: i32,
}
impl TryFrom<u32> for GeneratedStruct73 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            10000u32 => Ok(GeneratedStruct73 {
                power: 0i32,
                inverted: true,
            }),
            10001u32 => Ok(GeneratedStruct73 {
                power: 1i32,
                inverted: true,
            }),
            10002u32 => Ok(GeneratedStruct73 {
                power: 2i32,
                inverted: true,
            }),
            10003u32 => Ok(GeneratedStruct73 {
                inverted: true,
                power: 3i32,
            }),
            10004u32 => Ok(GeneratedStruct73 {
                inverted: true,
                power: 4i32,
            }),
            10005u32 => Ok(GeneratedStruct73 {
                power: 5i32,
                inverted: true,
            }),
            10006u32 => Ok(GeneratedStruct73 {
                inverted: true,
                power: 6i32,
            }),
            10007u32 => Ok(GeneratedStruct73 {
                inverted: true,
                power: 7i32,
            }),
            10008u32 => Ok(GeneratedStruct73 {
                inverted: true,
                power: 8i32,
            }),
            10009u32 => Ok(GeneratedStruct73 {
                inverted: true,
                power: 9i32,
            }),
            10010u32 => Ok(GeneratedStruct73 {
                inverted: true,
                power: 10i32,
            }),
            10011u32 => Ok(GeneratedStruct73 {
                inverted: true,
                power: 11i32,
            }),
            10012u32 => Ok(GeneratedStruct73 {
                power: 12i32,
                inverted: true,
            }),
            10013u32 => Ok(GeneratedStruct73 {
                power: 13i32,
                inverted: true,
            }),
            10014u32 => Ok(GeneratedStruct73 {
                inverted: true,
                power: 14i32,
            }),
            10015u32 => Ok(GeneratedStruct73 {
                inverted: true,
                power: 15i32,
            }),
            10016u32 => Ok(GeneratedStruct73 {
                inverted: false,
                power: 0i32,
            }),
            10017u32 => Ok(GeneratedStruct73 {
                power: 1i32,
                inverted: false,
            }),
            10018u32 => Ok(GeneratedStruct73 {
                inverted: false,
                power: 2i32,
            }),
            10019u32 => Ok(GeneratedStruct73 {
                inverted: false,
                power: 3i32,
            }),
            10020u32 => Ok(GeneratedStruct73 {
                power: 4i32,
                inverted: false,
            }),
            10021u32 => Ok(GeneratedStruct73 {
                inverted: false,
                power: 5i32,
            }),
            10022u32 => Ok(GeneratedStruct73 {
                inverted: false,
                power: 6i32,
            }),
            10023u32 => Ok(GeneratedStruct73 {
                power: 7i32,
                inverted: false,
            }),
            10024u32 => Ok(GeneratedStruct73 {
                inverted: false,
                power: 8i32,
            }),
            10025u32 => Ok(GeneratedStruct73 {
                inverted: false,
                power: 9i32,
            }),
            10026u32 => Ok(GeneratedStruct73 {
                inverted: false,
                power: 10i32,
            }),
            10027u32 => Ok(GeneratedStruct73 {
                inverted: false,
                power: 11i32,
            }),
            10028u32 => Ok(GeneratedStruct73 {
                power: 12i32,
                inverted: false,
            }),
            10029u32 => Ok(GeneratedStruct73 {
                power: 13i32,
                inverted: false,
            }),
            10030u32 => Ok(GeneratedStruct73 {
                inverted: false,
                power: 14i32,
            }),
            10031u32 => Ok(GeneratedStruct73 {
                power: 15i32,
                inverted: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct73 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct73 {
                power: 0i32,
                inverted: true,
            } => Ok(10000u32),
            GeneratedStruct73 {
                power: 1i32,
                inverted: true,
            } => Ok(10001u32),
            GeneratedStruct73 {
                power: 2i32,
                inverted: true,
            } => Ok(10002u32),
            GeneratedStruct73 {
                inverted: true,
                power: 3i32,
            } => Ok(10003u32),
            GeneratedStruct73 {
                inverted: true,
                power: 4i32,
            } => Ok(10004u32),
            GeneratedStruct73 {
                power: 5i32,
                inverted: true,
            } => Ok(10005u32),
            GeneratedStruct73 {
                inverted: true,
                power: 6i32,
            } => Ok(10006u32),
            GeneratedStruct73 {
                inverted: true,
                power: 7i32,
            } => Ok(10007u32),
            GeneratedStruct73 {
                inverted: true,
                power: 8i32,
            } => Ok(10008u32),
            GeneratedStruct73 {
                inverted: true,
                power: 9i32,
            } => Ok(10009u32),
            GeneratedStruct73 {
                inverted: true,
                power: 10i32,
            } => Ok(10010u32),
            GeneratedStruct73 {
                inverted: true,
                power: 11i32,
            } => Ok(10011u32),
            GeneratedStruct73 {
                power: 12i32,
                inverted: true,
            } => Ok(10012u32),
            GeneratedStruct73 {
                power: 13i32,
                inverted: true,
            } => Ok(10013u32),
            GeneratedStruct73 {
                inverted: true,
                power: 14i32,
            } => Ok(10014u32),
            GeneratedStruct73 {
                inverted: true,
                power: 15i32,
            } => Ok(10015u32),
            GeneratedStruct73 {
                inverted: false,
                power: 0i32,
            } => Ok(10016u32),
            GeneratedStruct73 {
                power: 1i32,
                inverted: false,
            } => Ok(10017u32),
            GeneratedStruct73 {
                inverted: false,
                power: 2i32,
            } => Ok(10018u32),
            GeneratedStruct73 {
                inverted: false,
                power: 3i32,
            } => Ok(10019u32),
            GeneratedStruct73 {
                power: 4i32,
                inverted: false,
            } => Ok(10020u32),
            GeneratedStruct73 {
                inverted: false,
                power: 5i32,
            } => Ok(10021u32),
            GeneratedStruct73 {
                inverted: false,
                power: 6i32,
            } => Ok(10022u32),
            GeneratedStruct73 {
                power: 7i32,
                inverted: false,
            } => Ok(10023u32),
            GeneratedStruct73 {
                inverted: false,
                power: 8i32,
            } => Ok(10024u32),
            GeneratedStruct73 {
                inverted: false,
                power: 9i32,
            } => Ok(10025u32),
            GeneratedStruct73 {
                inverted: false,
                power: 10i32,
            } => Ok(10026u32),
            GeneratedStruct73 {
                inverted: false,
                power: 11i32,
            } => Ok(10027u32),
            GeneratedStruct73 {
                power: 12i32,
                inverted: false,
            } => Ok(10028u32),
            GeneratedStruct73 {
                power: 13i32,
                inverted: false,
            } => Ok(10029u32),
            GeneratedStruct73 {
                inverted: false,
                power: 14i32,
            } => Ok(10030u32),
            GeneratedStruct73 {
                power: 15i32,
                inverted: false,
            } => Ok(10031u32),
            _ => Err(()),
        }
    }
}

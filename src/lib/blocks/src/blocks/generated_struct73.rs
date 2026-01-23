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
                inverted: true,
                power: 0i32,
            }),
            10001u32 => Ok(GeneratedStruct73 {
                inverted: true,
                power: 1i32,
            }),
            10002u32 => Ok(GeneratedStruct73 {
                inverted: true,
                power: 2i32,
            }),
            10003u32 => Ok(GeneratedStruct73 {
                power: 3i32,
                inverted: true,
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
                power: 7i32,
                inverted: true,
            }),
            10008u32 => Ok(GeneratedStruct73 {
                power: 8i32,
                inverted: true,
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
                inverted: true,
                power: 12i32,
            }),
            10013u32 => Ok(GeneratedStruct73 {
                power: 13i32,
                inverted: true,
            }),
            10014u32 => Ok(GeneratedStruct73 {
                power: 14i32,
                inverted: true,
            }),
            10015u32 => Ok(GeneratedStruct73 {
                inverted: true,
                power: 15i32,
            }),
            10016u32 => Ok(GeneratedStruct73 {
                power: 0i32,
                inverted: false,
            }),
            10017u32 => Ok(GeneratedStruct73 {
                inverted: false,
                power: 1i32,
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
                inverted: false,
                power: 4i32,
            }),
            10021u32 => Ok(GeneratedStruct73 {
                inverted: false,
                power: 5i32,
            }),
            10022u32 => Ok(GeneratedStruct73 {
                power: 6i32,
                inverted: false,
            }),
            10023u32 => Ok(GeneratedStruct73 {
                inverted: false,
                power: 7i32,
            }),
            10024u32 => Ok(GeneratedStruct73 {
                power: 8i32,
                inverted: false,
            }),
            10025u32 => Ok(GeneratedStruct73 {
                inverted: false,
                power: 9i32,
            }),
            10026u32 => Ok(GeneratedStruct73 {
                power: 10i32,
                inverted: false,
            }),
            10027u32 => Ok(GeneratedStruct73 {
                inverted: false,
                power: 11i32,
            }),
            10028u32 => Ok(GeneratedStruct73 {
                inverted: false,
                power: 12i32,
            }),
            10029u32 => Ok(GeneratedStruct73 {
                power: 13i32,
                inverted: false,
            }),
            10030u32 => Ok(GeneratedStruct73 {
                power: 14i32,
                inverted: false,
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
                inverted: true,
                power: 0i32,
            } => Ok(10000u32),
            GeneratedStruct73 {
                inverted: true,
                power: 1i32,
            } => Ok(10001u32),
            GeneratedStruct73 {
                inverted: true,
                power: 2i32,
            } => Ok(10002u32),
            GeneratedStruct73 {
                power: 3i32,
                inverted: true,
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
                power: 7i32,
                inverted: true,
            } => Ok(10007u32),
            GeneratedStruct73 {
                power: 8i32,
                inverted: true,
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
                inverted: true,
                power: 12i32,
            } => Ok(10012u32),
            GeneratedStruct73 {
                power: 13i32,
                inverted: true,
            } => Ok(10013u32),
            GeneratedStruct73 {
                power: 14i32,
                inverted: true,
            } => Ok(10014u32),
            GeneratedStruct73 {
                inverted: true,
                power: 15i32,
            } => Ok(10015u32),
            GeneratedStruct73 {
                power: 0i32,
                inverted: false,
            } => Ok(10016u32),
            GeneratedStruct73 {
                inverted: false,
                power: 1i32,
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
                inverted: false,
                power: 4i32,
            } => Ok(10020u32),
            GeneratedStruct73 {
                inverted: false,
                power: 5i32,
            } => Ok(10021u32),
            GeneratedStruct73 {
                power: 6i32,
                inverted: false,
            } => Ok(10022u32),
            GeneratedStruct73 {
                inverted: false,
                power: 7i32,
            } => Ok(10023u32),
            GeneratedStruct73 {
                power: 8i32,
                inverted: false,
            } => Ok(10024u32),
            GeneratedStruct73 {
                inverted: false,
                power: 9i32,
            } => Ok(10025u32),
            GeneratedStruct73 {
                power: 10i32,
                inverted: false,
            } => Ok(10026u32),
            GeneratedStruct73 {
                inverted: false,
                power: 11i32,
            } => Ok(10027u32),
            GeneratedStruct73 {
                inverted: false,
                power: 12i32,
            } => Ok(10028u32),
            GeneratedStruct73 {
                power: 13i32,
                inverted: false,
            } => Ok(10029u32),
            GeneratedStruct73 {
                power: 14i32,
                inverted: false,
            } => Ok(10030u32),
            GeneratedStruct73 {
                power: 15i32,
                inverted: false,
            } => Ok(10031u32),
            _ => Err(()),
        }
    }
}

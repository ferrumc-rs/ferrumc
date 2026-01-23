#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
pub struct GeneratedStruct24 {
    pub crafting: bool,
    pub orientation: FrontAndTop,
    pub triggered: bool,
}
impl TryFrom<u32> for GeneratedStruct24 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            27650u32 => Ok(GeneratedStruct24 {
                crafting: true,
                triggered: true,
                orientation: FrontAndTop::DownEast,
            }),
            27651u32 => Ok(GeneratedStruct24 {
                triggered: false,
                crafting: true,
                orientation: FrontAndTop::DownEast,
            }),
            27652u32 => Ok(GeneratedStruct24 {
                triggered: true,
                crafting: true,
                orientation: FrontAndTop::DownNorth,
            }),
            27653u32 => Ok(GeneratedStruct24 {
                crafting: true,
                orientation: FrontAndTop::DownNorth,
                triggered: false,
            }),
            27654u32 => Ok(GeneratedStruct24 {
                triggered: true,
                crafting: true,
                orientation: FrontAndTop::DownSouth,
            }),
            27655u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::DownSouth,
                crafting: true,
                triggered: false,
            }),
            27656u32 => Ok(GeneratedStruct24 {
                triggered: true,
                orientation: FrontAndTop::DownWest,
                crafting: true,
            }),
            27657u32 => Ok(GeneratedStruct24 {
                crafting: true,
                orientation: FrontAndTop::DownWest,
                triggered: false,
            }),
            27658u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::UpEast,
                crafting: true,
                triggered: true,
            }),
            27659u32 => Ok(GeneratedStruct24 {
                triggered: false,
                orientation: FrontAndTop::UpEast,
                crafting: true,
            }),
            27660u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::UpNorth,
                triggered: true,
                crafting: true,
            }),
            27661u32 => Ok(GeneratedStruct24 {
                crafting: true,
                orientation: FrontAndTop::UpNorth,
                triggered: false,
            }),
            27662u32 => Ok(GeneratedStruct24 {
                triggered: true,
                crafting: true,
                orientation: FrontAndTop::UpSouth,
            }),
            27663u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::UpSouth,
                crafting: true,
                triggered: false,
            }),
            27664u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::UpWest,
                triggered: true,
                crafting: true,
            }),
            27665u32 => Ok(GeneratedStruct24 {
                triggered: false,
                crafting: true,
                orientation: FrontAndTop::UpWest,
            }),
            27666u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::WestUp,
                crafting: true,
                triggered: true,
            }),
            27667u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::WestUp,
                triggered: false,
                crafting: true,
            }),
            27668u32 => Ok(GeneratedStruct24 {
                triggered: true,
                crafting: true,
                orientation: FrontAndTop::EastUp,
            }),
            27669u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::EastUp,
                triggered: false,
                crafting: true,
            }),
            27670u32 => Ok(GeneratedStruct24 {
                crafting: true,
                triggered: true,
                orientation: FrontAndTop::NorthUp,
            }),
            27671u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::NorthUp,
                triggered: false,
                crafting: true,
            }),
            27672u32 => Ok(GeneratedStruct24 {
                triggered: true,
                orientation: FrontAndTop::SouthUp,
                crafting: true,
            }),
            27673u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::SouthUp,
                crafting: true,
                triggered: false,
            }),
            27674u32 => Ok(GeneratedStruct24 {
                triggered: true,
                crafting: false,
                orientation: FrontAndTop::DownEast,
            }),
            27675u32 => Ok(GeneratedStruct24 {
                triggered: false,
                orientation: FrontAndTop::DownEast,
                crafting: false,
            }),
            27676u32 => Ok(GeneratedStruct24 {
                crafting: false,
                orientation: FrontAndTop::DownNorth,
                triggered: true,
            }),
            27677u32 => Ok(GeneratedStruct24 {
                crafting: false,
                orientation: FrontAndTop::DownNorth,
                triggered: false,
            }),
            27678u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::DownSouth,
                crafting: false,
                triggered: true,
            }),
            27679u32 => Ok(GeneratedStruct24 {
                crafting: false,
                orientation: FrontAndTop::DownSouth,
                triggered: false,
            }),
            27680u32 => Ok(GeneratedStruct24 {
                crafting: false,
                orientation: FrontAndTop::DownWest,
                triggered: true,
            }),
            27681u32 => Ok(GeneratedStruct24 {
                triggered: false,
                crafting: false,
                orientation: FrontAndTop::DownWest,
            }),
            27682u32 => Ok(GeneratedStruct24 {
                crafting: false,
                orientation: FrontAndTop::UpEast,
                triggered: true,
            }),
            27683u32 => Ok(GeneratedStruct24 {
                triggered: false,
                crafting: false,
                orientation: FrontAndTop::UpEast,
            }),
            27684u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::UpNorth,
                crafting: false,
                triggered: true,
            }),
            27685u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::UpNorth,
                triggered: false,
                crafting: false,
            }),
            27686u32 => Ok(GeneratedStruct24 {
                crafting: false,
                orientation: FrontAndTop::UpSouth,
                triggered: true,
            }),
            27687u32 => Ok(GeneratedStruct24 {
                crafting: false,
                orientation: FrontAndTop::UpSouth,
                triggered: false,
            }),
            27688u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::UpWest,
                triggered: true,
                crafting: false,
            }),
            27689u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::UpWest,
                crafting: false,
                triggered: false,
            }),
            27690u32 => Ok(GeneratedStruct24 {
                crafting: false,
                triggered: true,
                orientation: FrontAndTop::WestUp,
            }),
            27691u32 => Ok(GeneratedStruct24 {
                triggered: false,
                crafting: false,
                orientation: FrontAndTop::WestUp,
            }),
            27692u32 => Ok(GeneratedStruct24 {
                crafting: false,
                orientation: FrontAndTop::EastUp,
                triggered: true,
            }),
            27693u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::EastUp,
                triggered: false,
                crafting: false,
            }),
            27694u32 => Ok(GeneratedStruct24 {
                orientation: FrontAndTop::NorthUp,
                crafting: false,
                triggered: true,
            }),
            27695u32 => Ok(GeneratedStruct24 {
                crafting: false,
                orientation: FrontAndTop::NorthUp,
                triggered: false,
            }),
            27696u32 => Ok(GeneratedStruct24 {
                crafting: false,
                orientation: FrontAndTop::SouthUp,
                triggered: true,
            }),
            27697u32 => Ok(GeneratedStruct24 {
                crafting: false,
                orientation: FrontAndTop::SouthUp,
                triggered: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for GeneratedStruct24 {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            GeneratedStruct24 {
                crafting: true,
                triggered: true,
                orientation: FrontAndTop::DownEast,
            } => Ok(27650u32),
            GeneratedStruct24 {
                triggered: false,
                crafting: true,
                orientation: FrontAndTop::DownEast,
            } => Ok(27651u32),
            GeneratedStruct24 {
                triggered: true,
                crafting: true,
                orientation: FrontAndTop::DownNorth,
            } => Ok(27652u32),
            GeneratedStruct24 {
                crafting: true,
                orientation: FrontAndTop::DownNorth,
                triggered: false,
            } => Ok(27653u32),
            GeneratedStruct24 {
                triggered: true,
                crafting: true,
                orientation: FrontAndTop::DownSouth,
            } => Ok(27654u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::DownSouth,
                crafting: true,
                triggered: false,
            } => Ok(27655u32),
            GeneratedStruct24 {
                triggered: true,
                orientation: FrontAndTop::DownWest,
                crafting: true,
            } => Ok(27656u32),
            GeneratedStruct24 {
                crafting: true,
                orientation: FrontAndTop::DownWest,
                triggered: false,
            } => Ok(27657u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::UpEast,
                crafting: true,
                triggered: true,
            } => Ok(27658u32),
            GeneratedStruct24 {
                triggered: false,
                orientation: FrontAndTop::UpEast,
                crafting: true,
            } => Ok(27659u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::UpNorth,
                triggered: true,
                crafting: true,
            } => Ok(27660u32),
            GeneratedStruct24 {
                crafting: true,
                orientation: FrontAndTop::UpNorth,
                triggered: false,
            } => Ok(27661u32),
            GeneratedStruct24 {
                triggered: true,
                crafting: true,
                orientation: FrontAndTop::UpSouth,
            } => Ok(27662u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::UpSouth,
                crafting: true,
                triggered: false,
            } => Ok(27663u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::UpWest,
                triggered: true,
                crafting: true,
            } => Ok(27664u32),
            GeneratedStruct24 {
                triggered: false,
                crafting: true,
                orientation: FrontAndTop::UpWest,
            } => Ok(27665u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::WestUp,
                crafting: true,
                triggered: true,
            } => Ok(27666u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::WestUp,
                triggered: false,
                crafting: true,
            } => Ok(27667u32),
            GeneratedStruct24 {
                triggered: true,
                crafting: true,
                orientation: FrontAndTop::EastUp,
            } => Ok(27668u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::EastUp,
                triggered: false,
                crafting: true,
            } => Ok(27669u32),
            GeneratedStruct24 {
                crafting: true,
                triggered: true,
                orientation: FrontAndTop::NorthUp,
            } => Ok(27670u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::NorthUp,
                triggered: false,
                crafting: true,
            } => Ok(27671u32),
            GeneratedStruct24 {
                triggered: true,
                orientation: FrontAndTop::SouthUp,
                crafting: true,
            } => Ok(27672u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::SouthUp,
                crafting: true,
                triggered: false,
            } => Ok(27673u32),
            GeneratedStruct24 {
                triggered: true,
                crafting: false,
                orientation: FrontAndTop::DownEast,
            } => Ok(27674u32),
            GeneratedStruct24 {
                triggered: false,
                orientation: FrontAndTop::DownEast,
                crafting: false,
            } => Ok(27675u32),
            GeneratedStruct24 {
                crafting: false,
                orientation: FrontAndTop::DownNorth,
                triggered: true,
            } => Ok(27676u32),
            GeneratedStruct24 {
                crafting: false,
                orientation: FrontAndTop::DownNorth,
                triggered: false,
            } => Ok(27677u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::DownSouth,
                crafting: false,
                triggered: true,
            } => Ok(27678u32),
            GeneratedStruct24 {
                crafting: false,
                orientation: FrontAndTop::DownSouth,
                triggered: false,
            } => Ok(27679u32),
            GeneratedStruct24 {
                crafting: false,
                orientation: FrontAndTop::DownWest,
                triggered: true,
            } => Ok(27680u32),
            GeneratedStruct24 {
                triggered: false,
                crafting: false,
                orientation: FrontAndTop::DownWest,
            } => Ok(27681u32),
            GeneratedStruct24 {
                crafting: false,
                orientation: FrontAndTop::UpEast,
                triggered: true,
            } => Ok(27682u32),
            GeneratedStruct24 {
                triggered: false,
                crafting: false,
                orientation: FrontAndTop::UpEast,
            } => Ok(27683u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::UpNorth,
                crafting: false,
                triggered: true,
            } => Ok(27684u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::UpNorth,
                triggered: false,
                crafting: false,
            } => Ok(27685u32),
            GeneratedStruct24 {
                crafting: false,
                orientation: FrontAndTop::UpSouth,
                triggered: true,
            } => Ok(27686u32),
            GeneratedStruct24 {
                crafting: false,
                orientation: FrontAndTop::UpSouth,
                triggered: false,
            } => Ok(27687u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::UpWest,
                triggered: true,
                crafting: false,
            } => Ok(27688u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::UpWest,
                crafting: false,
                triggered: false,
            } => Ok(27689u32),
            GeneratedStruct24 {
                crafting: false,
                triggered: true,
                orientation: FrontAndTop::WestUp,
            } => Ok(27690u32),
            GeneratedStruct24 {
                triggered: false,
                crafting: false,
                orientation: FrontAndTop::WestUp,
            } => Ok(27691u32),
            GeneratedStruct24 {
                crafting: false,
                orientation: FrontAndTop::EastUp,
                triggered: true,
            } => Ok(27692u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::EastUp,
                triggered: false,
                crafting: false,
            } => Ok(27693u32),
            GeneratedStruct24 {
                orientation: FrontAndTop::NorthUp,
                crafting: false,
                triggered: true,
            } => Ok(27694u32),
            GeneratedStruct24 {
                crafting: false,
                orientation: FrontAndTop::NorthUp,
                triggered: false,
            } => Ok(27695u32),
            GeneratedStruct24 {
                crafting: false,
                orientation: FrontAndTop::SouthUp,
                triggered: true,
            } => Ok(27696u32),
            GeneratedStruct24 {
                crafting: false,
                orientation: FrontAndTop::SouthUp,
                triggered: false,
            } => Ok(27697u32),
            _ => Err(()),
        }
    }
}

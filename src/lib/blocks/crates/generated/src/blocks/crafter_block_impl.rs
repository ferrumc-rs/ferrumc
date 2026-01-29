use crate::CrafterBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for CrafterBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            27650u32 => Ok(CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::DownEast,
                triggered: true,
            }),
            27651u32 => Ok(CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::DownEast,
                triggered: false,
            }),
            27652u32 => Ok(CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::DownNorth,
                triggered: true,
            }),
            27653u32 => Ok(CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::DownNorth,
                triggered: false,
            }),
            27654u32 => Ok(CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::DownSouth,
                triggered: true,
            }),
            27655u32 => Ok(CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::DownSouth,
                triggered: false,
            }),
            27656u32 => Ok(CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::DownWest,
                triggered: true,
            }),
            27657u32 => Ok(CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::DownWest,
                triggered: false,
            }),
            27658u32 => Ok(CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::UpEast,
                triggered: true,
            }),
            27659u32 => Ok(CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::UpEast,
                triggered: false,
            }),
            27660u32 => Ok(CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::UpNorth,
                triggered: true,
            }),
            27661u32 => Ok(CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::UpNorth,
                triggered: false,
            }),
            27662u32 => Ok(CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::UpSouth,
                triggered: true,
            }),
            27663u32 => Ok(CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::UpSouth,
                triggered: false,
            }),
            27664u32 => Ok(CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::UpWest,
                triggered: true,
            }),
            27665u32 => Ok(CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::UpWest,
                triggered: false,
            }),
            27666u32 => Ok(CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::WestUp,
                triggered: true,
            }),
            27667u32 => Ok(CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::WestUp,
                triggered: false,
            }),
            27668u32 => Ok(CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::EastUp,
                triggered: true,
            }),
            27669u32 => Ok(CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::EastUp,
                triggered: false,
            }),
            27670u32 => Ok(CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::NorthUp,
                triggered: true,
            }),
            27671u32 => Ok(CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::NorthUp,
                triggered: false,
            }),
            27672u32 => Ok(CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::SouthUp,
                triggered: true,
            }),
            27673u32 => Ok(CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::SouthUp,
                triggered: false,
            }),
            27674u32 => Ok(CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::DownEast,
                triggered: true,
            }),
            27675u32 => Ok(CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::DownEast,
                triggered: false,
            }),
            27676u32 => Ok(CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::DownNorth,
                triggered: true,
            }),
            27677u32 => Ok(CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::DownNorth,
                triggered: false,
            }),
            27678u32 => Ok(CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::DownSouth,
                triggered: true,
            }),
            27679u32 => Ok(CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::DownSouth,
                triggered: false,
            }),
            27680u32 => Ok(CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::DownWest,
                triggered: true,
            }),
            27681u32 => Ok(CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::DownWest,
                triggered: false,
            }),
            27682u32 => Ok(CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::UpEast,
                triggered: true,
            }),
            27683u32 => Ok(CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::UpEast,
                triggered: false,
            }),
            27684u32 => Ok(CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::UpNorth,
                triggered: true,
            }),
            27685u32 => Ok(CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::UpNorth,
                triggered: false,
            }),
            27686u32 => Ok(CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::UpSouth,
                triggered: true,
            }),
            27687u32 => Ok(CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::UpSouth,
                triggered: false,
            }),
            27688u32 => Ok(CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::UpWest,
                triggered: true,
            }),
            27689u32 => Ok(CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::UpWest,
                triggered: false,
            }),
            27690u32 => Ok(CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::WestUp,
                triggered: true,
            }),
            27691u32 => Ok(CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::WestUp,
                triggered: false,
            }),
            27692u32 => Ok(CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::EastUp,
                triggered: true,
            }),
            27693u32 => Ok(CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::EastUp,
                triggered: false,
            }),
            27694u32 => Ok(CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::NorthUp,
                triggered: true,
            }),
            27695u32 => Ok(CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::NorthUp,
                triggered: false,
            }),
            27696u32 => Ok(CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::SouthUp,
                triggered: true,
            }),
            27697u32 => Ok(CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::SouthUp,
                triggered: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for CrafterBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::DownEast,
                triggered: true,
            } => Ok(27650u32),
            CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::DownEast,
                triggered: false,
            } => Ok(27651u32),
            CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::DownNorth,
                triggered: true,
            } => Ok(27652u32),
            CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::DownNorth,
                triggered: false,
            } => Ok(27653u32),
            CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::DownSouth,
                triggered: true,
            } => Ok(27654u32),
            CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::DownSouth,
                triggered: false,
            } => Ok(27655u32),
            CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::DownWest,
                triggered: true,
            } => Ok(27656u32),
            CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::DownWest,
                triggered: false,
            } => Ok(27657u32),
            CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::UpEast,
                triggered: true,
            } => Ok(27658u32),
            CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::UpEast,
                triggered: false,
            } => Ok(27659u32),
            CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::UpNorth,
                triggered: true,
            } => Ok(27660u32),
            CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::UpNorth,
                triggered: false,
            } => Ok(27661u32),
            CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::UpSouth,
                triggered: true,
            } => Ok(27662u32),
            CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::UpSouth,
                triggered: false,
            } => Ok(27663u32),
            CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::UpWest,
                triggered: true,
            } => Ok(27664u32),
            CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::UpWest,
                triggered: false,
            } => Ok(27665u32),
            CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::WestUp,
                triggered: true,
            } => Ok(27666u32),
            CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::WestUp,
                triggered: false,
            } => Ok(27667u32),
            CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::EastUp,
                triggered: true,
            } => Ok(27668u32),
            CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::EastUp,
                triggered: false,
            } => Ok(27669u32),
            CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::NorthUp,
                triggered: true,
            } => Ok(27670u32),
            CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::NorthUp,
                triggered: false,
            } => Ok(27671u32),
            CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::SouthUp,
                triggered: true,
            } => Ok(27672u32),
            CrafterBlock {
                crafting: true,
                orientation: FrontAndTop::SouthUp,
                triggered: false,
            } => Ok(27673u32),
            CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::DownEast,
                triggered: true,
            } => Ok(27674u32),
            CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::DownEast,
                triggered: false,
            } => Ok(27675u32),
            CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::DownNorth,
                triggered: true,
            } => Ok(27676u32),
            CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::DownNorth,
                triggered: false,
            } => Ok(27677u32),
            CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::DownSouth,
                triggered: true,
            } => Ok(27678u32),
            CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::DownSouth,
                triggered: false,
            } => Ok(27679u32),
            CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::DownWest,
                triggered: true,
            } => Ok(27680u32),
            CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::DownWest,
                triggered: false,
            } => Ok(27681u32),
            CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::UpEast,
                triggered: true,
            } => Ok(27682u32),
            CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::UpEast,
                triggered: false,
            } => Ok(27683u32),
            CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::UpNorth,
                triggered: true,
            } => Ok(27684u32),
            CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::UpNorth,
                triggered: false,
            } => Ok(27685u32),
            CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::UpSouth,
                triggered: true,
            } => Ok(27686u32),
            CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::UpSouth,
                triggered: false,
            } => Ok(27687u32),
            CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::UpWest,
                triggered: true,
            } => Ok(27688u32),
            CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::UpWest,
                triggered: false,
            } => Ok(27689u32),
            CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::WestUp,
                triggered: true,
            } => Ok(27690u32),
            CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::WestUp,
                triggered: false,
            } => Ok(27691u32),
            CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::EastUp,
                triggered: true,
            } => Ok(27692u32),
            CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::EastUp,
                triggered: false,
            } => Ok(27693u32),
            CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::NorthUp,
                triggered: true,
            } => Ok(27694u32),
            CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::NorthUp,
                triggered: false,
            } => Ok(27695u32),
            CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::SouthUp,
                triggered: true,
            } => Ok(27696u32),
            CrafterBlock {
                crafting: false,
                orientation: FrontAndTop::SouthUp,
                triggered: false,
            } => Ok(27697u32),
            _ => Err(()),
        }
    }
}

use crate::SculkSensorBlock;
#[allow(unused_imports)]
use ferrumc_block_properties::*;
impl TryFrom<u32> for SculkSensorBlock {
    type Error = ();
    fn try_from(data: u32) -> Result<Self, Self::Error> {
        match data {
            23347u32 => Ok(SculkSensorBlock {
                power: 0i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            }),
            23348u32 => Ok(SculkSensorBlock {
                power: 0i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
            }),
            23349u32 => Ok(SculkSensorBlock {
                power: 0i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
            }),
            23350u32 => Ok(SculkSensorBlock {
                power: 0i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
            }),
            23351u32 => Ok(SculkSensorBlock {
                power: 0i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            }),
            23352u32 => Ok(SculkSensorBlock {
                power: 0i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
            }),
            23353u32 => Ok(SculkSensorBlock {
                power: 1i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            }),
            23354u32 => Ok(SculkSensorBlock {
                power: 1i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
            }),
            23355u32 => Ok(SculkSensorBlock {
                power: 1i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
            }),
            23356u32 => Ok(SculkSensorBlock {
                power: 1i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
            }),
            23357u32 => Ok(SculkSensorBlock {
                power: 1i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            }),
            23358u32 => Ok(SculkSensorBlock {
                power: 1i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
            }),
            23359u32 => Ok(SculkSensorBlock {
                power: 2i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            }),
            23360u32 => Ok(SculkSensorBlock {
                power: 2i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
            }),
            23361u32 => Ok(SculkSensorBlock {
                power: 2i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
            }),
            23362u32 => Ok(SculkSensorBlock {
                power: 2i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
            }),
            23363u32 => Ok(SculkSensorBlock {
                power: 2i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            }),
            23364u32 => Ok(SculkSensorBlock {
                power: 2i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
            }),
            23365u32 => Ok(SculkSensorBlock {
                power: 3i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            }),
            23366u32 => Ok(SculkSensorBlock {
                power: 3i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
            }),
            23367u32 => Ok(SculkSensorBlock {
                power: 3i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
            }),
            23368u32 => Ok(SculkSensorBlock {
                power: 3i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
            }),
            23369u32 => Ok(SculkSensorBlock {
                power: 3i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            }),
            23370u32 => Ok(SculkSensorBlock {
                power: 3i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
            }),
            23371u32 => Ok(SculkSensorBlock {
                power: 4i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            }),
            23372u32 => Ok(SculkSensorBlock {
                power: 4i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
            }),
            23373u32 => Ok(SculkSensorBlock {
                power: 4i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
            }),
            23374u32 => Ok(SculkSensorBlock {
                power: 4i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
            }),
            23375u32 => Ok(SculkSensorBlock {
                power: 4i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            }),
            23376u32 => Ok(SculkSensorBlock {
                power: 4i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
            }),
            23377u32 => Ok(SculkSensorBlock {
                power: 5i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            }),
            23378u32 => Ok(SculkSensorBlock {
                power: 5i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
            }),
            23379u32 => Ok(SculkSensorBlock {
                power: 5i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
            }),
            23380u32 => Ok(SculkSensorBlock {
                power: 5i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
            }),
            23381u32 => Ok(SculkSensorBlock {
                power: 5i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            }),
            23382u32 => Ok(SculkSensorBlock {
                power: 5i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
            }),
            23383u32 => Ok(SculkSensorBlock {
                power: 6i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            }),
            23384u32 => Ok(SculkSensorBlock {
                power: 6i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
            }),
            23385u32 => Ok(SculkSensorBlock {
                power: 6i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
            }),
            23386u32 => Ok(SculkSensorBlock {
                power: 6i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
            }),
            23387u32 => Ok(SculkSensorBlock {
                power: 6i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            }),
            23388u32 => Ok(SculkSensorBlock {
                power: 6i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
            }),
            23389u32 => Ok(SculkSensorBlock {
                power: 7i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            }),
            23390u32 => Ok(SculkSensorBlock {
                power: 7i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
            }),
            23391u32 => Ok(SculkSensorBlock {
                power: 7i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
            }),
            23392u32 => Ok(SculkSensorBlock {
                power: 7i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
            }),
            23393u32 => Ok(SculkSensorBlock {
                power: 7i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            }),
            23394u32 => Ok(SculkSensorBlock {
                power: 7i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
            }),
            23395u32 => Ok(SculkSensorBlock {
                power: 8i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            }),
            23396u32 => Ok(SculkSensorBlock {
                power: 8i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
            }),
            23397u32 => Ok(SculkSensorBlock {
                power: 8i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
            }),
            23398u32 => Ok(SculkSensorBlock {
                power: 8i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
            }),
            23399u32 => Ok(SculkSensorBlock {
                power: 8i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            }),
            23400u32 => Ok(SculkSensorBlock {
                power: 8i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
            }),
            23401u32 => Ok(SculkSensorBlock {
                power: 9i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            }),
            23402u32 => Ok(SculkSensorBlock {
                power: 9i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
            }),
            23403u32 => Ok(SculkSensorBlock {
                power: 9i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
            }),
            23404u32 => Ok(SculkSensorBlock {
                power: 9i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
            }),
            23405u32 => Ok(SculkSensorBlock {
                power: 9i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            }),
            23406u32 => Ok(SculkSensorBlock {
                power: 9i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
            }),
            23407u32 => Ok(SculkSensorBlock {
                power: 10i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            }),
            23408u32 => Ok(SculkSensorBlock {
                power: 10i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
            }),
            23409u32 => Ok(SculkSensorBlock {
                power: 10i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
            }),
            23410u32 => Ok(SculkSensorBlock {
                power: 10i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
            }),
            23411u32 => Ok(SculkSensorBlock {
                power: 10i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            }),
            23412u32 => Ok(SculkSensorBlock {
                power: 10i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
            }),
            23413u32 => Ok(SculkSensorBlock {
                power: 11i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            }),
            23414u32 => Ok(SculkSensorBlock {
                power: 11i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
            }),
            23415u32 => Ok(SculkSensorBlock {
                power: 11i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
            }),
            23416u32 => Ok(SculkSensorBlock {
                power: 11i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
            }),
            23417u32 => Ok(SculkSensorBlock {
                power: 11i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            }),
            23418u32 => Ok(SculkSensorBlock {
                power: 11i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
            }),
            23419u32 => Ok(SculkSensorBlock {
                power: 12i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            }),
            23420u32 => Ok(SculkSensorBlock {
                power: 12i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
            }),
            23421u32 => Ok(SculkSensorBlock {
                power: 12i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
            }),
            23422u32 => Ok(SculkSensorBlock {
                power: 12i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
            }),
            23423u32 => Ok(SculkSensorBlock {
                power: 12i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            }),
            23424u32 => Ok(SculkSensorBlock {
                power: 12i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
            }),
            23425u32 => Ok(SculkSensorBlock {
                power: 13i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            }),
            23426u32 => Ok(SculkSensorBlock {
                power: 13i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
            }),
            23427u32 => Ok(SculkSensorBlock {
                power: 13i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
            }),
            23428u32 => Ok(SculkSensorBlock {
                power: 13i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
            }),
            23429u32 => Ok(SculkSensorBlock {
                power: 13i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            }),
            23430u32 => Ok(SculkSensorBlock {
                power: 13i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
            }),
            23431u32 => Ok(SculkSensorBlock {
                power: 14i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            }),
            23432u32 => Ok(SculkSensorBlock {
                power: 14i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
            }),
            23433u32 => Ok(SculkSensorBlock {
                power: 14i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
            }),
            23434u32 => Ok(SculkSensorBlock {
                power: 14i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
            }),
            23435u32 => Ok(SculkSensorBlock {
                power: 14i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            }),
            23436u32 => Ok(SculkSensorBlock {
                power: 14i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
            }),
            23437u32 => Ok(SculkSensorBlock {
                power: 15i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            }),
            23438u32 => Ok(SculkSensorBlock {
                power: 15i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
            }),
            23439u32 => Ok(SculkSensorBlock {
                power: 15i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
            }),
            23440u32 => Ok(SculkSensorBlock {
                power: 15i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
            }),
            23441u32 => Ok(SculkSensorBlock {
                power: 15i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            }),
            23442u32 => Ok(SculkSensorBlock {
                power: 15i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<u32> for SculkSensorBlock {
    type Error = ();
    fn try_into(self) -> Result<u32, Self::Error> {
        #[allow(unreachable_patterns)]
        match self {
            SculkSensorBlock {
                power: 0i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            } => Ok(23347u32),
            SculkSensorBlock {
                power: 0i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
            } => Ok(23348u32),
            SculkSensorBlock {
                power: 0i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
            } => Ok(23349u32),
            SculkSensorBlock {
                power: 0i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
            } => Ok(23350u32),
            SculkSensorBlock {
                power: 0i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            } => Ok(23351u32),
            SculkSensorBlock {
                power: 0i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
            } => Ok(23352u32),
            SculkSensorBlock {
                power: 1i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            } => Ok(23353u32),
            SculkSensorBlock {
                power: 1i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
            } => Ok(23354u32),
            SculkSensorBlock {
                power: 1i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
            } => Ok(23355u32),
            SculkSensorBlock {
                power: 1i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
            } => Ok(23356u32),
            SculkSensorBlock {
                power: 1i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            } => Ok(23357u32),
            SculkSensorBlock {
                power: 1i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
            } => Ok(23358u32),
            SculkSensorBlock {
                power: 2i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            } => Ok(23359u32),
            SculkSensorBlock {
                power: 2i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
            } => Ok(23360u32),
            SculkSensorBlock {
                power: 2i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
            } => Ok(23361u32),
            SculkSensorBlock {
                power: 2i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
            } => Ok(23362u32),
            SculkSensorBlock {
                power: 2i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            } => Ok(23363u32),
            SculkSensorBlock {
                power: 2i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
            } => Ok(23364u32),
            SculkSensorBlock {
                power: 3i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            } => Ok(23365u32),
            SculkSensorBlock {
                power: 3i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
            } => Ok(23366u32),
            SculkSensorBlock {
                power: 3i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
            } => Ok(23367u32),
            SculkSensorBlock {
                power: 3i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
            } => Ok(23368u32),
            SculkSensorBlock {
                power: 3i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            } => Ok(23369u32),
            SculkSensorBlock {
                power: 3i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
            } => Ok(23370u32),
            SculkSensorBlock {
                power: 4i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            } => Ok(23371u32),
            SculkSensorBlock {
                power: 4i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
            } => Ok(23372u32),
            SculkSensorBlock {
                power: 4i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
            } => Ok(23373u32),
            SculkSensorBlock {
                power: 4i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
            } => Ok(23374u32),
            SculkSensorBlock {
                power: 4i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            } => Ok(23375u32),
            SculkSensorBlock {
                power: 4i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
            } => Ok(23376u32),
            SculkSensorBlock {
                power: 5i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            } => Ok(23377u32),
            SculkSensorBlock {
                power: 5i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
            } => Ok(23378u32),
            SculkSensorBlock {
                power: 5i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
            } => Ok(23379u32),
            SculkSensorBlock {
                power: 5i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
            } => Ok(23380u32),
            SculkSensorBlock {
                power: 5i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            } => Ok(23381u32),
            SculkSensorBlock {
                power: 5i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
            } => Ok(23382u32),
            SculkSensorBlock {
                power: 6i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            } => Ok(23383u32),
            SculkSensorBlock {
                power: 6i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
            } => Ok(23384u32),
            SculkSensorBlock {
                power: 6i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
            } => Ok(23385u32),
            SculkSensorBlock {
                power: 6i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
            } => Ok(23386u32),
            SculkSensorBlock {
                power: 6i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            } => Ok(23387u32),
            SculkSensorBlock {
                power: 6i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
            } => Ok(23388u32),
            SculkSensorBlock {
                power: 7i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            } => Ok(23389u32),
            SculkSensorBlock {
                power: 7i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
            } => Ok(23390u32),
            SculkSensorBlock {
                power: 7i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
            } => Ok(23391u32),
            SculkSensorBlock {
                power: 7i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
            } => Ok(23392u32),
            SculkSensorBlock {
                power: 7i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            } => Ok(23393u32),
            SculkSensorBlock {
                power: 7i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
            } => Ok(23394u32),
            SculkSensorBlock {
                power: 8i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            } => Ok(23395u32),
            SculkSensorBlock {
                power: 8i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
            } => Ok(23396u32),
            SculkSensorBlock {
                power: 8i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
            } => Ok(23397u32),
            SculkSensorBlock {
                power: 8i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
            } => Ok(23398u32),
            SculkSensorBlock {
                power: 8i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            } => Ok(23399u32),
            SculkSensorBlock {
                power: 8i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
            } => Ok(23400u32),
            SculkSensorBlock {
                power: 9i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            } => Ok(23401u32),
            SculkSensorBlock {
                power: 9i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
            } => Ok(23402u32),
            SculkSensorBlock {
                power: 9i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
            } => Ok(23403u32),
            SculkSensorBlock {
                power: 9i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
            } => Ok(23404u32),
            SculkSensorBlock {
                power: 9i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            } => Ok(23405u32),
            SculkSensorBlock {
                power: 9i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
            } => Ok(23406u32),
            SculkSensorBlock {
                power: 10i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            } => Ok(23407u32),
            SculkSensorBlock {
                power: 10i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
            } => Ok(23408u32),
            SculkSensorBlock {
                power: 10i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
            } => Ok(23409u32),
            SculkSensorBlock {
                power: 10i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
            } => Ok(23410u32),
            SculkSensorBlock {
                power: 10i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            } => Ok(23411u32),
            SculkSensorBlock {
                power: 10i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
            } => Ok(23412u32),
            SculkSensorBlock {
                power: 11i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            } => Ok(23413u32),
            SculkSensorBlock {
                power: 11i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
            } => Ok(23414u32),
            SculkSensorBlock {
                power: 11i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
            } => Ok(23415u32),
            SculkSensorBlock {
                power: 11i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
            } => Ok(23416u32),
            SculkSensorBlock {
                power: 11i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            } => Ok(23417u32),
            SculkSensorBlock {
                power: 11i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
            } => Ok(23418u32),
            SculkSensorBlock {
                power: 12i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            } => Ok(23419u32),
            SculkSensorBlock {
                power: 12i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
            } => Ok(23420u32),
            SculkSensorBlock {
                power: 12i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
            } => Ok(23421u32),
            SculkSensorBlock {
                power: 12i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
            } => Ok(23422u32),
            SculkSensorBlock {
                power: 12i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            } => Ok(23423u32),
            SculkSensorBlock {
                power: 12i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
            } => Ok(23424u32),
            SculkSensorBlock {
                power: 13i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            } => Ok(23425u32),
            SculkSensorBlock {
                power: 13i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
            } => Ok(23426u32),
            SculkSensorBlock {
                power: 13i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
            } => Ok(23427u32),
            SculkSensorBlock {
                power: 13i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
            } => Ok(23428u32),
            SculkSensorBlock {
                power: 13i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            } => Ok(23429u32),
            SculkSensorBlock {
                power: 13i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
            } => Ok(23430u32),
            SculkSensorBlock {
                power: 14i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            } => Ok(23431u32),
            SculkSensorBlock {
                power: 14i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
            } => Ok(23432u32),
            SculkSensorBlock {
                power: 14i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
            } => Ok(23433u32),
            SculkSensorBlock {
                power: 14i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
            } => Ok(23434u32),
            SculkSensorBlock {
                power: 14i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            } => Ok(23435u32),
            SculkSensorBlock {
                power: 14i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
            } => Ok(23436u32),
            SculkSensorBlock {
                power: 15i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: true,
            } => Ok(23437u32),
            SculkSensorBlock {
                power: 15i32,
                sculk_sensor_phase: SculkSensorPhase::Inactive,
                waterlogged: false,
            } => Ok(23438u32),
            SculkSensorBlock {
                power: 15i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: true,
            } => Ok(23439u32),
            SculkSensorBlock {
                power: 15i32,
                sculk_sensor_phase: SculkSensorPhase::Active,
                waterlogged: false,
            } => Ok(23440u32),
            SculkSensorBlock {
                power: 15i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: true,
            } => Ok(23441u32),
            SculkSensorBlock {
                power: 15i32,
                sculk_sensor_phase: SculkSensorPhase::Cooldown,
                waterlogged: false,
            } => Ok(23442u32),
            _ => Err(()),
        }
    }
}

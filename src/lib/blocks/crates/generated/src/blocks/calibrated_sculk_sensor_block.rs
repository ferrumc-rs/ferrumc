#[allow(unused_imports)]
use ferrumc_block_properties::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CalibratedSculkSensorBlock {
    pub facing: Direction,
    pub power: i32,
    pub sculk_sensor_phase: SculkSensorPhase,
    pub waterlogged: bool,
}

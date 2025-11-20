use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct HungerData {
    /// 0-20 (half chicken legs of food)
    pub food_level: u8,
    /// saturation level
    pub food_saturation_level: f32,
    /// Exhaustion level (accumulates as player moves/acts)
    pub food_exhaustion_level: f32,
}

impl Default for HungerData {
    fn default() -> Self {
        Self {
            food_level: 20, // Full food
            food_saturation_level: 5.0,
            food_exhaustion_level: 0.0,
        }
    }
}

impl HungerData {
    pub fn new(level: u8, saturation: f32, exhaustion: f32) -> Self {
        Self {
            food_level: level,
            food_saturation_level: saturation,
            food_exhaustion_level: exhaustion,
        }
    }
}

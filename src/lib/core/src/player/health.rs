use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct HealthData {
    pub current: f32,
    pub max: f32,
}

// Standard Minecraft defaults (20 health = 10 hearts)
impl Default for HealthData {
    fn default() -> Self {
        Self {
            current: 20.0,
            max: 20.0,
        }
    }
}

impl HealthData {
    pub fn new(current: f32, max: f32) -> Self {
        Self { current, max }
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }
}

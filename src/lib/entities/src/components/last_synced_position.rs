use bevy_ecs::prelude::Component;
use ferrumc_core::transform::position::Position;

/// Component that tracks the last position synchronized to clients
#[derive(Component, Debug, Clone)]
pub struct LastSyncedPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl LastSyncedPosition {
    pub fn from_position(pos: &Position) -> Self {
        Self {
            x: pos.x,
            y: pos.y,
            z: pos.z,
        }
    }

    /// Calculate delta in Minecraft protocol units (1/4096 of a block)
    pub fn delta_to(&self, new_pos: &Position) -> (i16, i16, i16) {
        const SCALE: f64 = 4096.0;
        let dx = ((new_pos.x - self.x) * SCALE) as i16;
        let dy = ((new_pos.y - self.y) * SCALE) as i16;
        let dz = ((new_pos.z - self.z) * SCALE) as i16;
        (dx, dy, dz)
    }

    pub fn has_moved(&self, new_pos: &Position) -> bool {
        (self.x - new_pos.x).abs() > 0.001
            || (self.y - new_pos.y).abs() > 0.001
            || (self.z - new_pos.z).abs() > 0.001
    }
}

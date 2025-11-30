use bevy_ecs::prelude::Component;
use bevy_math::DVec3;
use ferrumc_core::transform::position::Position;

/// Component that tracks the last position synchronized to clients
#[derive(Component, Debug, Clone, Copy)]
pub struct LastSyncedPosition(pub DVec3);

impl LastSyncedPosition {
    pub fn from_position(pos: &Position) -> Self {
        Self(pos.coords)
    }

    /// Calculate delta in Minecraft protocol units (1/4096 of a block)
    pub fn delta_to(&self, new_pos: &Position) -> (i16, i16, i16) {
        const SCALE: f64 = 4096.0;
        let delta = new_pos.coords - self.0;
        (
            (delta.x * SCALE) as i16,
            (delta.y * SCALE) as i16,
            (delta.z * SCALE) as i16,
        )
    }

    pub fn has_moved(&self, new_pos: &Position) -> bool {
        let delta = self.delta_to(new_pos);
        delta.0 != 0 || delta.1 != 0 || delta.2 != 0
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_math::DVec3;

    #[test]
    fn test_from_position() {
        let pos = Position::new(10.5, 64.0, -20.3);
        let last_synced = LastSyncedPosition::from_position(&pos);

        assert_eq!(last_synced.0, DVec3::new(10.5, 64.0, -20.3));
    }

    #[test]
    fn test_delta_to_no_movement() {
        let pos = Position::new(10.0, 64.0, 20.0);
        let last_synced = LastSyncedPosition::from_position(&pos);

        let delta = last_synced.delta_to(&pos);
        assert_eq!(delta, (0, 0, 0));
    }

    #[test]
    fn test_delta_to_with_movement() {
        let old_pos = Position::new(10.0, 64.0, 20.0);
        let new_pos = Position::new(10.5, 64.2, 20.1);
        let last_synced = LastSyncedPosition::from_position(&old_pos);

        let delta = last_synced.delta_to(&new_pos);
        // Delta in protocol units (1/4096 of a block)
        // 0.5 * 4096 = 2048
        // 0.2 * 4096 = 819.2 -> 819
        // 0.1 * 4096 = 409.6 -> 409
        assert_eq!(delta, (2048, 819, 409));
    }

    #[test]
    fn test_has_moved_false() {
        let pos = Position::new(10.0, 64.0, 20.0);
        let last_synced = LastSyncedPosition::from_position(&pos);

        assert!(!last_synced.has_moved(&pos));
    }

    #[test]
    fn test_has_moved_true() {
        let old_pos = Position::new(10.0, 64.0, 20.0);
        let new_pos = Position::new(10.001, 64.0, 20.0);
        let last_synced = LastSyncedPosition::from_position(&old_pos);

        assert!(last_synced.has_moved(&new_pos));
    }

    #[test]
    fn test_has_moved_small_delta_below_threshold() {
        let old_pos = Position::new(10.0, 64.0, 20.0);
        // Movement smaller than 1/4096 should round to 0
        let new_pos = Position::new(10.0001, 64.0, 20.0);
        let last_synced = LastSyncedPosition::from_position(&old_pos);

        assert!(!last_synced.has_moved(&new_pos));
    }
}

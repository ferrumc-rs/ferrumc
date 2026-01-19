use bevy_ecs::prelude::{Entity, Message};
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;

#[derive(Message, Debug)]
pub struct Movement {
    pub entity: Entity,
    /// Delta position in protocol units (1/4096 of a block), pre-calculated by packet handlers
    pub delta_position: Option<(i16, i16, i16)>,
    pub rotation: Option<Rotation>,
    pub on_ground: bool,
}

impl Movement {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            delta_position: None,
            rotation: None,
            on_ground: true,
        }
    }

    /// Set delta position in protocol units (multiply block coords by 4096)
    pub fn delta_position(mut self, delta: (i16, i16, i16)) -> Self {
        self.delta_position = Some(delta);
        self
    }

    /// Helper to calculate delta from old and new positions.
    /// Uses saturating arithmetic to prevent overflow for large movements (>8 blocks).
    /// The broadcast handler will detect saturated values and use teleport packets instead.
    pub fn position_delta_from(mut self, old_pos: &Position, new_pos: &Position) -> Self {
        // Calculate delta with clamping to prevent i16 overflow.
        // If player moves more than ~8 blocks in one tick, the delta would overflow.
        // Clamping to i16 range ensures we get max values which trigger teleport logic.
        let calc_delta = |old: f64, new: f64| -> i16 {
            let delta = (new * 4096.0) - (old * 4096.0);
            delta.clamp(i16::MIN as f64, i16::MAX as f64) as i16
        };
        self.delta_position = Some((
            calc_delta(old_pos.x, new_pos.x),
            calc_delta(old_pos.y, new_pos.y),
            calc_delta(old_pos.z, new_pos.z),
        ));
        self
    }

    pub fn rotation(mut self, rotation: Rotation) -> Self {
        self.rotation = Some(rotation);
        self
    }

    pub fn on_ground(mut self, on_ground: bool) -> Self {
        self.on_ground = on_ground;
        self
    }
}

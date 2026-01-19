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

    /// Helper to calculate delta from old and new positions
    pub fn position_delta_from(mut self, old_pos: &Position, new_pos: &Position) -> Self {
        self.delta_position = Some((
            ((new_pos.x * 4096.0) - (old_pos.x * 4096.0)) as i16,
            ((new_pos.y * 4096.0) - (old_pos.y * 4096.0)) as i16,
            ((new_pos.z * 4096.0) - (old_pos.z * 4096.0)) as i16,
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

use bevy_ecs::prelude::{Entity, Message};
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;

#[derive(Message, Debug)]
pub struct Movement {
    pub entity: Entity,
    pub position: Option<Position>,
    pub rotation: Option<Rotation>,
    pub on_ground: Option<bool>,
}

impl Movement {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            position: None,
            rotation: None,
            on_ground: None,
        }
    }
    pub fn position(mut self, position: Position) -> Self {
        self.position = Some(position);
        self
    }

    pub fn rotation(mut self, rotation: Rotation) -> Self {
        self.rotation = Some(rotation);
        self
    }

    pub fn on_ground(mut self, on_ground: bool) -> Self {
        self.on_ground = Some(on_ground);
        self
    }
}

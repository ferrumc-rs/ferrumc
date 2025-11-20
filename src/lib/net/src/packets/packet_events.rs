use bevy_ecs::prelude::{Entity, Event};
use ferrumc_components::player::transform::position::Position;
use ferrumc_components::player::transform::rotation::Rotation;

#[derive(Event, Debug)]
pub struct TransformEvent {
    pub entity: Entity,
    pub position: Option<Position>,
    pub rotation: Option<Rotation>,
    pub on_ground: Option<bool>,
}

impl TransformEvent {
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

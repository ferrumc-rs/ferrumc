use bevy_ecs::prelude::{Entity, Event};

#[derive(Event)]
pub struct ConnectionKillEvent {
    pub entity: Entity,
    pub reason: Option<String>,
}

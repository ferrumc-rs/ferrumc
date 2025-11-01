use bevy_ecs::{entity::Entity, event::Event};

#[derive(Event)]
pub struct PlayerDisconnectEvent {
    pub entity: Entity,
}

use bevy_ecs::prelude::{Message, Event};

#[derive(Message)]
pub struct ConnectionKillEvent {
    pub entity: Entity,
    pub reason: Option<String>,
}

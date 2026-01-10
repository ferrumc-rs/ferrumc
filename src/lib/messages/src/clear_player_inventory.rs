use bevy_ecs::entity::Entity;
use bevy_ecs::message::Message;

#[derive(Message)]
pub struct ClearPlayerInventory {
    pub player: Entity,
}
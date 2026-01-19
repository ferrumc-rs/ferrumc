use bevy_ecs::entity::Entity;
use bevy_ecs::message::Message;

#[derive(Message)]
pub struct GiveItemToPlayer {
    pub player: Entity,
    pub item_id: u16,
    pub quantity: u32,
}

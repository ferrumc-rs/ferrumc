use bevy_ecs::entity::Entity;
use dashmap::DashMap;
use ferrumc_core::data::player::PlayerData;

#[derive(Debug, Default)]
pub struct PlayerState {
    player_data: DashMap<Entity, PlayerData>,
}

impl PlayerState {
    pub fn is_connected(&self, entity: Entity) -> bool {
        self.player_data.contains_key(&entity)
    }

    pub fn disconnect(&self, entity: Entity) {
        self.player_data.remove(&entity);
    }

    pub fn connect(&self, entity: Entity, data: PlayerData) {
        self.player_data.insert(entity, data);
    }
}

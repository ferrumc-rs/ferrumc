use bevy_ecs::entity::Entity;
use crossbeam_queue::SegQueue;
use dashmap::DashMap;

#[derive(Debug, Default)]
pub struct PlayerList {
    pub player_list: DashMap<Entity, (u128, String)>,
    pub disconnection_queue: SegQueue<(Entity, Option<String>)>,
}

impl PlayerList {
    pub fn is_connected(&self, entity: Entity) -> bool {
        self.player_list.contains_key(&entity)
    }

    pub fn disconnect(&self, entity: Entity, reason: Option<String>) {
        self.player_list.remove(&entity);
        self.disconnection_queue.push((entity, reason));
    }
}

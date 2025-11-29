use bevy_ecs::entity::Entity;
use crossbeam_queue::SegQueue;
use dashmap::DashMap;

#[derive(Debug, Default)]
pub struct PlayerList {
    // Key: Entity ID
    // Value: (UUID as u128, Username)
    pub player_list: DashMap<Entity, (u128, String)>,
    pub disconnection_queue: SegQueue<(Entity, Option<String>)>,
}

impl PlayerList {
    pub fn is_connected(&self, entity: Entity) -> bool {
        self.player_list.contains_key(&entity)
    }

    pub fn disconnect(&self, entity: Entity, reason: Option<String>) {
        // 1. Try to remove the player from the active list.
        //    `DashMap::remove` is atomic. It will only return `Some`
        //    for the first thread that calls this.

        let removal_result = self.player_list.remove(&entity);
        // ------------------------

        // 2. Only push to the disconnect queue if we were the
        //    first ones to successfully remove the player.
        if removal_result.is_some() {
            self.disconnection_queue.push((entity, reason));
        }
    }
}

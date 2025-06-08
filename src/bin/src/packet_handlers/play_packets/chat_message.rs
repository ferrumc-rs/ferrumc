use bevy_ecs::prelude::{EventWriter, Res};
use bevy_ecs::system::Query;
use ferrumc_core::transform::position::Position;
use ferrumc_entities::events::SpawnZombieEvent;
use ferrumc_net::ChatMessageReceiver;
use tracing::info;

pub fn handle(
    events: Res<ChatMessageReceiver>,
    pos_query: Query<&Position>,
    mut ev_spawn_zombie: EventWriter<SpawnZombieEvent>,
) {
    for (packet, entity_id) in events.0.try_iter() {
        info!("[CHAT] Received message: {}", packet.message);
        if matches!(packet.message.as_str(), "zombie") {
            info!("[CHAT] Zombie command received, spawning zombie!");

            let Ok(pos) = pos_query.get(entity_id) else {
                tracing::error!("Failed to get position for entity {:?}", entity_id);
                continue;
            };
            ev_spawn_zombie.write(SpawnZombieEvent {
                position: pos.clone(),
            });
        }
    }
}

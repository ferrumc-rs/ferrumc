use crate::systems::new_connections::NewConnectionRecv;
use bevy_ecs::prelude::World;
use crossbeam_channel::Receiver;
use ferrumc_core::chunks::world_sync_tracker::WorldSyncTracker;
use ferrumc_core::conn::player_count_update_cooldown::PlayerCountUpdateCooldown;
use ferrumc_net::connection::NewConnection;
use ferrumc_state::GlobalStateResource;

pub fn register_resources(
    world: &mut World,
    new_conn_recv: Receiver<NewConnection>,
    global_state: GlobalStateResource,
) {
    world.insert_resource(NewConnectionRecv(new_conn_recv));
    world.insert_resource(global_state);
    world.insert_resource(PlayerCountUpdateCooldown {
        last_update: std::time::Instant::now(),
    });
    world.insert_resource(WorldSyncTracker {
        last_synced: std::time::Instant::now(),
    });
}

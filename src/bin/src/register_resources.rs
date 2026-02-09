use crate::systems::handle_block_updates::BlockUpdates;
use crate::systems::new_connections::NewConnectionRecv;
use bevy_ecs::prelude::World;
use crossbeam_channel::Receiver;
use ferrumc_config::server_config::get_global_config;
use ferrumc_core::chunks::world_sync_tracker::WorldSyncTracker;
use ferrumc_core::time::WorldTime;
use ferrumc_entities::components::PhysicalRegistry;
use ferrumc_net::connection::NewConnection;
use ferrumc_performance::ServerPerformance;
use ferrumc_state::GlobalStateResource;

pub fn register_resources(
    world: &mut World,
    new_conn_recv: Receiver<NewConnection>,
    global_state: GlobalStateResource,
) {
    world.insert_resource(NewConnectionRecv(new_conn_recv));
    world.insert_resource(global_state);
    world.insert_resource(WorldSyncTracker {
        last_synced: std::time::Instant::now(),
    });
    world.insert_resource(WorldTime::default());
    world.insert_resource(ServerPerformance::new(get_global_config().tps));
    world.insert_resource(PhysicalRegistry::new());
    world.insert_resource(BlockUpdates::default());
}

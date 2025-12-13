use crate::systems::new_connections::NewConnectionRecv;
use bevy_ecs::prelude::{Resource, World};
use crossbeam_channel::Receiver;
use ferrumc_core::chunks::world_sync_tracker::WorldSyncTracker;
use ferrumc_net::connection::NewConnection;
use ferrumc_state::GlobalStateResource;
use tokio::runtime::Handle;

/// Resource that holds a handle to the Tokio runtime.
///
/// ECS systems run on the main thread which has no Tokio runtime context.
/// To spawn async tasks from ECS systems, use this handle instead of `tokio::spawn()`.
///
/// # Example
/// ```ignore
/// fn my_system(tokio_handle: Res<TokioHandle>) {
///     tokio_handle.0.spawn(async move {
///         // async work here
///     });
/// }
/// ```
#[derive(Resource)]
pub struct TokioHandle(pub Handle);

pub fn register_resources(
    world: &mut World,
    new_conn_recv: Receiver<NewConnection>,
    global_state: GlobalStateResource,
    tokio_handle: Handle,
) {
    world.insert_resource(NewConnectionRecv(new_conn_recv));
    world.insert_resource(global_state);
    world.insert_resource(WorldSyncTracker {
        last_synced: std::time::Instant::now(),
    });
    world.insert_resource(TokioHandle(tokio_handle));
}

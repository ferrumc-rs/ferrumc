use bevy_ecs::prelude::{Res, ResMut};
use ferrumc_core::chunks::world_sync_tracker::WorldSyncTracker;
use ferrumc_state::GlobalStateResource;

pub fn sync_world(state: Res<GlobalStateResource>, mut last_synced: ResMut<WorldSyncTracker>) {
    if state.0.shut_down.load(std::sync::atomic::Ordering::Relaxed) {
        return;
    }

    // Check if the world needs to be synced
    if last_synced.last_synced.elapsed().as_secs() >= 15 {
        tracing::info!("Syncing world...");
        state.0.world.sync().expect("Failed to sync world");

        // Update the last synced time
        last_synced.last_synced = std::time::Instant::now();
    }
}

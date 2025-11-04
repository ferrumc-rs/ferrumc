use bevy_ecs::prelude::{Res, ResMut};
use ferrumc_core::chunks::world_sync_tracker::WorldSyncTracker;
use ferrumc_state::GlobalStateResource;

pub fn sync_world(state: Res<GlobalStateResource>, mut last_synced: ResMut<WorldSyncTracker>) {
    if state.0.shut_down.load(std::sync::atomic::Ordering::Relaxed) {
        return;
    }

    // Always schedule a sync; frequency is handled by the schedule period.
    let _handle = state.0.thread_pools.io_pool.oneshot({
        let state = state.0.clone();
        move || {
            state.world.sync().expect("Failed to sync world");
        }
    });

    last_synced.last_synced = std::time::Instant::now();
}

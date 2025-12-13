mod chunk_calculator;
mod chunk_sending;
pub mod connection_killer;
pub mod debug_display;
pub mod keep_alive_system;
pub mod lan_pinger;
pub mod listeners;
mod mq;
pub mod new_connections;
pub mod shutdown_systems;
pub mod world;
pub mod world_sync;

pub fn register_game_systems(schedule: &mut bevy_ecs::schedule::Schedule) {
    // Tick-bound systems only (run every game tick)
    schedule.add_systems(new_connections::accept_new_connections);
    schedule.add_systems(mq::process);
    schedule.add_systems(debug_display::handle);

    // NOTE: The old chunk_calculator and chunk_sending systems are disabled.
    // Chunk loading is now handled by async per-player tasks spawned in new_connections.rs.
    // The tasks are notified via ChunkSender channels when players cross chunk boundaries.
    // This approach:
    // - Avoids O(N) per-tick iteration over all players
    // - Offloads chunk IO/serialization to background tasks
    // - Tasks sleep (0% CPU) when players are stationary
    //
    // Old systems (kept for reference, may be removed later):
    // schedule.add_systems(chunk_calculator::handle);
    // schedule.add_systems(chunk_sending::handle);

    // Should always be last
    schedule.add_systems(connection_killer::connection_killer);
}

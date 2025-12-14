pub mod connection_killer;
pub mod debug_display;
pub mod keep_alive_system;
pub mod lan_pinger;
pub mod listeners;
mod mq;
pub mod new_connections;
pub mod shutdown_systems;
pub mod time_sync;
pub mod world;
pub mod world_sync;

pub fn register_game_systems(schedule: &mut bevy_ecs::schedule::Schedule) {
    // Tick-bound systems only (run every game tick)
    schedule.add_systems(new_connections::accept_new_connections);
    schedule.add_systems(mq::process);
    schedule.add_systems(debug_display::handle);

    // Should always be last
    schedule.add_systems(connection_killer::connection_killer);
}

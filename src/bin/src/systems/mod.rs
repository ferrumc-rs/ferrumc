mod connection_killer;
mod cross_chunk_boundary;
mod keep_alive_system;
pub mod new_connections;
pub mod send_chunks;

pub fn register_game_systems(schedule: &mut bevy_ecs::schedule::Schedule) {
    schedule.add_systems(keep_alive_system::keep_alive_system);
    schedule.add_systems(new_connections::accept_new_connections);
    schedule.add_systems(cross_chunk_boundary::cross_chunk_boundary);

    // Should always be last
    schedule.add_systems(connection_killer::connection_killer);
}

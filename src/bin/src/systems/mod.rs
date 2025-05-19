mod keep_alive_system;
pub mod send_chunks;
pub mod new_connections;
mod connection_killer;

pub fn register_game_systems(schedule: &mut bevy_ecs::schedule::Schedule) {
    schedule.add_systems(keep_alive_system::keep_alive_system);
    schedule.add_systems(new_connections::accept_new_connections);
    schedule.add_systems(connection_killer::connection_killer);
    schedule.add_systems(send_chunks::chunk_sender_system);
}

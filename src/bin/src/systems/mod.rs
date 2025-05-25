use bevy_ecs::prelude::IntoScheduleConfigs;

pub mod connection_killer;
mod cross_chunk_boundary;
mod keep_alive_system;
pub mod new_connections;
mod player_count_update;
pub mod send_chunks;

pub fn register_game_systems(schedule: &mut bevy_ecs::schedule::Schedule) {
    schedule.add_systems(
        keep_alive_system::keep_alive_system.before(connection_killer::connection_killer),
    );
    schedule.add_systems(
        new_connections::accept_new_connections.before(connection_killer::connection_killer),
    );
    schedule.add_systems(
        cross_chunk_boundary::cross_chunk_boundary.before(connection_killer::connection_killer),
    );
    schedule.add_systems(
        player_count_update::player_count_updater.before(connection_killer::connection_killer),
    );

    // Should always be last
    schedule.add_systems(connection_killer::connection_killer);
}

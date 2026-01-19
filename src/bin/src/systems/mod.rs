use bevy_ecs::schedule::IntoScheduleConfigs;
mod chunk_calculator;
mod chunk_sending;
pub mod chunk_unloader;
pub mod connection_killer;
pub mod day_cycle;
pub mod emit_player_joined;
pub mod keep_alive_system;
pub mod lan_pinger;
pub mod listeners;
pub mod mobs;
mod mq;
pub mod new_connections;
mod particles;
pub mod physics;
mod player_swimming;
mod send_entity_updates;
pub mod shutdown_systems;
pub mod world_sync;

pub fn register_game_systems(schedule: &mut bevy_ecs::schedule::Schedule) {
    // Tick-bound systems only (run every game tick)
    // NOTE: accept_new_connections is registered separately in game_loop.rs
    // with apply_deferred and emit_player_joined chained after it.
    schedule.add_systems(
        (
            chunk_calculator::handle,
            chunk_sending::handle,
            // chunk_unloader::handle,
        )
            .chain(),
    );
    schedule.add_systems(mq::process);
    schedule.add_systems(player_swimming::detect_player_swimming);

    schedule.add_systems(send_entity_updates::handle);

    schedule.add_systems(day_cycle::tick_daylight_cycle);

    // Should always be last
    schedule.add_systems(connection_killer::connection_killer);
    schedule.add_systems(particles::handle);
}

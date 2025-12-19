mod chunk_calculator;
mod chunk_sending;
pub mod connection_killer;
pub mod keep_alive_system;
pub mod lan_pinger;
pub mod listeners;
pub mod mobs;
mod mq;
pub mod new_connections;
pub mod physics;
pub mod shutdown_systems;
pub mod world_sync;

pub fn register_game_systems(schedule: &mut bevy_ecs::schedule::Schedule) {
    // Tick-bound systems only (run every game tick)
    schedule.add_systems(new_connections::accept_new_connections);
    schedule.add_systems(chunk_calculator::handle);
    schedule.add_systems(chunk_sending::handle);
    schedule.add_systems(mq::process);

    // Entity physics and synchronization
    schedule.add_systems(physics::collisions::handle);
    schedule.add_systems(physics::drag::handle);
    schedule.add_systems(physics::gravity::handle);
    schedule.add_systems(physics::velocity::handle);

    // Should always be last
    schedule.add_systems(connection_killer::connection_killer);
}

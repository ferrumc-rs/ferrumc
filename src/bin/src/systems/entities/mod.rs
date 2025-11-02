pub mod entity_movement;
pub mod entity_spawner;
pub mod entity_sync;
pub mod entity_tick;
pub mod spawn_command_processor;

use bevy_ecs::schedule::Schedule;

/// Save all systems bind to entities
pub fn register_entity_systems(schedule: &mut Schedule) {
    schedule.add_systems((
        spawn_command_processor::spawn_command_processor_system, // Process spawn commands from /spawnpig
        entity_spawner::entity_spawner_system,
        entity_tick::pig_tick_system, // Tick AI/behavior for pigs
        entity_movement::entity_physics_system,
        entity_movement::entity_age_system,
        entity_sync::entity_sync_system,
    ));
}

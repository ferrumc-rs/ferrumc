pub mod entity_movement;
pub mod entity_movement_sync;
pub mod entity_spawner;
pub mod entity_sync;
pub mod entity_tick;
pub mod ground_check;
pub mod spawn_command_processor;

use bevy_ecs::schedule::Schedule;

/// Save all systems bind to entities
pub fn register_entity_systems(schedule: &mut Schedule) {
    schedule.add_systems((
        spawn_command_processor::spawn_command_processor_system, // Process spawn commands from /spawnpig
        entity_spawner::entity_spawner_system,
        ground_check::ground_check_system, // Check if entities are on ground
        entity_tick::pig_tick_system,      // Tick AI/behavior for pigs
        entity_movement::entity_physics_system, // Apply physics (gravity, movement)
        entity_movement::entity_age_system,
        entity_sync::entity_sync_system, // Sync new entities to clients
        entity_movement_sync::entity_movement_sync_system, // Sync entity movement to clients
    ));
}

use bevy_ecs::schedule::IntoScheduleConfigs;
pub mod collisions;
pub mod drag;
pub mod gravity;
pub mod velocity;

pub fn register_physics(schedule: &mut bevy_ecs::schedule::Schedule) {
    schedule.add_systems(
        (
            gravity::handle,
            drag::handle,
            velocity::handle,
            collisions::handle,
        )
            .chain(),
    );
}

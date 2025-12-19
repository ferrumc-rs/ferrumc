pub mod collisions;
pub mod drag;
pub mod gravity;
pub mod velocity;

pub fn register_physics(schedule: &mut bevy_ecs::schedule::Schedule) {
    schedule.add_systems(gravity::handle);
    schedule.add_systems(drag::handle);
    schedule.add_systems(velocity::handle);
    schedule.add_systems(collisions::handle);
}

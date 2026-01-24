pub mod block_break;

pub fn register_light_listeners(schedule: &mut bevy_ecs::schedule::Schedule) {
    schedule.add_systems(block_break::handle);
}

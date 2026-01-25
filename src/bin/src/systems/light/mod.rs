mod block_break;
mod block_place;

pub fn register_light_listeners(schedule: &mut bevy_ecs::schedule::Schedule) {
    schedule.add_systems(block_break::handle);
    schedule.add_systems(block_place::handle);
}

mod pig;

pub fn register_mob_systems(schedule: &mut bevy_ecs::schedule::Schedule) {
    schedule.add_systems(pig::tick_pig);
}

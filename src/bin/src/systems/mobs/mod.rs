mod combat;
mod pig;

pub fn register_mob_systems(schedule: &mut bevy_ecs::schedule::Schedule) {
    // pig::tick_pig is intentionally not registered yet.
    schedule.add_systems(combat::tick_combat);
}

mod combat;
mod pig;

#[expect(unused_parens)]
pub fn register_mob_systems(schedule: &mut bevy_ecs::schedule::Schedule) {
    schedule.add_systems(
        (
            // pig::tick_pig,
            combat::tick_combat
        ),
    );
}

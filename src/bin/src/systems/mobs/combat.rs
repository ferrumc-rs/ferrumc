use bevy_ecs::prelude::Query;
use ferrumc_entities::components::combat::CombatProperties;

pub fn tick_combat(mut query: Query<&mut CombatProperties>) {
    for mut combat in query.iter_mut() {
        combat.tick();
    }
}

use rand::prelude::IndexedRandom;

pub(crate) fn resolve_random_player(
    mut world: &mut bevy_ecs::prelude::World,
) -> Option<bevy_ecs::prelude::Entity> {
    use ferrumc_core::identity::player_identity::PlayerIdentity;
    let players: Vec<bevy_ecs::prelude::Entity> = world
        .query::<(bevy_ecs::prelude::Entity, &PlayerIdentity)>()
        .iter(&mut world)
        .map(|(entity, _)| entity)
        .collect();
    if players.is_empty() {
        None
    } else {
        let rng = &mut rand::rng();
        players.choose(rng).cloned()
    }
}

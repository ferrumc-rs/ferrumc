pub(crate) fn resolve_any_player(
    mut world: &mut bevy_ecs::prelude::World,
) -> Vec<bevy_ecs::prelude::Entity> {
    use ferrumc_core::identity::player_identity::PlayerIdentity;
    world.query::<(bevy_ecs::prelude::Entity, &PlayerIdentity)>().iter(&mut world).map(|(entity, _)| entity).collect()
}
use bevy_ecs::prelude::Entity;
use ferrumc_core::identity::entity_identity::EntityIdentity;
use ferrumc_core::identity::player_identity::PlayerIdentity;

pub(crate) fn resolve_any_entity(
    iter: impl Iterator<Item = (Entity, Option<&EntityIdentity>, Option<&PlayerIdentity>)>,
) -> Vec<Entity> {
    iter.map(|(entity, _, _)| entity).collect()
}

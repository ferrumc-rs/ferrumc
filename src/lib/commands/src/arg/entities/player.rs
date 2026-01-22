use bevy_ecs::prelude::Entity;
use ferrumc_core::identity::entity_identity::EntityIdentity;
use ferrumc_core::identity::player_identity::PlayerIdentity;

pub(crate) fn resolve_player_name(
    name: String,
    iter: impl Iterator<Item = (Entity, Option<&EntityIdentity>, Option<&PlayerIdentity>)>,
) -> Option<Entity> {
    for (entity, _, player_id) in iter {
        if let Some(identity) = player_id
            && identity.username == name
        {
            return Some(entity);
        }
    }
    None
}

use bevy_ecs::entity::Entity;
use ferrumc_core::identity::entity_identity::EntityIdentity;
use ferrumc_core::identity::player_identity::PlayerIdentity;

pub(crate) fn resolve_any_player(
    iter: impl Iterator<Item = (Entity, Option<&EntityIdentity>, Option<&PlayerIdentity>)>,
) -> Vec<Entity> {
    let mut players = Vec::new();
    for (entity, _, player_id) in iter {
        if player_id.is_some() {
            players.push(entity);
        }
    }
    players
}

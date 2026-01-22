use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::World;
use ferrumc_core::identity::entity_identity::EntityIdentity;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use uuid::Uuid;

pub(crate) fn resolve_uuid(
    uuid: Uuid,
    iter: impl Iterator<Item = (Entity, Option<&EntityIdentity>, Option<&PlayerIdentity>)>,
) -> Option<Entity> {
    for (entity, entity_id_opt, player_id_opt) in iter {
        if let Some(player_id) = player_id_opt {
            if player_id.uuid == uuid {
                return Some(entity);
            }
        }
        if let Some(entity_id) = entity_id_opt {
            if entity_id.uuid == uuid {
                return Some(entity);
            }
        }
    }
    None
}

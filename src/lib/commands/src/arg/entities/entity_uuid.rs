use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::World;
use uuid::Uuid;
use ferrumc_core::identity::entity_identity::EntityIdentity;
use ferrumc_core::identity::player_identity::PlayerIdentity;

pub(crate) fn resolve_uuid(uuid: Uuid, world: &mut World) -> Option<Entity> {
    let mut query = world.query::<(Entity, &PlayerIdentity)>();
    for (entity, identity) in query.iter(world) {
        if identity.uuid == uuid {
            return Some(entity);
        }
    }
    let mut query = world.query::<(Entity, &EntityIdentity)>();
    for (entity, identity) in query.iter(world) {
        if identity.uuid == uuid {
            return Some(entity);
        }
    }
    None
}
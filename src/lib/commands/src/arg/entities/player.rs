use bevy_ecs::prelude::{Entity, World};
use ferrumc_core::identity::player_identity::PlayerIdentity;

pub(crate) fn resolve_player_name(name: String, world: &mut World) -> Option<Entity> {
    let mut query = world.query::<(Entity, &PlayerIdentity)>();
    for (entity, identity) in query.iter(world) {
        if identity.username == name {
            return Some(entity);
        }
    }
    None
}

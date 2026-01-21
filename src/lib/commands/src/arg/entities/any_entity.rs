use bevy_ecs::prelude::Entity;

pub(crate) fn resolve_any_entity(world: &mut bevy_ecs::prelude::World) -> Vec<Entity> {
    world.query::<Entity>().iter(&mut *world).collect()
}

use crate::component::ComponentStorage;
use crate::entity::EntityManager;

mod entity;
mod component;
mod helpers;


fn main() {
    test_component();
}

fn test_entity() {
    let mut entity_manager = EntityManager::new();

    let entity1 = entity_manager.create_entity();
    let entity2 = entity_manager.create_entity();

    println!("Entity 1: {:?}", entity1); // Entity { id: 0, generation: 0 }
    println!("Entity 2: {:?}", entity2); // Entity { id: 1, generation: 0 }

    // Delete entity 1
    entity_manager.delete_entity(entity1);

    // Create a new entity
    let entity3 = entity_manager.create_entity();

    // Reuse the ID of the deleted entity
    println!("Entity 3: {:?}", entity3); // Entity { id: 0, generation: 1 }
}
fn test_component() {
    let component_storage = ComponentStorage::new();
}
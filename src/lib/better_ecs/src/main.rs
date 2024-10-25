use crate::components::ComponentManager;
use crate::entity::EntityAllocator;

mod entity;
mod components;

fn main() {
    let allocator = EntityAllocator::new();
    let storage = ComponentManager::new();

    {
        let some_entity = allocator.create();

        println!("Some entity: {}", some_entity);

        storage.insert::<i32>(some_entity, 5).unwrap();

        println!("Inserted i32 component");

        let mutable_ref = storage.get_mut::<i32>(some_entity).unwrap();

        println!("Mutable ref: {}", *mutable_ref);

        let immutable_ref = storage.get::<i32>(some_entity);

        assert_eq!(immutable_ref.as_deref(), None);
        
        drop(mutable_ref);
        
        let immutable_ref = storage.get::<i32>(some_entity);
        
        assert_eq!(immutable_ref.as_deref(), Some(&5));
    }
}
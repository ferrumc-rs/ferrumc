/*#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::thread;

    use crate::component::{ComponentStorage, DynamicComponent, Position};
    use crate::entity::EntityManager;

    #[derive(Debug, PartialEq)]
    struct TestComponent {
        value: i32,
    }
    impl DynamicComponent for TestComponent {}

    #[test]
    fn test_insert_and_get() {
        let storage = ComponentStorage::new();
        storage.insert(0usize, TestComponent { value: 42 });
        let component = storage.get::<TestComponent>(0usize);
        assert!(component.is_some());
        assert_eq!(component.unwrap().value, 42);
    }

    #[test]
    fn test_get_nonexistent() {
        let storage = ComponentStorage::new();
        let component = storage.get::<TestComponent>(0usize);
        assert!(component.is_none());
    }

    #[test]
    fn test_insert_overwrite() {
        let storage = ComponentStorage::new();
        storage.insert(0usize, TestComponent { value: 42 });
        storage.insert(0usize, TestComponent { value: 84 });
        let component = storage.get::<TestComponent>(0usize);
        assert_eq!(component.unwrap().value, 84);
    }

    #[test]
    fn test_multiple_component_types() {
        let storage = ComponentStorage::new();
        storage.insert(0usize, TestComponent { value: 42 });
        storage.insert(0usize, Position { x: 1.0, y: 2.0 });

        let test_component = storage.get::<TestComponent>(0usize);
        let position = storage.get::<Position>(0usize);

        let position = position.unwrap();

        assert_eq!(test_component.unwrap().value, 42);
        assert_eq!(position.x, 1.0);
        assert_eq!(position.y, 2.0);
    }

    #[test]
    fn test_concurrent_access() {
        let storage = Arc::new(ComponentStorage::new());
        let storage_clone = Arc::clone(&storage);

        storage.insert(0usize, TestComponent { value: 0 });

        let thread = thread::spawn(move || {
            for i in 0..1000 {
                storage_clone.insert(0usize, TestComponent { value: i });
            }
        });

        for i in 1000..2000 {
            storage.insert(0usize, TestComponent { value: i });
        }

        thread.join().unwrap();

        let final_value = storage.get::<TestComponent>(0usize).unwrap().value;
        assert!(final_value >= 999 && final_value < 2000);
    }

    #[test]
    fn test_large_entity_id() {
        let mut em = EntityManager::new();
        let storage = ComponentStorage::new();

        for _ in 0..1_000_000_000 {
            em.create_entity();
        }

        let large_id = 10_000_000usize;

        storage.insert(large_id, TestComponent { value: 42 });
        let component = storage.get::<TestComponent>(large_id);

        assert_eq!(component.unwrap().value, 42);
    }

    #[test]
    fn test_multiple_reads() {
        let storage = ComponentStorage::new();
        storage.insert(0usize, TestComponent { value: 42 });

        let read1 = storage.get::<TestComponent>(0usize);
        let read2 = storage.get::<TestComponent>(0usize);

        assert_eq!(read1.unwrap().value, 42);
        assert_eq!(read2.unwrap().value, 42);
    }

    #[test]
    fn test_insert_different_type() {
        let storage = ComponentStorage::new();
        storage.insert(0usize, TestComponent { value: 42 });
        storage.insert(0usize, Position { x: 1.0, y: 2.0 });

        let test_component = storage.get::<TestComponent>(0usize);
        let position = storage.get::<Position>(0usize);

        let position = position.unwrap();

        assert_eq!(test_component.unwrap().value, 42);
        assert_eq!(position.x, 1.0);
        assert_eq!(position.y, 2.0);
    }
}*/
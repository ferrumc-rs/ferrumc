/*use std::sync::{Arc, Mutex};
use crate::component::{ComponentStorage, DynamicComponent};

#[derive(Debug)]
struct DropCounter {
    count: Arc<Mutex<usize>>,
}

impl DropCounter {
    fn new(count: Arc<Mutex<usize>>) -> Self {
        Self { count }
    }
}

impl Drop for DropCounter {
    fn drop(&mut self) {
        let mut count = self.count.lock().unwrap();
        *count += 1;
    }
}

#[derive(Debug)]
struct TestComponent {
    _counter: DropCounter,
}

impl DynamicComponent for TestComponent {}

#[tokio::test]
pub async fn test_component_storage_memory_leak() {
    let drop_count = Arc::new(Mutex::new(0));

    {
        let storage = ComponentStorage::new();

        // Insert components
        for i in 0..100 {
            let component = TestComponent {
                _counter: DropCounter::new(Arc::clone(&drop_count)),
            };
            storage.insert(i as usize, component);
        }

        // Access components
        for i in 0..100 {
            let _component = storage.get::<TestComponent>(i as usize).await;
        }

        // Remove components
        for i in 0..100 {
            storage.remove::<TestComponent>(i as usize);
        }

        // Insert new components
        for i in 0..50 {
            let component = TestComponent {
                _counter: DropCounter::new(Arc::clone(&drop_count)),
            };
            storage.insert(i as usize, component);
        }
    } // ComponentStorage is dropped here

    // Wait a bit to ensure all async operations are completed
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let final_drop_count = *drop_count.lock().unwrap();
    println!("Expected 150 drops, got {}", final_drop_count);
}*/

#[cfg(test)]
mod tests_pro_max {
    use std::sync::{Arc, Mutex};

    use tokio::sync::Semaphore;

    use crate::ecs::component::{Component, ComponentStorage};

    #[derive(Debug)]
    struct DropCounter {
        count: Arc<Mutex<usize>>,
    }

    impl DropCounter {
        fn new(count: Arc<Mutex<usize>>) -> Self {
            Self { count }
        }
    }

    impl Drop for DropCounter {
        fn drop(&mut self) {
            let mut count = self.count.lock().unwrap();
            *count += 1;
        }
    }

    #[derive(Debug)]
    struct TestComponent {
        _counter: DropCounter,
    }

    impl Component for TestComponent {}

    // TODO: Fix this test
    #[tokio::test]
    #[ignore]
    async fn test_component_storage_complex_scenarios() {
        let drop_count = Arc::new(Mutex::new(0));
        let storage = Arc::new(ComponentStorage::new());

        // Scenario 1: Concurrent insertions and removals
        let concurrent_ops = 1000;
        let semaphore = Arc::new(Semaphore::new(100)); // Limit concurrent tasks

        let mut handles = vec![];
        for i in 0..concurrent_ops {
            let storage_clone = Arc::clone(&storage);
            let drop_count_clone = Arc::clone(&drop_count);
            let sem_clone = Arc::clone(&semaphore);

            let handle = tokio::spawn(async move {
                let _permit = sem_clone.acquire().await.unwrap();
                let component = TestComponent {
                    _counter: DropCounter::new(Arc::clone(&drop_count_clone)),
                };
                storage_clone.insert(i, component);

                // 50% chance of immediate removal
                if i % 2 == 0 {
                    storage_clone.remove::<TestComponent>(i).unwrap();
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }

        // Scenario 2: Overwriting components
        for i in 0..100 {
            let component = TestComponent {
                _counter: DropCounter::new(Arc::clone(&drop_count)),
            };
            storage.insert(i as usize, component);
        }

        // Scenario 3: Accessing non-existent components
        for i in concurrent_ops..(concurrent_ops + 100) {
            let _ = storage.get::<TestComponent>(i).await;
        }

        // Scenario 4: Removing non-existent components
        for i in concurrent_ops..(concurrent_ops + 100) {
            storage.remove::<TestComponent>(i).unwrap();
        }

        // Shouldn't exactly happen like ever the bottom thing 👇
        /*// Scenario 5: Inserting at very large indices
        let large_indices = [usize::MAX, usize::MAX - 1, usize::MAX / 2];
        for &i in &large_indices {
            let component = TestComponent {
                _counter: DropCounter::new(Arc::clone(&drop_count)),
            };
            storage.insert(i, component);
        }*/

        // Allow time for async operations to complete
        // tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        // Drop the storage
        drop(storage);

        // Wait a bit more to ensure all components are dropped
        // tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        let final_drop_count = *drop_count.lock().unwrap();
        let expected_drop_count = concurrent_ops + 100; // 1000 concurrent ops, 100 overwrites, 100 accesses, 100 removals
        assert_eq!(
            final_drop_count, expected_drop_count,
            "Expected {} drops, got {}",
            expected_drop_count, final_drop_count
        );
    }
}

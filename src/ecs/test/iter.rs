#![cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::Duration;

    use futures::future::join_all;
    use tokio::sync::Barrier;

    use crate::ecs::component::{Component, ComponentStorage};
    use crate::ecs::entity::EntityManager;
    use crate::ecs::query::Query;
    use crate::utils::encoding::position::Position;
    use crate::utils::encoding::velocity::Velocity;

    #[derive(Debug, Clone, Copy)]
    struct Health(f32);
    impl Component for Health {}

    #[tokio::test]
    async fn test_basic_query() {
        let storage = ComponentStorage::new();
        let entity_manager = EntityManager::new();

        let entity = entity_manager.create_entity().await;
        storage.insert(entity, Position { x: 1, y: 2, z: 0 });

        let query = Query::<&Position>::new(&entity_manager, &storage);
        let results: Vec<_> = query.iter().await.collect();

        let entity_id: usize = entity.into();

        assert_eq!(results.len(), 1);
        assert_eq!(entity_id, results[0].0);
        assert_eq!(results[0].1.x, 1);
        assert_eq!(results[0].1.y, 2);
    }

    #[tokio::test]
    async fn test_multi_component_query() {
        let storage = ComponentStorage::new();
        let entity_manager = EntityManager::new();

        let entity1 = entity_manager.create_entity().await;
        let entity2 = entity_manager.create_entity().await;

        storage.insert(entity1, Position { x: 1, y: 2, z: 0 });
        storage.insert(entity1, Velocity { x: 3, y: 4, z: 0 });
        storage.insert(entity2, Position { x: 5, y: 6, z: 0 });

        let query = Query::<(&mut Position, &Velocity)>::new(&entity_manager, &storage);
        let results: Vec<_> = query.iter().await.collect();

        let entity1_id: usize = entity1.into();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, entity1_id);
        assert_eq!(results[0].1 .0.x, 1);
        assert_eq!(results[0].1 .1.x, 3);
    }

    #[tokio::test]
    async fn test_mutable_query() {
        let storage = ComponentStorage::new();
        let entity_manager = EntityManager::new();

        let entity = entity_manager.create_entity().await;
        storage.insert(entity, Position { x: 1, y: 2, z: 0 });

        let query = Query::<&mut Position>::new(&entity_manager, &storage);
        for (_, mut pos) in query.iter().await {
            pos.x += 1;
        }

        let query = Query::<&Position>::new(&entity_manager, &storage);
        let results: Vec<_> = query.iter().await.collect();
        assert_eq!(results[0].1.x, 2);
    }

    #[tokio::test]
    async fn test_concurrent_reads() {
        let storage = Arc::new(ComponentStorage::new());
        let entity_manager = EntityManager::new();

        for i in 0..1000 {
            let entity = entity_manager.create_entity().await;
            storage.insert(entity, Position { x: i, y: 0, z: 0 });
        }

        let entity_manager = Arc::new(entity_manager);
        let barrier = Arc::new(Barrier::new(10));

        let handles: Vec<_> = (0..10)
            .map(|_| {
                let storage_clone = storage.clone();
                let entity_manager_clone = entity_manager.clone();
                let barrier_clone = barrier.clone();

                tokio::spawn(async move {
                    barrier_clone.wait().await;
                    let query = Query::<&Position>::new(&entity_manager_clone, &storage_clone);
                    let results: Vec<_> = query.iter().await.collect();
                    assert_eq!(results.len(), 1000);
                })
            })
            .collect();

        join_all(handles).await;
    }

    #[tokio::test]
    async fn test_concurrent_writes() {
        let storage = Arc::new(ComponentStorage::new());
        let entity_manager = EntityManager::new();

        for i in 0..1000 {
            let entity = entity_manager.create_entity().await;
            storage.insert(entity, Position { x: i, y: 0, z: 0 });
        }

        let entity_manager = Arc::new(entity_manager);
        let barrier = Arc::new(Barrier::new(10));

        let handles: Vec<_> = (0..10)
            .map(|_| {
                let storage_clone = storage.clone();
                let entity_manager_clone = entity_manager.clone();
                let barrier_clone = barrier.clone();

                tokio::spawn(async move {
                    barrier_clone.wait().await;
                    let query = Query::<&mut Position>::new(&entity_manager_clone, &storage_clone);
                    for (_, mut pos) in query.iter().await {
                        pos.x += 1;
                    }
                })
            })
            .collect();

        join_all(handles).await;

        let query = Query::<&Position>::new(&entity_manager, &storage);
        let results: Vec<_> = query.iter().await.collect();
        for (i, (_, pos)) in results.into_iter().enumerate() {
            assert_eq!(pos.x, (i + 10) as i32);
        }
    }

    // TODO: Fix this test
    #[tokio::test]
    #[ignore]
    async fn test_mixed_queries() {
        let storage = Arc::new(ComponentStorage::new());
        let entity_manager = EntityManager::new();

        for i in 0..1000 {
            let entity = entity_manager.create_entity().await;
            storage.insert(entity, Position { x: i, y: 0, z: 0 });
            storage.insert(entity, Velocity { x: 1, y: 1, z: 0 });
            storage.insert(entity, Health(100.0));
        }

        let entity_manager = Arc::new(entity_manager);
        let barrier = Arc::new(Barrier::new(3));

        let read_handle = tokio::spawn({
            let storage_clone = storage.clone();
            let entity_manager_clone = entity_manager.clone();
            let barrier_clone = barrier.clone();
            async move {
                barrier_clone.wait().await;
                let query = Query::<&Position>::new(&entity_manager_clone, &storage_clone);
                for _ in 0..100 {
                    let _: Vec<_> = query.iter().await.collect();
                    tokio::time::sleep(Duration::from_millis(1)).await;
                }
            }
        });

        let write_handle = tokio::spawn({
            let storage_clone = storage.clone();
            let entity_manager_clone = entity_manager.clone();
            let barrier_clone = barrier.clone();
            async move {
                barrier_clone.wait().await;
                let query = Query::<&mut Velocity>::new(&entity_manager_clone, &storage_clone);
                for _ in 0..50 {
                    for (_, mut vel) in query.iter().await {
                        vel.x += 0.1 as i32;
                        vel.y += 0.1 as i16;
                    }
                    tokio::time::sleep(Duration::from_millis(2)).await;
                }
            }
        });

        let complex_handle = tokio::spawn({
            let storage_clone = storage.clone();
            let entity_manager_clone = entity_manager.clone();
            let barrier_clone = barrier.clone();
            async move {
                barrier_clone.wait().await;
                let query = Query::<(&Position, &Velocity, &mut Health)>::new(
                    &entity_manager_clone,
                    &storage_clone,
                );
                for _ in 0..25 {
                    for (_, (pos, vel, mut health)) in query.iter().await {
                        health.0 -= ((pos.x * vel.x).abs() + 1) as f32;
                    }
                    tokio::time::sleep(Duration::from_millis(4)).await;
                }
            }
        });

        join_all(vec![read_handle, write_handle, complex_handle]).await;

        // Verify final state
        let query = Query::<(&Position, &Velocity, &Health)>::new(&entity_manager, &storage);
        let results: Vec<_> = query.iter().await.collect();
        assert_eq!(results.len(), 1000);
        for (i, (_, (pos, vel, health))) in results.into_iter().enumerate() {
            assert_eq!(pos.x, i as i32);
            assert!(vel.x > 1 && vel.x <= 6);
            assert!(vel.y > 1 && vel.y <= 6);
            assert!(health.0 < 100f32);
        }
    }

    #[tokio::test]
    async fn test_edge_cases() {
        let storage = ComponentStorage::new();
        let entity_manager = EntityManager::new();

        // Test query on empty storage
        {
            let query = Query::<&Position>::new(&entity_manager, &storage);
            let results: Vec<_> = query.iter().await.collect();
            assert_eq!(results.len(), 0);
        }

        let entity = entity_manager.create_entity().await;
        // Test query with non-existent component
        {
            storage.insert(entity, Position { x: 1, y: 2, z: 0 });
            let query = Query::<&Velocity>::new(&entity_manager, &storage);
            let results: Vec<_> = query.iter().await.collect();
            assert_eq!(results.len(), 0);
        }

        // Test query after removing a component
        {
            storage.remove::<Position>(entity).unwrap();
            let query = Query::<&Position>::new(&entity_manager, &storage);
            let results: Vec<_> = query.iter().await.collect();
            assert_eq!(results.len(), 0);
        }

        // Test query with maximum number of components (6 in this case)
        #[derive(Debug, Clone, Copy)]
        struct A(f32);
        impl Component for A {}
        #[derive(Debug, Clone, Copy)]
        struct B(f32);
        impl Component for B {}
        #[derive(Debug, Clone, Copy)]
        struct C(f32);
        impl Component for C {}

        let entity = entity_manager.create_entity().await;
        storage.insert(entity, Position { x: 1, y: 2, z: 0 });
        storage.insert(entity, Velocity { x: 3, y: 4, z: 0 });
        storage.insert(entity, Health(100.0));
        storage.insert(entity, A(5.0));
        storage.insert(entity, B(6.0));
        storage.insert(entity, C(7.0));

        let query =
            Query::<(&Position, &Velocity, &Health, &A, &B, &C)>::new(&entity_manager, &storage);
        let results: Vec<_> = query.iter().await.collect();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].1 .0.x, 1);
        assert_eq!(results[0].1 .1.x, 3);
        assert_eq!(results[0].1 .2 .0, 100f32);
        assert_eq!(results[0].1 .3 .0, 5f32);
        assert_eq!(results[0].1 .4 .0, 6f32);
        assert_eq!(results[0].1 .5 .0, 7f32);
    }

    #[tokio::test]
    async fn test_memory_safety() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Weak;

        static DROP_COUNT: AtomicUsize = AtomicUsize::new(0);

        #[derive(Debug)]
        #[allow(dead_code)]
        struct DropCounter(Weak<()>);

        impl Drop for DropCounter {
            fn drop(&mut self) {
                DROP_COUNT.fetch_add(1, Ordering::SeqCst);
            }
        }

        impl Component for DropCounter {}

        {
            let storage = ComponentStorage::new();
            let entity_manager = EntityManager::new();

            let rc = Arc::new(());
            let weak = Arc::downgrade(&rc);

            for _ in 0..1000 {
                let entity = entity_manager.create_entity().await;
                storage.insert(entity, DropCounter(weak.clone()));
            }

            let query = Query::<&DropCounter>::new(&entity_manager, &storage);
            let _results: Vec<_> = query.iter().await.collect();

            drop(rc);
        }

        assert_eq!(DROP_COUNT.load(Ordering::SeqCst), 1000);
    }

    #[tokio::test]
    async fn test_concurrent_read_write_with_rayon() {
        use rayon::iter::ParallelIterator;
        use rayon::prelude::IntoParallelIterator;
        use std::sync::atomic::{AtomicBool, Ordering};
        use tokio::time::{sleep, Duration};

        let storage = Arc::new(ComponentStorage::new());
        let entity_manager = EntityManager::new();

        // Create entities with positions
        for i in 0..10 {
            let entity = entity_manager.create_entity().await;
            storage.insert(entity, Position { x: i, y: 0, z: 0 });
        }

        let entity_manager = Arc::new(entity_manager);
        let running = Arc::new(AtomicBool::new(true));

        // Writer task
        let write_handle = {
            let storage_clone = storage.clone();
            let entity_manager_clone = entity_manager.clone();
            let running_clone = running.clone();
            tokio::spawn(async move {
                let mut iteration = 0;
                while running_clone.load(Ordering::Relaxed) {
                    let query = Query::<&mut Position>::new(&entity_manager_clone, &storage_clone);
                    let results: Vec<_> = query.iter().await.collect();
                    results.into_par_iter().for_each(|(entity_id, mut pos)| {
                        pos.x += 1;
                        println!(
                            "Writer {}: Updated entity {} position to ({}, {})",
                            iteration, entity_id, pos.x, pos.y
                        );
                    });
                    iteration += 1;
                    sleep(Duration::from_millis(50)).await;
                }
            })
        };

        // Reader tasks
        let read_handles: Vec<_> = (0..2)
            .map(|reader_id| {
                let storage_clone = storage.clone();
                let entity_manager_clone = entity_manager.clone();
                let running_clone = running.clone();
                tokio::spawn(async move {
                    let mut iteration = 0;
                    while running_clone.load(Ordering::Relaxed) {
                        let query = Query::<&Position>::new(&entity_manager_clone, &storage_clone);
                        let results: Vec<_> = query.iter().await.collect();
                        results.into_par_iter().for_each(|(entity_id, pos)| {
                            println!(
                                "Reader {} (Iteration {}): Read entity {} position as ({}, {})",
                                reader_id, iteration, entity_id, pos.x, pos.y
                            );
                        });
                        iteration += 1;
                        sleep(Duration::from_millis(25)).await;
                    }
                })
            })
            .collect();

        // Let the tasks run for a while
        sleep(Duration::from_secs(1)).await;

        // Signal tasks to stop
        running.store(false, Ordering::Relaxed);

        // Wait for all tasks to complete
        write_handle.await.unwrap();
        for handle in read_handles {
            handle.await.unwrap();
        }

        // Verify final state
        let query = Query::<&Position>::new(&entity_manager, &storage);
        let results: Vec<_> = query.iter().await.collect();
        results.into_par_iter().for_each(|(entity_id, pos)| {
            println!(
                "Final: Entity {} position is ({}, {})",
                entity_id, pos.x, pos.y
            );
            assert!(
                pos.x >= entity_id as i32,
                "Unexpected final position for entity {}",
                entity_id
            );
        });
    }
}

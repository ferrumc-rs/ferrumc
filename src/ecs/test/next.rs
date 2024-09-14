#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::ecs::component::ComponentStorage;
    use crate::ecs::entity::EntityManager;
    use crate::ecs::query::Query;
    use crate::utils::encoding::position::Position;
    use crate::utils::encoding::velocity::Velocity;

    #[tokio::test]
    async fn test_next_basic() {
        let storage = ComponentStorage::new();
        let entity_manager = EntityManager::new();

        let entity1 = entity_manager.create_entity().await;
        let entity2 = entity_manager.create_entity().await;

        storage.insert(entity1, Position { x: 1, y: 2, z: 0 });
        storage.insert(entity2, Position { x: 3, y: 4, z: 0 });

        let mut query = Query::<&Position>::new(&entity_manager, &storage);

        let (id1, pos1) = query.next().await.unwrap();
        assert_eq!(id1, entity1.id as usize);
        assert_eq!(pos1.x, 1);
        assert_eq!(pos1.y, 2);

        let (id2, pos2) = query.next().await.unwrap();
        assert_eq!(id2, entity2.id as usize);
        assert_eq!(pos2.x, 3);
        assert_eq!(pos2.y, 4);

        assert!(query.next().await.is_none());
    }

    #[tokio::test]
    async fn test_next_multi_component() {
        let storage = ComponentStorage::new();
        let entity_manager = EntityManager::new();

        let entity1 = entity_manager.create_entity().await;
        let entity2 = entity_manager.create_entity().await;

        storage.insert(entity1, Position { x: 1, y: 2, z: 0 });
        storage.insert(entity1, Velocity { x: 3, y: 4, z: 0 });
        storage.insert(entity2, Position { x: 5, y: 6, z: 0 });

        let mut query = Query::<(&Position, &Velocity)>::new(&entity_manager, &storage);

        let (id1, (pos1, vel1)) = query.next().await.unwrap();
        assert_eq!(id1, entity1.id as usize);
        assert_eq!(pos1.x, 1);
        assert_eq!(vel1.x, 3);

        assert!(query.next().await.is_none());
    }

    #[tokio::test]
    async fn test_next_mutable() {
        let storage = ComponentStorage::new();
        let entity_manager = EntityManager::new();

        let entity = entity_manager.create_entity().await;
        storage.insert(entity, Position { x: 1, y: 2, z: 0 });

        let mut query = Query::<&mut Position>::new(&entity_manager, &storage);

        let (id, mut pos) = query.next().await.unwrap();
        assert_eq!(id, entity.id as usize);
        pos.x += 1;

        drop(pos); // Explicitly drop the RwLockWriteGuard.

        assert!(query.next().await.is_none());

        let query = Query::<&Position>::new(&entity_manager, &storage);
        let results: Vec<_> = query.iter().await.collect();
        assert_eq!(results[0].1.x, 2);
    }

    #[tokio::test]
    async fn test_next_concurrent() {
        let storage = Arc::new(ComponentStorage::new());
        let entity_manager = EntityManager::new();

        for i in 0..100 {
            let entity = entity_manager.create_entity().await;
            storage.insert(entity, Position { x: i, y: 0, z: 0 });
        }

        let entity_manager = Arc::new(entity_manager);

        let handles: Vec<_> = (0..10)
            .map(|_| {
                let storage_clone = storage.clone();
                let entity_manager_clone = entity_manager.clone();

                tokio::spawn(async move {
                    let mut query = Query::<&Position>::new(&entity_manager_clone, &storage_clone);
                    let mut count = 0;
                    while (query.next().await).is_some() {
                        count += 1;
                    }
                    count
                })
            })
            .collect();

        let results = futures::future::join_all(handles)
            .await
            .into_iter()
            .map(|r| r.unwrap())
            .collect::<Vec<_>>();
        assert_eq!(results.iter().sum::<usize>(), 1000);
    }

    #[tokio::test]
    async fn test_next_with_reads_and_writes() {
        let storage = Arc::new(ComponentStorage::new());
        let entity_manager = EntityManager::new();

        for i in 0..100 {
            let entity = entity_manager.create_entity().await;
            storage.insert(entity, Position { x: i, y: 0, z: 0 });
        }

        let entity_manager = Arc::new(entity_manager);

        let read_handle = tokio::spawn({
            let storage_clone = storage.clone();
            let entity_manager_clone = entity_manager.clone();
            async move {
                let mut query = Query::<&Position>::new(&entity_manager_clone, &storage_clone);
                let mut count = 0;
                while (query.next().await).is_some() {
                    count += 1;
                    tokio::time::sleep(std::time::Duration::from_millis(1)).await;
                }
                count
            }
        });

        let write_handle = tokio::spawn({
            let storage_clone = storage.clone();
            let entity_manager_clone = entity_manager.clone();
            async move {
                let mut query = Query::<&mut Position>::new(&entity_manager_clone, &storage_clone);
                let mut count = 0;
                while let Some((_, mut pos)) = query.next().await {
                    pos.x += 1;
                    count += 1;
                    tokio::time::sleep(std::time::Duration::from_millis(1)).await;
                }
                count
            }
        });

        let (read_count, write_count) = tokio::join!(read_handle, write_handle);
        assert_eq!(read_count.unwrap(), 100);
        assert_eq!(write_count.unwrap(), 100);
    }
}

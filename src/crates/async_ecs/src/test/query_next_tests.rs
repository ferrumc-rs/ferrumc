#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::component::{ComponentStorage, Position, Velocity};
    use crate::entity::EntityManager;
    use crate::query::Query;

    #[tokio::test]
    async fn test_next_basic() {
        let storage = ComponentStorage::new();
        let mut entity_manager = EntityManager::new();

        let entity1 = entity_manager.create_entity();
        let entity2 = entity_manager.create_entity();

        storage.insert(entity1, Position { x: 1.0, y: 2.0 });
        storage.insert(entity2, Position { x: 3.0, y: 4.0 });


        let mut query = Query::<&Position>::new(&entity_manager, &storage);

        let (id1, pos1) = query.next().await.unwrap();
        assert_eq!(id1, entity1.into());
        assert_eq!(pos1.x, 1.0);
        assert_eq!(pos1.y, 2.0);

        let (id2, pos2) = query.next().await.unwrap();
        assert_eq!(id2, entity2.into());
        assert_eq!(pos2.x, 3.0);
        assert_eq!(pos2.y, 4.0);

        assert!(query.next().await.is_none());
    }

    #[tokio::test]
    async fn test_next_multi_component() {
        let storage = ComponentStorage::new();
        let mut entity_manager = EntityManager::new();

        let entity1 = entity_manager.create_entity();
        let entity2 = entity_manager.create_entity();

        storage.insert(entity1, Position { x: 1.0, y: 2.0 });
        storage.insert(entity1, Velocity { x: 3.0, y: 4.0 });
        storage.insert(entity2, Position { x: 5.0, y: 6.0 });

        let mut query = Query::<(&Position, &Velocity)>::new(&entity_manager, &storage);

        let (id1, (pos1, vel1)) = query.next().await.unwrap();
        assert_eq!(id1, entity1.into());
        assert_eq!(pos1.x, 1.0);
        assert_eq!(vel1.x, 3.0);

        assert!(query.next().await.is_none());
    }

    #[tokio::test]
    async fn test_next_mutable() {
        let storage = ComponentStorage::new();
        let mut entity_manager = EntityManager::new();

        let entity = entity_manager.create_entity();
        storage.insert(entity, Position { x: 1.0, y: 2.0 });

        let mut query = Query::<&mut Position>::new(&entity_manager, &storage);

        let (id, mut pos) = query.next().await.unwrap();
        assert_eq!(id, entity.into());
        pos.x += 1.0;
        
        drop(pos); // Explicitly drop the RwLockWriteGuard.

        assert!(query.next().await.is_none());
        
        let query = Query::<&Position>::new(&entity_manager, &storage);
        let results: Vec<_> = query.iter().await.collect();
        assert_eq!(results[0].1.x, 2.0);
    }

    #[tokio::test]
    async fn test_next_concurrent() {
        let storage = Arc::new(ComponentStorage::new());
        let mut entity_manager = EntityManager::new();

        for i in 0..100 {
            let entity = entity_manager.create_entity();
            storage.insert(entity, Position { x: i as f32, y: 0.0 });
        }

        let entity_manager = Arc::new(entity_manager);

        let handles: Vec<_> = (0..10)
            .map(|_| {
                let storage_clone = storage.clone();
                let entity_manager_clone = entity_manager.clone();

                tokio::spawn(async move {
                    let mut query = Query::<&Position>::new(&entity_manager_clone, &storage_clone);
                    let mut count = 0;
                    while let Some(_) = query.next().await {
                        count += 1;
                    }
                    count
                })
            })
            .collect();

        let results = futures::future::join_all(handles).await.into_iter().map(|r| r.unwrap()).collect::<Vec<_>>();
        assert_eq!(results.iter().sum::<usize>(), 1000);
    }

    #[tokio::test]
    async fn test_next_with_reads_and_writes() {
        let storage = Arc::new(ComponentStorage::new());
        let mut entity_manager = EntityManager::new();

        for i in 0..100 {
            let entity = entity_manager.create_entity();
            storage.insert(entity, Position { x: i as f32, y: 0.0 });
        }

        let entity_manager = Arc::new(entity_manager);

        let read_handle = tokio::spawn({
            let storage_clone = storage.clone();
            let entity_manager_clone = entity_manager.clone();
            async move {
                let mut query = Query::<&Position>::new(&entity_manager_clone, &storage_clone);
                let mut count = 0;
                while let Some(_) = query.next().await {
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
                    pos.x += 1.0;
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
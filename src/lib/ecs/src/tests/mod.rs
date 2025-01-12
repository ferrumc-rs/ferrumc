use crate::components::ComponentManager;
use crate::entities::EntityManager;
use crate::query::Query;
use crate::Universe;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashSet;
use std::random::random;
use std::sync::atomic::AtomicU16;
use std::sync::Arc;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Position {
    x: u32,
    y: u32,
}

unsafe impl Send for Position {}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Player {
    username: String,
}

unsafe impl Send for Player {}

#[tokio::test]
async fn test_basic() {
    let entity_manager = EntityManager::new();
    let component_storage = ComponentManager::new();

    for x in 0..10 {
        entity_manager
            .builder(&component_storage)
            .with(Position { x, y: x * 2 })
            .await
            .unwrap()
            .with(Player {
                username: format!("Player{}", x),
            })
            .await
            .unwrap()
            .build();
    }
    let mut query = Query::<(&Player, &mut Position)>::new(&component_storage).await;

    let mut count = 0;

    while let Some((_player, _position)) = query.next().await {
        count += 1;
    }

    assert_eq!(count, 10);
}

#[tokio::test]
async fn test_query() {
    let entity_manager = EntityManager::new();
    let component_storage = ComponentManager::new();

    let mut test_values: HashSet<((u32, u32), String)> = HashSet::new();

    let values: Vec<(Position, Player)> = (0..10)
        .map(|x| {
            (
                Position { x, y: x * 2 },
                Player {
                    username: format!("Player{}", x),
                },
            )
        })
        .collect();
    for (position, player) in &values {
        test_values.insert(((position.x, position.y), player.username.clone()));
    }

    for (position, player) in values {
        entity_manager
            .builder(&component_storage)
            .with(position)
            .await
            .unwrap()
            .with(player)
            .await
            .unwrap()
            .build();
    }

    let mut query = Query::<(&Player, &Position)>::new(&component_storage).await;

    while let Some((_eid, (player, position))) = query.next().await {
        assert!(test_values.contains(&((position.x, position.y), player.username.clone())));
    }
}

#[tokio::test]
async fn test_fetch() {
    let universe = crate::Universe::new();
    universe
        .builder()
        .with(Position { x: 0, y: 50 })
        .await
        .unwrap()
        .build();
    let mut q = universe.query::<&Position>().await;
    let res = q.next().await;
    assert!(res.is_some());
    let (_eid, pos) = res.unwrap();
    assert_eq!(pos.x, 0);
    assert_eq!(pos.y, 50);
}

#[tokio::test]
async fn test_false_fetch() {
    let universe = crate::Universe::new();
    universe.builder().build();
    let mut q = universe.query::<&Player>().await;
    let res = q.next().await;
    assert!(res.is_none());
}

#[tokio::test]
async fn test_concurrent_contention() {
    let universe = Arc::new(crate::Universe::new());
    for _ in 0..1_000 {
        universe
            .builder()
            .with(Position {
                x: random(),
                y: random(),
            })
            .await
            .unwrap()
            .build();
    }

    let test_fn = async |universe: Arc<Universe>| {
        let mut entities = universe.query::<&Position>().await.into_entities();
        entities.shuffle(&mut rand::rng());
        for eid in entities {
            let mut pos = universe
                .get_mut::<Position>(eid)
                .await
                .expect("Failed to get position");
            pos.x = rand::rng().random();
            pos.y = rand::rng().random();
        }
    };

    let mut join_set = tokio::task::JoinSet::new();
    for _ in 0..100 {
        let universe = universe.clone();
        join_set.spawn(async move {
            test_fn(universe.clone()).await;
        });
    }
    let threads_running = Arc::new(AtomicU16::new(100));
    for _ in 0..100 {
        let universe = universe.clone();
        let threads_running = threads_running.clone();
        std::thread::spawn(move || {
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                test_fn(universe.clone()).await;
                threads_running.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
            });
        });
    }
    join_set.join_all().await;
    while threads_running.load(std::sync::atomic::Ordering::Relaxed) > 0 {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}

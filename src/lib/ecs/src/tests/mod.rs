use crate::components::ComponentManager;
use crate::entities::EntityManager;
use crate::query::Query;
use std::collections::HashSet;
use std::iter::Iterator;

#[derive(Debug, Eq, Hash, PartialEq)]
#[expect(dead_code)]
struct Position {
    x: u32,
    y: u32,
}

unsafe impl Send for Position {}

#[derive(Debug, Eq, Hash, PartialEq)]
#[expect(dead_code)]
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

    while let Some((player, position)) = query.next().await {
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

    while let Some((eid, (player, position))) = query.next().await {
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
    let (eid, pos) = res.unwrap();
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

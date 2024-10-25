use std::thread;
use std::time::{Duration, Instant};
use crate::components::ComponentStorage;
use crate::entities::EntityManager;
use crate::query::Query;
use rayon::prelude::*;


#[derive(Debug)]
#[expect(dead_code)]
struct Position {
    x: u32,
    y: u32,
}

unsafe impl Send for Position {}

#[derive(Debug)]
#[expect(dead_code)]
struct Player {
    username: String,
}

unsafe impl Send for Player {}

#[test]
fn test_basic() {
    let entity_manager = EntityManager::new();
    let component_storage = ComponentStorage::new();

    for x in 0..10 {
        entity_manager
            .builder(&component_storage)
            .with(Position { x, y: x * 2 })
            .with(Player { username: format!("Player{}", x) })
            .build();
    }
    let query = Query::<(&Player, &mut Position)>::new(&component_storage);
    
    let start = Instant::now();
    let durations = dashmap::DashSet::new();
    ParallelIterator::for_each(query.into_par_iter(), |(_player, position)| {
        let sleep_duration = Duration::from_millis(100 * (position.x as u64));
        durations.insert(sleep_duration.as_millis());
        thread::sleep(sleep_duration);
    });
    
    let duration = start.elapsed();
    
    // Should be true, since we're running all branches in parallel, therefore, 
    // at-most it should take the time of the longest branch.
    // Since CI is pretty slow, we just check that it's less than the combined duration of all
    // the durations cos we can't rely on a specific speed
    
    assert!(duration.as_millis() < durations.iter().map(|x| *x).sum());
}
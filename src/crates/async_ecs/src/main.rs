use std::sync::Arc;
use std::thread;
use std::time::Duration;
use parking_lot::RwLock;
use crate::component::{ComponentStorage, Position, Velocity};
use crate::entity::EntityManager;

mod entity;
mod component;
mod helpers;
mod error;
mod tests;

fn main() {
    test_two_thread_component();
}

fn test_two_thread_component() {
    let entity_manager = Arc::new(RwLock::new(EntityManager::new()));
    let component_storage = Arc::new(ComponentStorage::new());

    // Create one entity and add initial components
    let entity = {
        let mut em = entity_manager.write();
        let e = em.create_entity();
        component_storage.insert(e, Position { x: 0.0, y: 0.0 });
        component_storage.insert(e, Velocity { x: 1.0, y: 1.0 });
        e
    };

    let em_modify = entity_manager.clone();
    let cs_modify = component_storage.clone();
    let modify_handle = thread::spawn(move || {
        for i in 0..50 {  // Modify 50 times
            {
                let em = em_modify.read();
                if let Some(position) = cs_modify.get::<Position>(entity) {
                    let mut position = position;
                    if let Some(velocity) = cs_modify.get::<Velocity>(entity) {
                        position.x += velocity.x;
                        position.y += velocity.y;
                    }
                }

                if let Some(velocity) = cs_modify.get::<Velocity>(entity) {
                    let mut velocity = velocity;
                    velocity.x += 0.1;
                    velocity.y += 0.1;
                }
            }
            thread::sleep(Duration::from_millis(100));  // Sleep to allow logging thread to run
        }
    });

    let em_log = entity_manager.clone();
    let cs_log = component_storage.clone();
    let log_handle = thread::spawn(move || {
        for _ in 0..6000 {  // Log 60 times (slightly more than modify thread iterations)
            {
                let em = em_log.read();
                if let Some(position) = cs_log.get::<Position>(entity) {
                    if let Some(velocity) = cs_log.get::<Velocity>(entity) {
                        println!(
                            "Entity state - Position: ({:.2}, {:.2}), Velocity: ({:.2}, {:.2})",
                            position.x, position.y, velocity.x, velocity.y
                        );
                    }
                }
            }
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Wait for both threads to complete
    modify_handle.join().unwrap();
    log_handle.join().unwrap();

    // Verify final state
    let em = entity_manager.read();
    if let Some(position) = component_storage.get::<Position>(entity) {
        if let Some(velocity) = component_storage.get::<Velocity>(entity) {
            println!("Final state - Position: {:?}, Velocity: {:?}", *position, *velocity);
        }
    }
}
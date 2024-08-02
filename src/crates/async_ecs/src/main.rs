
mod entity;
mod component;
mod helpers;
mod error;
mod query;
mod tests;

#[tokio::main]
async fn main() {
    /*test_two_thread_component();*/
    /*test_concurrent().await;*/
}
/*
#[allow(dead_code)]
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

    let cs_modify = component_storage.clone();
    let modify_handle = thread::spawn(move || {
        for _ in 0..50 {  // Modify 50 times
            {
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

    let cs_log = component_storage.clone();
    let log_handle = thread::spawn(move || {
        for _ in 0..6000 {  // Log 60 times (slightly more than modify thread iterations)
            {
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
    if let Some(position) = component_storage.get::<Position>(entity) {
        if let Some(velocity) = component_storage.get::<Velocity>(entity) {
            println!("Final state - Position: {:?}, Velocity: {:?}", *position, *velocity);
        }
    }
}


async fn test_concurrent() {
    println!("Testing concurrent");

    let (rw_em, component_storage) = GET_WORLD();
    let mut em = rw_em.write();
    let entity = em.create_entity();
    drop(em);
    let position = Position { x: 0.0, y: 0.0 };
    component_storage.insert(entity, position);

    // create a thread to hold the lock with a mutable read
    let write_handle = tokio::spawn(async move {
        let em = rw_em.read();
        let query = Query::<Position>::new(&em, &component_storage);
        for (_, mut position) in query.iter() {
            position.x += 1.0;

            println!("position: {:?}", *position);
            println!("Waiting for 1 second");
            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
            println!("Done waiting for 1 second");
        }
    });

    // create a thread to hold the lock with a mutable write
    let read_handle = tokio::spawn(async move {
        let em = rw_em.read();
        let query = Query::<Position>::new(&em, &component_storage);
        for (_, position) in query.iter() {
            // try reading the position
            println!("position: {:?}", *position);
        }
    });


    // wait for the threads to finish
    tokio::try_join!(write_handle/*, read_handle*/).unwrap();

    tokio::time::sleep(Duration::from_millis(2000)).await;
}*/
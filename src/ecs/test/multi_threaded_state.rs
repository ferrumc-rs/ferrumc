use std::sync::Arc;

use crate::ecs::world::World;

pub struct ServerState {
    pub world: Arc<World>,
}

impl ServerState {
    pub fn new(world: World) -> Self {
        ServerState { world: Arc::new(world) }
    }
}

#[tokio::test]
async fn test_multithreaded_state() {
    let world = World::new();
    let state = Arc::new(ServerState::new(world));

    // call the same thread 3 times
    for _ in 0..3 {
        let state = Arc::clone(&state);
        let thread = tokio::spawn(async move {
            let entity = state.world.create_entity().await.build();
            println!("created entity: {:?}", entity);
        });
        thread.await.unwrap();
    }
}
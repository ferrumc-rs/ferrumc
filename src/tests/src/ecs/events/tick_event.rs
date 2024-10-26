use ferrumc_events::infrastructure::Event;
use ferrumc_net::packets::outgoing::tick_event::TickEvent;

#[cfg(test)]
use std::sync::Arc;

use ferrumc_ecs::Universe;
use ferrumc_net::{server, ServerState};

async fn test_ticking(net_state: Arc<ServerState>) {
    // is game time saved in a file??
    let mut tick = 0;
    for _ in 0..1 {
        // TODO handle error
        let res = TickEvent::trigger(TickEvent::new(tick), net_state.clone()).await;

        if res.is_err() {
            panic!("error : {:?}", res);
        }
        tick += 1;
    }
}

#[tokio::test]
async fn tick_event() {
    let listener = server::create_server_listener()
        .await
        .expect("server failed to get created");

    let state = Arc::new(ServerState::new(Universe::new()));
    let handle = tokio::task::spawn(test_ticking(state.clone()));
    server::listen(state, listener)
        .await
        .expect("server failed to listen");
    assert!(handle.await.is_ok());
}

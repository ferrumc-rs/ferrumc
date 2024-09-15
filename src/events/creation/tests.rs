/*use crate::events::registry::{call_event, EventHandler};
use ferrumc_macros::{event_handler, EventHandler};


#[derive(EventHandler)]
struct Handler1;

impl EventHandler for Handler1 {
    type EventType = TestEvent;

    fn handle(&self, event: &mut Self::EventType) {
        println!("Handler 1 called with value: {}", event.value);
        event.value += 1;
    }
}

#[derive(EventHandler)]
#[event_handler(priority = "fastest")]
struct Handler2;

impl EventHandler for Handler2 {
    type EventType = TestEvent;

    fn handle(&self, event: &mut Self::EventType) {
        println!("Handler 2 called with value: {}", event.value);
        event.value += 1;
    }
}

*/
use crate::events::creation::registry::dispatch_event;
use ferrumc_macros::event_handler;
use std::sync::Arc;
use tokio::net::TcpListener;
use crate::create_state;
use crate::state::GlobalState;

struct TestEvent {
    value: i32,
}

#[event_handler(priority = "fastest")]
async fn handler(event: Arc<parking_lot::RwLock<TestEvent>>, _state: GlobalState) {
    let mut event = event.write();
    println!("Handler called with value: {}", event.value);
    event.value += i32::MAX;
}

#[event_handler]
async fn handler2(event: Arc<parking_lot::RwLock<TestEvent>>, _state: GlobalState) {
    let mut event = event.write();
    println!("Handler 2 called with value: {}", event.value);
    event.value -= i32::MAX;
}

#[tokio::test]
async fn test_if_this_even_compiles() -> anyhow::Result<()> {
    let state = create_state(TcpListener::bind("0.0.0.0:9009").await?).await?;
    let some_event = TestEvent {
        value: 0,
    };

    let some_event = Arc::new(parking_lot::RwLock::new(some_event));
    dispatch_event(Arc::clone(&some_event), state).await;

    println!("Final value: {}", some_event.read().value);


    Ok(())
}










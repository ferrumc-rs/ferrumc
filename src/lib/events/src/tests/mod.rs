use std::sync::Arc;
use parking_lot::RwLock;

use crate::infrastructure::{get_event_listeners, insert_into_events, Event};




// Final API example:
//
// #[event_handler]
// pub fn handle_event(event: Arc<Rwlock<SomeEvent>>) {
//     println!("Event: {:?}", event);
// }





#[derive(Debug)]
struct SomeEvent {
    data: i32,
}

impl Event for SomeEvent {
    fn name() -> &'static str {
        "SomeEvent"
    }
}

#[tokio::test]
async fn test_something() {
    let event_data = Arc::new(RwLock::new(SomeEvent {
        data: 0
    }));

    for listener in get_event_listeners::<SomeEvent>() {
        listener(Arc::clone(&event_data)).await;
    }
}


#[ctor::ctor]
fn __register_some_event_listener() {
    insert_into_events(|ev: Arc<RwLock<SomeEvent>>| Box::pin(some_event_listener(ev)), 0);
}

async fn some_event_listener(event: Arc<RwLock<SomeEvent>>) {
    let mut ev = event.write();
    ev.data = 10;
    println!("I set the event's data to 10");
}

#[ctor::ctor]
fn __register_some_event_listener2() {
    insert_into_events(|ev: Arc<RwLock<SomeEvent>>| Box::pin(some_event_listener2(ev)), 255);
}

async fn some_event_listener2(event: Arc<RwLock<SomeEvent>>) {
    let ev = event.read();
    println!("I read the event's data: {}", ev.data);
}
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
    let listeners = get_event_listeners::<SomeEvent>();

    let event_data = Arc::new(RwLock::new(SomeEvent {
        data: 0
    }));

    for listener in listeners {
        listener(Arc::clone(&event_data)).await;
    }
}



#[ctor::ctor]
fn register_some_event() {
    insert_into_events(|ev: Arc<RwLock<SomeEvent>>| Box::pin(async move {
        let mut ev = ev.write();
        ev.data = 10;
        println!("I set the event's data to 10");
    }));
}

#[ctor::ctor]
fn register_some_event2() {
    insert_into_events(|ev: Arc<RwLock<SomeEvent>>| Box::pin(async move {
        let ev = ev.read();
        println!("I read the event's data: {}", ev.data);
    }));
}
use crate::events::registry::{call_event, EventHandler};
use ferrumc_macros::{event_handler, EventHandler};

struct TestEvent {
    value: i32,
}
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

#[test]
fn test_if_this_even_compiles() {
    let mut some_event = TestEvent {
        value: 0,
    };

    call_event(&mut some_event);

    println!("Final value: {}", some_event.value);
}

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
use ferrumc_macros::event_handler;
use crate::events::registry::call_event;

struct TestEvent {
    value: i32,
}

#[event_handler(priority = "fastest")]
fn handler(event: &mut TestEvent) {
    println!("Handler 1 called with value: {}", event.value);
    event.value += 999;
}

#[event_handler]
fn handler2(event: &TestEvent) {
    println!("Handler 2 called with value: {}", event.value);
}

#[test]
fn test_if_this_even_compiles() {
    let mut some_event = TestEvent {
        value: 0,
    };

    call_event(&mut some_event);

    println!("Final value: {}", some_event.value);
}










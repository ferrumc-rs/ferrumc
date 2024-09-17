// Security or something like that
#![forbid(unsafe_code)]

use std::sync::Arc;
use parking_lot::RwLock;
use tracing::info;
use ferrumc_events::infrastructure::{get_event_listeners, Event};
use ferrumc_macros::event_handler;

#[tokio::main]
async fn main() {
    ferrumc_logging::init_logging();
    
    println!("good day to ya. enjoy your time with ferrumc!");

    let some_event = Arc::new(RwLock::new( SomeEvent {
        some_data: 42,
    }));

    for listener in get_event_listeners::<SomeEvent>() {
        listener(some_event.clone()).await;
    }

}


#[derive(Debug)]
struct SomeEvent {
    pub some_data: i32,
}

impl Event for SomeEvent {
    fn name() -> &'static str {
        "SomeEvent"
    }
}

#[event_handler(priority = "fastest")]
async fn some_test_event(some_event: Arc<RwLock<SomeEvent>>) {
    info!("some_test_event: {:?}", some_event.read().some_data);
}

#[event_handler(priority = "slowest")]
async fn some_test_event2(some_event: Arc<RwLock<SomeEvent>>) {
    info!("some_test_event 2: {:?}", some_event.read().some_data);
}
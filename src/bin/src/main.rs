// Security or something like that
#![forbid(unsafe_code)]

use ferrumc_events::infrastructure::Event;
use ferrumc_macros::event_handler;

#[tokio::main]
async fn main() {
    ferrumc_logging::init_logging();

    println!("good day to ya. enjoy your time with ferrumc!");
    println!("beep boop beep boop...");

    let some_event = SomeEvent { data: 42 };

    SomeEvent::trigger(some_event)
        .await
        .expect("Failed to trigger SomeEvent");
}

#[derive(Debug)]
struct SomeEvent {
    pub data: i32,
}

#[derive(Debug)]
pub enum SomeEventError {}

impl Event for SomeEvent {
    type Data = Self;
    type Error = SomeEventError;

    fn name() -> &'static str {
        "SomeEvent"
    }
}

#[event_handler(priority = "fastest")]
async fn some_event_listener(mut event: SomeEvent) -> Result<SomeEvent, SomeEventError> {
    event.data = 10;
    println!("I set the event's data to 10");
    Ok(event)
}

#[event_handler(priority = "slowest")]
async fn some_event_listener2(event: SomeEvent) -> Result<SomeEvent, SomeEventError> {
    println!("I read the event's data: {}", event.data);
    Ok(event)
}

#[event_handler]
async fn some_event_listener3(event: SomeEvent) -> Result<SomeEvent, SomeEventError> {
    println!("im just here to listen to the event");
    Ok(event)
}

use crate::infrastructure::Event;

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

#[derive(Debug)]
pub enum SomeEventError {}

impl Event for SomeEvent {
    type Data = Self;

    type Error = SomeEventError;

    fn name() -> &'static str {
        "SomeEvent"
    }
}

#[tokio::test]
async fn test_something() {
    let event_data = SomeEvent { data: 0 };

    SomeEvent::trigger(event_data).await.unwrap();
}

// #[ctor::ctor]
// fn __register_some_event_listener() {
//     SomeEvent::register(Box::pin(some_event_listener(ev)), priority)
//     insert_into_events(|ev: Arc<RwLock<SomeEvent>>| , 0);
// }
//
// async fn some_event_listener(event: Arc<RwLock<SomeEvent>>) {
//     let mut ev = event.write();
//     ev.data = 10;
//     println!("I set the event's data to 10");
// }
//
// #[ctor::ctor]
// fn __register_some_event_listener2() {
//     insert_into_events(|ev: Arc<RwLock<SomeEvent>>| Box::pin(some_event_listener2(ev)), 255);
// }
//
// async fn some_event_listener2(event: Arc<RwLock<SomeEvent>>) {
//     let ev = event.read();
//     println!("I read the event's data: {}", ev.data);
// }

#[ctor::ctor]
fn __register_some_event_listener() {
    SomeEvent::register(|ev: SomeEvent| Box::pin(some_event_listener(ev)), 0);
}

async fn some_event_listener(mut event: SomeEvent) -> Result<SomeEvent, SomeEventError> {
    event.data = 10;
    println!("I set the event's data to 10");
    Ok(event)
}

#[ctor::ctor]
fn __register_some_event_listener2() {
    SomeEvent::register(|ev: SomeEvent| Box::pin(some_event_listener2(ev)), 255);
}

async fn some_event_listener2(event: SomeEvent) -> Result<SomeEvent, SomeEventError> {
    println!("I read the event's data: {}", event.data);
    Ok(event)
}

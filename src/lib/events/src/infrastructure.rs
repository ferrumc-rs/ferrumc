use std::any::Any;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, LazyLock};
use parking_lot::RwLock;
// API design
// #[event_handler]
// pub fn handle_event(event: Arc<Rwlock<SomeEvent>>) {
//     println!("Event: {:?}", event);
// }

pub trait Event: Send + Sync + 'static {
    fn name(&self) -> &'static str;
}

type ThreadSafeRwLock<E> = Arc<RwLock<E>>;

type AsyncEventListener<E> = fn(E) -> Pin<Box<dyn Future<Output = ()> + Send>>;


static EVENTS: LazyLock<RwLock<Vec<Box<dyn Any + Send + Sync>>>> = LazyLock::new(|| RwLock::new(Vec::new()));

pub fn insert_into_events<E: Send + Sync + 'static>(ev: AsyncEventListener<ThreadSafeRwLock<E>>) {
    EVENTS.write().push(Box::new(ev));
}

pub fn get_event_listeners<E: 'static>() -> Vec<AsyncEventListener<ThreadSafeRwLock<E>>> {
    EVENTS.read()
        .iter()
        .filter_map(|boxed| boxed.downcast_ref::<AsyncEventListener<ThreadSafeRwLock<E>>>().cloned())
        .collect()
}
use std::any::Any;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, LazyLock};
use hashbrown::HashMap;
use parking_lot::RwLock;

pub trait Event: Send + Sync + 'static {
    fn name() -> &'static str;
}

type ThreadSafeRwLock<E> = Arc<RwLock<E>>;

type AsyncEventListener<E> = fn(E) -> Pin<Box<dyn Future<Output = ()> + Send>>;


/// This is a map of event names to event listeners
/// e.g.
/// {
///    "SomeEvent": [listener1, listener2],
///   "AnotherEvent": [listener3, listener4]
/// }
static EVENTS: LazyLock<RwLock<HashMap<&'static str, Vec<Box<dyn Any + Send + Sync>>>>> = LazyLock::new(|| RwLock::new(HashMap::new()));

pub fn insert_into_events<E: Event>(ev: AsyncEventListener<ThreadSafeRwLock<E>>) {
    let name = E::name();
    EVENTS.write().entry(name).or_insert_with(Vec::new).push(Box::new(ev));
}

pub fn get_event_listeners<E: Event>() -> Vec<AsyncEventListener<ThreadSafeRwLock<E>>> {
    /*EVENTS.read()
        .iter()
        .filter_map(|boxed| boxed.downcast_ref::<AsyncEventListener<ThreadSafeRwLock<E>>>().cloned())
        .collect()*/
    
    let name = E::name();
    EVENTS.write()
        .get(name)
        .unwrap_or(&Vec::new())
        .iter()
        .filter_map(|boxed| boxed.downcast_ref::<AsyncEventListener<ThreadSafeRwLock<E>>>().cloned())
        .collect()
}
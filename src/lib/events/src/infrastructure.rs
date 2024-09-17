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

type AsyncEventListenerFn<E> = fn(E) -> Pin<Box<dyn Future<Output = ()> + Send>>;

pub struct EventListener<E> {
    listener: AsyncEventListenerFn<E>,
    // 0 ~ 255, 0 being run first, 255 being run last
    priority: u8
}

/// This is a map of event names to event listeners
/// e.g.
/// {
///    "SomeEvent": [listener1, listener2],
///   "AnotherEvent": [listener3, listener4]
/// }
static EVENTS: LazyLock<RwLock<HashMap<&'static str, Vec<Box<dyn Any + Send + Sync>>>>> = LazyLock::new(|| RwLock::new(HashMap::new()));

pub fn insert_into_events<E: Event>(ev: AsyncEventListenerFn<ThreadSafeRwLock<E>>, priority: u8) {
    let name = E::name();
    let listener = EventListener {
        listener: ev,
        priority
    };
    EVENTS.write().entry(name).or_insert_with(Vec::new).push(Box::new(listener));
}

pub fn get_event_listeners<E: Event>() -> Vec<AsyncEventListenerFn<ThreadSafeRwLock<E>>> {
    EVENTS
        .read()
        .get(E::name())
        .map(|events| {
            let mut listeners = events.iter()
                .filter_map(|boxed| boxed.downcast_ref::<EventListener<ThreadSafeRwLock<E>>>())
                .collect::<Vec<_>>();

            listeners.sort_by_key(|listener| listener.priority);
            listeners.into_iter()
                .map(|listener| listener.listener)
                .collect()
        }).unwrap_or_default()
}
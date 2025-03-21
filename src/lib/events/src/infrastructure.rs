use std::{any::Any, future::Future, pin::Pin, sync::LazyLock};

use dashmap::DashMap;

/// A Lazily initialized HashMap wrapped in a ShardedLock optimized for reads.
type LazyRwListenerMap<K, V> = LazyLock<DashMap<K, V>>;

type SyncEventListener<E> =
    fn(<E as Event>::Data, <E as Event>::State) -> Result<<E as Event>::Data, <E as Event>::Error>;

/// This is the global map of event listeners.
/// It is lazily initialized at runtime.
///
/// It links an event string name to its set of listeners
/// e.g.
/// {
///    "SomeEvent": [listener1, listener2],
///   "AnotherEvent": [listener3, listener4]
/// }
static EVENTS_LISTENERS: LazyRwListenerMap<&'static str, Vec<Box<dyn Any + Send + Sync>>> =
    LazyLock::new(DashMap::new);

/// An event listener structure that contains a pointer to an asynchronous event listener
/// and its priority of execution.
pub struct EventListener<E: Event> {
    /// An asynchronous event listener which returns a result with a potentially modified data or error.
    listener: SyncEventListener<E>,
    /// Priority of this listener
    priority: u8,
}

impl<E: Event> EventListener<E> {
    /// Trampoline function to convert from Box<Self> to Box<dyn ...>
    pub fn to_dyn(self: Box<Self>) -> Box<dyn Any + Send + Sync> {
        self
    }
}

/// Trait that permit to access priority through the help of a function
pub trait Priority {
    fn priority(&self) -> u8;
}

impl<E: Event> Priority for EventListener<E> {
    fn priority(&self) -> u8 {
        self.priority
    }
}

pub trait Event: Sized + Send + Sync + 'static {
    /// Event data structure
    type Data: Send + Sync;

    /// State
    type State: Send + Sync + Clone;

    /// Event specific error
    type Error: std::fmt::Debug + Send;

    /// Stringified name of the event
    fn name() -> &'static str;

    /// Trigger an event execution
    ///
    /// This method will pass the data to the listener with the highest priority which
    /// will give its result to the next one with a lesser priority and so on.
    ///
    /// Returns `Ok(())` if the execution succeeded. `Err(EventsError)` ifa listener failed.
    fn trigger(event: Self::Data, state: Self::State) -> Result<(), Self::Error> {
        #[cfg(debug_assertions)]
        let start = std::time::Instant::now();

        let listeners = EVENTS_LISTENERS
            .get(Self::name())
            .expect("Failed to find event listeners. Impossible;");
        // Convert listeners iterator into Stream

        #[expect(clippy::manual_try_fold)]
        listeners
            .iter()
            // TODO: Remove this since it's not possible to have a wrong type in the map of the event???
            // Maybe some speedup?
            // Filter only listeners we can downcast into the correct type
            .filter_map(|dyn_list| dyn_list.downcast_ref::<EventListener<Self>>())
            // Trigger listeners in a row
            .fold(Ok(event), |intercepted, listener| {
                let state = state.clone();
                match intercepted {
                    Err(_) => intercepted,
                    Ok(intercepted) => (listener.listener)(intercepted, state),
                }
            })
            .expect("Failed to trigger event listeners");
        #[cfg(debug_assertions)]
        tracing::trace!("Event {} took {:?}", Self::name(), start.elapsed());

        Ok(())
    }

    /// Register a new event listener for this event
    fn register(listener: SyncEventListener<Self>, priority: u8) {
        // Create the event listener structure
        let listener = EventListener::<Self> { listener, priority };

        // Write guard the event listeners global map
        let map = &EVENTS_LISTENERS;

        // Remove listeners to sort them
        let event_listeners = map.remove(Self::name()).unwrap_or_default().1;

        // Downcast them to access their priority field
        let mut sorted_listeners = event_listeners
            .into_iter()
            .filter_map(|boxed| boxed.downcast::<EventListener<Self>>().ok())
            .collect::<Vec<_>>();

        // Append our new listener
        sorted_listeners.push(Box::new(listener));

        // Sort them then recollect them.
        sorted_listeners.sort_by_key(|listener| listener.priority);
        let event_listeners: Vec<Box<dyn Any + Send + Sync>> = sorted_listeners
            .into_iter()
            .map(|listener| listener.to_dyn())
            .collect::<Vec<_>>();

        // Reinsert sorted listeners into global map
        map.insert(Self::name(), event_listeners);
    }
}

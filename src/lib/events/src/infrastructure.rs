use std::{any::Any, future::Future, pin::Pin, sync::LazyLock};

use dashmap::DashMap;
use futures::{stream, StreamExt};

/// A Lazily initialized HashMap wrapped in a ShardedLock optimized for reads.
type LazyRwListenerMap<K, V> = LazyLock<DashMap<K, V>>;

pub type AsyncEventListener<E> = fn(
    <E as Event>::Data,
    <E as Event>::State,
) -> Pin<
    Box<dyn Future<Output = Result<<E as Event>::Data, <E as Event>::Error>> + Send>,
>;

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
    listener: AsyncEventListener<E>,
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

#[allow(async_fn_in_trait)]
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
    async fn trigger(event: Self::Data, state: Self::State) -> Result<(), Self::Error> {
        let listeners = EVENTS_LISTENERS
            .get(Self::name())
            .expect("Failed to find event listeners. Impossible;");

        // Convert listeners iterator into Stream
        stream::iter(listeners.iter())
            // TODO: Remove this since it's not possible to have a wrong type in the map of the event???
            // Maybe some speedup?
            // Filter only listeners we can downcast into the correct type
            .filter_map(|dyn_list| async { dyn_list.downcast_ref::<EventListener<Self>>() })
            // Trigger listeners in a row
            .fold(Ok(event), |intercepted, listener| {
                let state = state.clone();
                async move {
                    if intercepted.is_err() {
                        intercepted
                    } else {
                        (listener.listener)(intercepted.unwrap(), state).await
                    }
                }
            })
            .await?;

        Ok(())
    }

    /*/// Trigger the execution of an event with concurrency support
        ///
        /// If the event structure supports cloning. This method can be used to execute
        /// listeners of the same priority concurrently (using tokio::task). This imply a
        /// cloning cost at each listener execution. See `Event::trigger` for a more
        /// efficient but more linear approach.
        ///
        /// # Mutability policy
        ///
        /// The listeners having the same priority being runned concurrently, there are no
        /// guarantees in the order of mutation of the event data.
        ///
        /// It is recommended to ensure listeners of the same priority exclusively update fields
        /// in the event data that are untouched by other listeners of the same group.
        async fn trigger_concurrently(event: Self::Data) -> Result<(), Self::Error>
        where
            Self::Data: Clone,
        {
            let read_guard = &EVENTS_LISTENERS;
            let listeners = read_guard.get(Self::name()).unwrap();

            // Convert listeners iterator into Stream
            let mut stream = stream::iter(listeners.iter());

            let mut priority_join_set = Vec::new();
            let mut current_priority = 0;

            while let Some(Some(listener)) = stream
                .next()
                .await
                .map(|l| l.downcast_ref::<EventListener<Self>>())
            {
                if listener.priority == current_priority {
                    priority_join_set.push(tokio::spawn((listener.listener)(event.clone())));
                } else {
                    // Await over all listeners launched
                    let joined = future::join_all(priority_join_set.iter_mut()).await;

                    // If one listener fail we return the first error
                    if let Some(err) = joined
                        .into_iter()
                        .filter_map(|res| res.expect("No task should ever panic. Impossible;").err())
                        .next()
                    {
                        return Err(err);
                    }

                    // Update priority to the new listener(s)
                    current_priority = listener.priority;
                    priority_join_set.push(tokio::spawn((listener.listener)(event.clone())));
                }
            }

            Ok(())
        }
    */
    /// Register a a new event listener for this event
    fn register(listener: AsyncEventListener<Self>, priority: u8) {
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

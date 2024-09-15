use std::any::Any;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use crate::state::GlobalState;

pub trait EventHandlerWrapper: Send + Sync + 'static {
    fn handle(&self, event: Arc<dyn Any + Send + Sync>, state: GlobalState) -> Pin<Box<dyn Future<Output=()> + Send + '_>>;
    fn event_type_id(&self) -> std::any::TypeId;
}


#[derive(Debug)]
pub struct EventPriority(u8);

impl EventPriority {
    pub const fn default() -> Self {
        Self(128)
    }
}

impl From<u8> for EventPriority {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

pub struct EventContainer {
    /// before <-----> after (lower = runs first)
    /// 0 <-----> 255
    /// Default is 128 (not too high, not too low)
    priority: EventPriority,
    pub(crate) handler: &'static dyn EventHandlerWrapper,
}

impl EventContainer {
    pub const fn new(priority: u8, handler: &'static dyn EventHandlerWrapper) -> Self {
        Self {
            priority: EventPriority(priority),
            handler,
        }
    }
}


pub fn get_event_handlers() -> Vec<&'static EventContainer> {
    inventory::iter::<EventContainer>
        .into_iter()
        .collect()
}

pub fn get_event_handlers_for<T: 'static>() -> Vec<&'static EventContainer> {
    let mut handlers = get_event_handlers()
        .into_iter()
        .filter(|h| h.handler.event_type_id() == std::any::TypeId::of::<T>())
        .collect::<Vec<_>>();

    handlers.sort_by_key(|h| h.priority.0);

    handlers
}

pub async fn dispatch_event<T: 'static + Any + Send + Sync>(event: Arc<T>, state: GlobalState) {
    let handlers = get_event_handlers_for::<T>();

    let event = event as Arc<dyn Any + Send + Sync>;


    for handler in handlers.iter() {
        handler.handler.handle(Arc::clone(&event), state.clone()).await;
    }
}

pub struct FunctionEventHandler<E: Send + Sync> {
    // pub handler: fn(Arc<E>) -> Pin<Box<dyn Future<Output=()> + Send + '_>>,
    // pub handler: Arc<dyn Fn(Arc<E>) -> Pin<Box<dyn Future<Output=()> + Send + 'static>>>,
//     fn(parking_lot::lock_api::RwLock<parking_lot::RawRwLock, TestEvent>) -> impl futures::Future<Output = ()> {handler}
    pub handler: fn(Arc<E>, GlobalState) -> Pin<Box<dyn Future<Output=()> + Send + 'static>>,
}

impl<E: 'static + Any + Send + Sync> EventHandlerWrapper for FunctionEventHandler<E> {
    fn handle(&self, event: Arc<dyn Any + Send + Sync>, state: GlobalState) -> Pin<Box<dyn Future<Output=()> + Send + '_>> {
        Box::pin(async move {
            let event = Arc::downcast::<E>(event).expect("wrong type for event");
            (self.handler)(event, state).await;
        })
    }

    fn event_type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<E>()
    }
}


inventory::collect!(EventContainer);
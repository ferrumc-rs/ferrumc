use std::any::Any;

pub trait EventHandler: Send + Sync + 'static {
    type EventType: 'static;

    fn handle(&self, event: &mut Self::EventType);
}

pub trait EventHandlerWrapper: Send + Sync + 'static {
    fn handle(&self, event: &mut dyn Any);
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
    handler: &'static dyn EventHandlerWrapper,
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

    handlers.sort_by(|a, b| a.priority.0.cmp(&b.priority.0));

    handlers
}

pub fn call_event<T: 'static>(event: &mut T) {
    let handlers = get_event_handlers_for::<T>();

    for handler in handlers.iter() {
        handler.handler.handle(event as &mut dyn Any);
    }
}

impl<T: EventHandler> EventHandlerWrapper for T {
    fn handle(&self, event: &mut dyn Any) {
        let Some(event) = event.downcast_mut::<T::EventType>() else {
            println!("Invalid event type received for handler: {:?}", std::any::type_name::<T::EventType>());
            return;
        };
        self.handle(event);
    }

    fn event_type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<T::EventType>()
    }
}


pub struct FunctionEventHandlerMut<E> {
    pub handler: fn(&mut E),
}

pub struct FunctionEventHandlerRef<E> {
    pub handler: fn(&E),
}

impl<E: 'static> EventHandlerWrapper for FunctionEventHandlerMut<E> {
    fn handle(&self, event: &mut dyn Any) {
        let Some(event) = event.downcast_mut::<E>() else {
            println!("Invalid event type received for handler: {:?}", std::any::type_name::<E>());
            return;
        };
        (self.handler)(event);
    }

    fn event_type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<E>()
    }
}

impl<E: 'static> EventHandlerWrapper for FunctionEventHandlerRef<E> {
    fn handle(&self, event: &mut dyn Any) {
        if let Some(event) = (&*event).downcast_ref::<E>() {
            (self.handler)(event);
        } else {
            println!("Invalid event type received for handler");
        }
    }

    fn event_type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<E>()
    }
}


inventory::collect!(EventContainer);
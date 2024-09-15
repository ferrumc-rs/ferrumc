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

pub struct EventContainer {
    /// before <-----> after (lower = runs first)
    /// 0 <-----> 255
    /// Default is 128 (not too high, not too low)
    priority: EventPriority,
    handler: &'static dyn EventHandlerWrapper,
}

impl EventContainer {
    pub const fn new(priority: EventPriority, handler: &'static dyn EventHandlerWrapper) -> Self {
        Self {
            priority,
            handler,
        }
    }
}


pub fn get_event_handlers() -> Vec<&'static EventContainer> {
    inventory::iter::<EventContainer>
        .into_iter()
        .collect()
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

mod test {
    use crate::events::registry::{get_event_handlers, EventHandler};
    use ferrumc_macros::EventHandler;
    use std::any::Any;

    struct TestEvent {
        value: i32,
    }
    #[derive(EventHandler)]
    struct Handler1;

    impl EventHandler for Handler1 {
        type EventType = TestEvent;

        fn handle(&self, event: &mut Self::EventType) {
            println!("Handler 1 called with value: {}", event.value);
            event.value += 1;
        }
    }

    #[derive(EventHandler)]
    struct Handler2;

    impl EventHandler for Handler2 {
        type EventType = TestEvent;

        fn handle(&self, event: &mut Self::EventType) {
            println!("Handler 2 called with value: {}", event.value);
            event.value += 1;
        }
    }

    #[test]
    fn test_if_this_even_compiles() {
        let mut handlers = get_event_handlers();

        let mut some_event = TestEvent {
            value: 0,
        };

        for handler in handlers.iter_mut().rev() {
            println!("Handler with priority: {:?}", handler.priority);
            handler.handler.handle(&mut some_event as &mut dyn Any);
        }

        println!("Final value: {}", some_event.value);
    }
}


inventory::collect!(EventContainer);
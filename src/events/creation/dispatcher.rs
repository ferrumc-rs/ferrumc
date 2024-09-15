use std::any::Any;
use std::sync::Arc;
use crate::events::creation::registry::{dispatch_event};
use crate::state::GlobalState;

pub struct EventDispatcher;


impl EventDispatcher {
    pub fn new() -> Self {
        Self
    }
    pub async fn dispatch_event<T: 'static + Any + Send + Sync>(&self, event: T, state: GlobalState) {
        let event = Arc::new(event);
        dispatch_event::<T>(event, state).await;
    }
}
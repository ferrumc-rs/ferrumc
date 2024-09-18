use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum EventsError {
    #[error("FerrumC was unable to register the following event `{event_name}` for the following reasons: {error}")]
    UnableToRegister {
        event_name: &'static str,
        error: String,
    },
    #[error("A listener failed")]
    ListenerFailed,
}

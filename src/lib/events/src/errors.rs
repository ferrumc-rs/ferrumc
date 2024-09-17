use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum EventsError {
    #[error("Something failed lol")]
    SomeError,
}
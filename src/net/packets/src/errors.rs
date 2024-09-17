use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum NetPacketError {
    #[error("Something failed lol")]
    SomeError,
}
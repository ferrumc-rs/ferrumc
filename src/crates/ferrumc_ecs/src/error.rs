use crate::world::Entity;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic {0}")]
    DeallocationError(DeallocationErrorType),
}

#[derive(thiserror::Error, Debug)]
pub enum DeallocationErrorType {
    #[error("Entity {0} not found")]
    EntityNotFound(Entity),
    #[error("Entity {0} has a different generation")]
    InvalidGeneration(Entity),
}


impl Into<ferrumc_utils::error::Error> for Error {
    fn into(self) -> ferrumc_utils::error::Error {
        ferrumc_utils::error::Error::Other(Box::new(self))
    }
}
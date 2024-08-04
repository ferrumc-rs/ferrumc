#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Entity {0} not found")]
    EntityNotFound(usize),
    #[error("Component not found")]
    ComponentNotFound,
    #[error("Couldn't remove component since it's locked")]
    ComponentLocked,
}
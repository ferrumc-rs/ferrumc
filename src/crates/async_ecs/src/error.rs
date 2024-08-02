#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Entity {0} not found")]
    EntityNotFound(u32),
    #[error("Component not found")]
    ComponentNotFound,
}
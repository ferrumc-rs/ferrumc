use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum ECSError {
    #[error("Component not found")]
    ComponentNotFound,
    #[error("Component is locked")]
    ComponentLocked,
}

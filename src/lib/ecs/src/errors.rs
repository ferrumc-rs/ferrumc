use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum ECSError {
    #[error("Component not found")]
    ComponentNotFound,
    #[error("Component is locked")]
    ComponentLocked,
    #[error("Component type not found")]
    ComponentTypeNotFound,
    #[error("Component retrieval error")]
    ComponentRetrievalError,
    #[error("Component removal error")]
    ComponentRemovalError,
}

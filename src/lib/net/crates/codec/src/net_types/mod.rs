pub mod var_int;

#[derive(Debug, thiserror::Error)]
pub enum NetTypesError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Invalid VarInt")]
    InvalidVarInt,
    #[error("I couldn't convert the value into a valid i32")]
    InvalidInputI32,
}
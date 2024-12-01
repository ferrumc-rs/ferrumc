pub mod var_int;
pub mod length_prefixed_vec;
pub mod network_position;
pub mod bitset;

#[derive(Debug, thiserror::Error)]
pub enum NetTypesError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Invalid VarInt")]
    InvalidVarInt,
    #[error("I couldn't convert the value into a valid i32")]
    InvalidInputI32,
}
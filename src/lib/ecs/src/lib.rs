pub mod errors;

pub mod components;
pub mod entities;
pub mod query;

#[cfg(test)]
mod tests;
pub type ECSResult<T> = Result<T, errors::ECSError>;
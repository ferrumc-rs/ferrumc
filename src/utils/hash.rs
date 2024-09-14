use std::hash::{Hash, Hasher};

/// A simple function to hash any type that implements Hash
///
/// Basically just a wrapper around the regular hashing method, so you don't have to have the hasher
/// variable in scope
///
/// # Example
/// ```ignore
/// let hashed: u64 = hash("hello");
/// ```
///
#[inline]
pub fn hash(input: impl Hash) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    input.hash(&mut hasher);
    hasher.finish()
}

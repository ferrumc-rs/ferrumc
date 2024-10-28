use std::hash::{Hash, Hasher};
use fnv::FnvHasher;

/// ### DO NOT USE THIS FOR SECURITY PURPOSES
/// This is a very simple hashing function that is not secure at all. It is only meant to be used
/// for hashing data in memory for quick lookups. It is not meant to be used for hashing passwords
/// or any other sensitive data.
pub fn hash<T: Hash>(item: T) -> u64 {
    let mut hasher = FnvHasher::default();
    item.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fnv_consistency() {
        let data = "consistent_hash";
        let hash1 = hash(data);
        let hash2 = hash(data);

        assert_eq!(hash1, hash2, "FNV should produce consistent results for the same input");
    }

    #[test]
    fn test_fnv_diff_input() {
        let data1 = "hello_world";
        let data2 = "goodbye_world";
        let hash1 = hash(data1);
        let hash2 = hash(data2);

        assert_ne!(hash1, hash2, "FNV should produce different results for different inputs");
    }
    
    #[test]
    fn test_specific_output() {
        let data = "hello_world";
        let hash = hash(data);

        assert_eq!(hash, 0x768aff4672817d95, "FNV should produce a specific output for a specific input");
    }
}
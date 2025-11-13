use once_cell::sync::Lazy;
use simd_json::OwnedValue;
use simd_json::derived::ValueObjectAccess;
use simd_json::prelude::ValueArrayAccess;

// Parse once at startup
static REGISTRY_BYTES: &[u8] = include_bytes!("../../../../assets/data/registries.json");

static LOADED_REGISTRY: Lazy<OwnedValue> = Lazy::new(|| {
    // simd-json mutates the buffer during parsing → needs a Vec<u8>
    let mut buf = REGISTRY_BYTES.to_vec();
    simd_json::to_owned_value(&mut buf).expect("parse registries.json")
});

/// Looks up a value in the loaded JSON registry by a given path.
///
/// # Arguments
/// * `path` - A string slice representing the path to the desired value in the JSON structure.
///   Path segments are separated by `/`. Numeric segments are treated as array indices.
///
/// # Returns
/// * `Option<&OwnedValue>` - Returns a reference to the value if found, or `None` if the path does not exist.
///
/// # Examples
/// ```
/// # use simd_json::prelude::{TypedScalarValue, ValueAsScalar};
/// # use ferrumc_registry::lookup;
/// let value = lookup("minecraft:item/entries/minecraft:apple/protocol_id");
/// assert!(value.is_some());
/// assert!(value.unwrap().is_u64()); // check if the value is a number
/// assert_eq!(value.unwrap().as_u64().unwrap(), 840);
/// ```
pub fn lookup(path: &str) -> Option<&OwnedValue> {
    let mut cur: &OwnedValue = &LOADED_REGISTRY;

    if path.is_empty() {
        return Some(cur); // return the root if path is empty
    }

    for seg in path.split('/') {
        // allow numeric segments as array indices
        if let Ok(idx) = seg.parse::<usize>() {
            cur = cur.get_idx(idx)?; // works if current is an array
        } else {
            cur = cur.get(seg)?; // works if current is an object
        }
    }
    Some(cur)
}

/// Looks up a value in the loaded JSON registry by a given path and returns an owned clone.
///
/// # Arguments
/// * `path` - A string slice representing the path to the desired value in the JSON structure.
///
/// # Returns
/// * `Option<OwnedValue>` - Returns an owned clone of the value if found, or `None` if the path does not exist.
///
/// # Examples
/// ```
/// # use simd_json::prelude::{TypedScalarValue, ValueAsScalar};
/// # use ferrumc_registry::lookup_owned;
/// let value = lookup_owned("minecraft:item/entries/minecraft:apple/protocol_id");
/// assert_eq!(value.unwrap().as_u64().unwrap(), 840);
/// ```
pub fn lookup_owned(path: &str) -> Option<OwnedValue> {
    lookup(path).cloned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use simd_json::prelude::ValueAsScalar;
    #[test]
    fn test_lookup() {
        let value = lookup("minecraft:item/entries/minecraft:apple/protocol_id");
        assert!(value.is_some());
        let value = value.unwrap();
        let numeric_value = value.as_u64();
        assert!(numeric_value.is_some());
        assert_eq!(numeric_value.unwrap(), 840);
    }

    #[test]
    fn test_lookup_owned() {
        let value = lookup_owned("minecraft:item/entries/minecraft:apple/protocol_id");
        assert!(value.is_some());
        let value = value.unwrap();
        let numeric_value = value.as_u64();
        assert!(numeric_value.is_some());
        assert_eq!(numeric_value.unwrap(), 840);
    }

    #[test]
    fn test_lookup_non_existent() {
        let value = lookup("minecraft:item/entries/minecraft:non_existent/protocol_id");
        assert!(value.is_none());
    }

    #[test]
    fn test_lookup_empty_path() {
        // Edge case: empty path should return the root
        let value = lookup("");
        assert!(value.is_some());
        assert_eq!(value.unwrap(), &*LOADED_REGISTRY);
    }
}

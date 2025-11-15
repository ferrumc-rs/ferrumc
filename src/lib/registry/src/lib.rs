use once_cell::sync::Lazy;
use simd_json::OwnedValue;
use simd_json::derived::{TypedArrayValue, ValueObjectAccess};
use simd_json::prelude::ValueArrayAccess;

// Parse once at startup
static REGISTRY_BYTES: &[u8] = include_bytes!("../../../../assets/data/registries.json");

static LOADED_REGISTRY: Lazy<OwnedValue> = Lazy::new(|| {
    // simd-json mutates the buffer during parsing → needs a Vec<u8>
    let mut buf = REGISTRY_BYTES.to_vec();
    simd_json::to_owned_value(&mut buf).expect("parse registries.json")
});

static BLOCKSTATES_BYTES: &[u8] = include_bytes!("../../../../assets/data/blockstates.json");
static LOADED_BLOCKSTATES: Lazy<OwnedValue> = Lazy::new(|| {
    let mut buf = BLOCKSTATES_BYTES.to_vec();
    simd_json::to_owned_value(&mut buf).expect("parse blockstates.json")
});

static ITEM_TO_BLOCK_BYTES: &[u8] =
    include_bytes!("../../../../assets/data/item_to_block_mapping.json");
static LOADED_ITEM_TO_BLOCK: Lazy<OwnedValue> = Lazy::new(|| {
    let mut buf = ITEM_TO_BLOCK_BYTES.to_vec();
    simd_json::to_owned_value(&mut buf).expect("parse item_to_block_mapping.json")
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
            // First, check if `cur` is an array.
            if cur.is_array() {
                cur = cur.get_idx(idx)?;
            } else {
                // If it's not an array, treat the number as an object key
                cur = cur.get(seg)?;
            }
        } else {
            cur = cur.get(seg)?;
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

/// Looks up a block state by its protocol ID in `blockstates.json`.
pub fn lookup_blockstate(protocol_id: &str) -> Option<&OwnedValue> {
    LOADED_BLOCKSTATES.get(protocol_id)
}

// --- ADD THIS NEW FUNCTION ---
/// Looks up a block state ID from an item ID in `item_to_block_mapping.json`.
pub fn lookup_item_to_block(item_id: &str) -> Option<&OwnedValue> {
    LOADED_ITEM_TO_BLOCK.get(item_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use simd_json::{derived::TypedObjectValue, prelude::ValueAsScalar};
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

    #[test]
    fn test_lookup_main_registry() {
        // Test that the main registry is loading
        let apple_id = lookup("minecraft:item/entries/minecraft:apple/protocol_id");
        assert!(apple_id.is_some());
    }

    #[test]
    fn test_lookup_blockstate_static_loader() {
        // Test if LOADED_BLOCKSTATES is parsing correctly.
        // This will panic if the file isn't found or parsed.
        assert!(
            LOADED_BLOCKSTATES.is_object(),
            "blockstates.json is not a JSON object"
        );
    }

    #[test]
    fn test_lookup_blockstate_stone() {
        // This is the test that's failing in item.rs
        // Let's see if it fails here too.
        let stone_entry = lookup_blockstate("1");
        assert!(
            stone_entry.is_some(),
            "lookup_blockstate(\"1\") returned None"
        );

        let name = stone_entry.unwrap().get("name").unwrap().as_str().unwrap();
        assert_eq!(name, "minecraft:stone");
    }

    #[test]
    fn test_lookup_blockstate_grass() {
        let grass_entry = lookup_blockstate("9");
        assert!(
            grass_entry.is_some(),
            "lookup_blockstate(\"9\") returned None"
        );

        let name = grass_entry.unwrap().get("name").unwrap().as_str().unwrap();
        assert_eq!(name, "minecraft:grass_block");
    }
}

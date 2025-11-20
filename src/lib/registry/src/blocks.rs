use crate::get_registry;

/// Looks up a block *name* (e.g., "minecraft:stone") from a protocol ID string.
pub fn lookup_blockstate_name(protocol_id: &str) -> Option<&'static str> {
    get_registry()
        .blockstates
        .id_to_name
        .get(protocol_id)
        .map(|s| s.as_str())
}
/// Looks up a block state ID string from an item ID string.
pub fn lookup_item_to_block_id_str(item_id_str: &str) -> Option<&'static str> {
    get_registry()
        .item_block_map
        .item_id_str_to_blockstate_id_str
        .get(item_id_str)
        .map(|s| s.as_str())
}

/// Looks up a block's hardness (e.g., 1.5 for stone) from its name.
pub fn lookup_block_hardness(block_name: &str) -> Option<f32> {
    get_registry()
        .block_properties
        .name_to_hardness
        .get(block_name)
        .copied()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::init;

    fn setup() {
        init();
    }

    #[test]
    fn test_lookup_blockstate_stone() {
        setup();
        let stone_entry = lookup_blockstate_name("1");
        assert!(
            stone_entry.is_some(),
            "lookup_blockstate_name(\"1\") returned None"
        );
        assert_eq!(stone_entry.unwrap(), "minecraft:stone");
    }

    #[test]
    fn test_lookup_item_to_block_id() {
        setup();
        // Assuming item "1" (stone) maps to blockstate "1"
        let block_id_str = lookup_item_to_block_id_str("1");
        assert!(
            block_id_str.is_some(),
            "lookup_item_to_block_id_str(\"1\") returned None"
        );
        assert_eq!(block_id_str.unwrap(), "1");
    }
}

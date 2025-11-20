use crate::get_registry;

/// Looks up an item's protocol ID (e.g., 840) from its name (e.g., "minecraft:apple").
pub fn lookup_item_protocol_id(name: &str) -> Option<i32> {
    get_registry().items.name_to_id.get(name).copied()
}

/// Looks up an item's name (e.g., "minecraft:apple") from its protocol ID (e.g., 840).
pub fn lookup_item_name(protocol_id: i32) -> Option<&'static str> {
    get_registry()
        .items
        .id_to_name
        .get(&protocol_id)
        .map(|s| s.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::init; // Import init to setup the registry for testing

    fn setup() {
        init();
    }

    #[test]
    fn test_lookup_item_protocol_id() {
        setup();
        let apple_id = lookup_item_protocol_id("minecraft:apple");
        assert!(
            apple_id.is_some(),
            "lookup_item_protocol_id(\"minecraft:apple\") failed"
        );
        assert_eq!(apple_id.unwrap(), 840);

        let cobble_id = lookup_item_protocol_id("minecraft:cobblestone");
        assert!(
            cobble_id.is_some(),
            "lookup_item_protocol_id(\"minecraft:cobblestone\") failed"
        );
    }

    #[test]
    fn test_lookup_item_name() {
        setup();
        let apple_name = lookup_item_name(840);
        assert!(apple_name.is_some(), "lookup_item_name(840) failed");
        assert_eq!(apple_name.unwrap(), "minecraft:apple");
    }
}

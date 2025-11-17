#[cfg(test)]
mod tests {
    use crate::generated::*;

    #[test]
    fn test_blocks_generated() {
        // Test that we can access some block data
        let air = crate::blocks::air::AIR;
        assert_eq!(air.id, 0);

        let stone = crate::blocks::stone::STONE;
        assert_eq!(stone.id, 1);
    }

    #[test]
    fn test_enchantments_generated() {
        // Test that we can access enchantment data
        let sharpness = crate::generated::enchantments::SHARPNESS;
        assert_eq!(sharpness.id, 0);
        assert_eq!(sharpness.max_level, 5);
    }

    #[test]
    fn test_items_generated() {
        // Test that we can access item data
        let air_item = crate::generated::items::AIR;
        assert_eq!(air_item.name, "minecraft:air");
    }
}

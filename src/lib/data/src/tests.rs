#[cfg(test)]
mod tests {
    use super::generated::*;

    #[test]
    fn test_blocks_generated() {
        // Test that we can access some block data
        let air = blocks::AIR;
        assert_eq!(air.id, 0);
        
        let stone = blocks::STONE;
        assert_eq!(stone.id, 1);
        
        // Test instrument enum
        use blocks::Instrument;
        let instrument = Instrument::HARP;
        assert_eq!(instrument as u8, Instrument::HARP as u8);
    }

    #[test]
    fn test_enchantments_generated() {
        // Test that we can access enchantment data
        let sharpness = enchantments::SHARPNESS;
        assert_eq!(sharpness.id, 0);
        assert_eq!(sharpness.max_level, 5);
    }

    #[test]
    fn test_items_generated() {
        // Test that we can access item data
        let air_item = items::AIR;
        assert_eq!(air_item.name, "minecraft:air");
    }
}
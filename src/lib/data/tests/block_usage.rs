#[cfg(test)]
mod tests {
    use crate::blocks;

    #[test]
    fn test_block_access() {
        // Access specific blocks
        let air = blocks::air::AIR;
        let stone = blocks::stone::STONE;

        assert_eq!(air.id, 0);
        assert_eq!(air.name, "air");
        assert_eq!(stone.id, 1);
        assert_eq!(stone.name, "stone");

        // Test block properties
        assert_eq!(air.hardness, 0.0);
        assert_eq!(stone.hardness, 1.5);

        println!("✓ Block access works correctly!");
        println!("  Air: ID={}, hardness={}", air.id, air.hardness);
        println!("  Stone: ID={}, hardness={}", stone.id, stone.hardness);
    }

    #[test]
    fn test_block_states() {
        // Test block states
        let air_states = blocks::air::STATES;
        assert!(!air_states.is_empty());

        let first_state = air_states[0];
        assert_eq!(first_state.id, 0);
        assert_eq!(first_state.piston_behavior, "NORMAL");

        println!("✓ Block states work correctly!");
        println!(
            "  Air state 0: piston_behavior={}",
            first_state.piston_behavior
        );
    }

    #[test]
    fn test_shapes() {
        // Test shapes
        let shapes = blocks::shapes::SHAPES;
        assert!(!shapes.is_empty());

        let first_shape = shapes[0];
        assert_eq!(first_shape.min, [0.0, 0.0, 0.0]);
        assert_eq!(first_shape.max, [1.0, 1.0, 1.0]);

        println!("✓ Shapes work correctly!");
        println!(
            "  First shape: {:?} to {:?}",
            first_shape.min, first_shape.max
        );
    }
}

use crate::blocks;

#[test]
fn test_blocks_basic_usage() {
    // Test from blocks_usage.rs
    let air_block = blocks::AIR;
    let stone_block = blocks::STONE;
    let grass_block = blocks::GRASS_BLOCK;

    assert_eq!(air_block.id, 0);
    assert_eq!(air_block.name, "air");
    assert_eq!(stone_block.id, 1);
    assert_eq!(stone_block.name, "stone");
    assert_eq!(grass_block.name, "grass_block");

    // Test block properties
    assert!(stone_block.hardness > 0.0);
    assert!(grass_block.slipperiness > 0.0);
}

#[test]
fn test_blocks_lookup_functions() {
    // Test ID lookup
    let block_by_id = blocks::Block::by_id(1);
    assert!(block_by_id.is_some());
    assert_eq!(block_by_id.unwrap().name, "stone");

    // Test string lookup
    let block_by_name = blocks::Block::by_name("stone");
    assert!(block_by_name.is_some());
    assert_eq!(block_by_name.unwrap().id, 1);

    // Test non-existent lookups
    assert!(blocks::Block::by_id(9999).is_none());
    assert!(blocks::Block::by_name("nonexistent_block").is_none());
}

#[test]
fn test_blocks_api_compatibility() {
    // Test that both APIs still work
    assert_eq!(blocks::STONE.id, blocks::stone::STONE.id);
    assert_eq!(blocks::AIR.id, blocks::air::AIR.id);
    assert_eq!(blocks::WATER.id, blocks::water::WATER.id);
}

#[test]
fn test_blocks_properties() {
    // Test interesting blocks with specific properties
    let diamond_ore = blocks::DIAMOND_ORE;
    let obsidian = blocks::OBSIDIAN;
    let bedrock = blocks::BEDROCK;

    assert!(diamond_ore.hardness > 0.0);
    assert!(diamond_ore.blast_resistance > 0.0);

    // Obsidian should be very hard and blast resistant
    assert!(obsidian.hardness > 20.0);
    assert!(obsidian.blast_resistance > 1000.0);

    // Bedrock should be unbreakable
    assert!(bedrock.hardness == -1.0);
    assert!(bedrock.blast_resistance > 1000000.0);
}

#[test]
fn test_blocks_states() {
    // Test block states
    let air_states = blocks::air::STATES;
    let water_states = blocks::water::STATES;

    assert!(!air_states.is_empty());
    assert!(!water_states.is_empty());

    // Air should have exactly one state
    assert_eq!(air_states.len(), 1);

    // Water should have multiple states (leveling, flowing, etc.)
    assert!(water_states.len() > 1);

    // Test state properties
    if let Some(first_state) = air_states.first() {
        assert!(!first_state.piston_behavior.is_empty());
    }
}

#[test]
fn test_blocks_shapes() {
    // Test collision shapes
    let shapes = blocks::shapes::SHAPES;
    assert!(!shapes.is_empty());

    if let Some(first_shape) = shapes.first() {
        assert!(first_shape.min_x <= first_shape.max_x);
        assert!(first_shape.min_y <= first_shape.max_y);
        assert!(first_shape.min_z <= first_shape.max_z);
    }
}

#[test]
fn test_blocks_comprehensive_demo() {
    // Test from blocks_demo.rs - comprehensive block testing

    // Basic blocks
    let air = blocks::AIR;
    let stone = blocks::STONE;
    let grass = blocks::GRASS_BLOCK;
    let water = blocks::WATER;

    // Verify basic properties
    assert_eq!(air.id, 0);
    assert_eq!(stone.id, 1);
    assert!(grass.id > 0);
    assert!(water.id > 0);

    // Verify names are not empty
    assert!(!air.name.is_empty());
    assert!(!stone.name.is_empty());
    assert!(!grass.name.is_empty());
    assert!(!water.name.is_empty());

    // Test slipperiness values
    assert!(stone.slipperiness >= 0.0);
    assert!(grass.slipperiness >= 0.0);
    assert!(water.velocity_multiplier >= 0.0);

    // Test interesting blocks
    let gold_ore = blocks::gold_ore::GOLD_ORE;
    let diamond_ore = blocks::diamond_ore::DIAMOND_ORE;
    let glass = blocks::glass::GLASS;

    let interesting_blocks = vec![
        ("Gold Ore", gold_ore),
        ("Diamond Ore", diamond_ore),
        ("Glass", glass),
    ];

    for (name, block) in interesting_blocks {
        assert!(
            !block.name.is_empty(),
            "{} should have non-empty name",
            name
        );
        assert!(
            block.hardness >= 0.0 || block.hardness == -1.0,
            "{} should have valid hardness",
            name
        );
        assert!(
            block.blast_resistance >= 0.0,
            "{} should have valid blast resistance",
            name
        );
    }
}

#[test]
fn test_blocks_api_improved() {
    // Test from blocks_api_test.rs - improved API testing

    // Test direct access
    const {
        assert!(blocks::AIR.id == 0);
        assert!(blocks::STONE.id == 1);
        assert!(blocks::GRASS_BLOCK.id > 0);
        assert!(blocks::WATER.id > 0);
    }
    // Test ID lookup for specific blocks
    if let Some(block) = blocks::Block::by_id(1) {
        assert_eq!(block.name, "stone");
        assert!(block.hardness > 0.0);
    } else {
        panic!("Should find block with ID 1");
    }

    if let Some(block) = blocks::Block::by_id(35) {
        assert!(!block.name.is_empty());
        assert!(block.hardness >= 0.0);
    } else {
        panic!("Should find block with ID 35");
    }

    // Test string lookup
    let test_blocks = vec!["stone", "diamond_ore", "bedrock"];
    for block_name in test_blocks {
        if let Some(block) = blocks::Block::by_name(block_name) {
            assert!(!block.name.is_empty());
        } else {
            panic!("Should find block by name: {}", block_name);
        }
    }

    // Test that both APIs work
    assert_eq!(blocks::STONE.id, blocks::stone::STONE.id);

    // Test total blocks count
    assert!(!blocks::ALL_BLOCKS.is_empty());
    assert!(blocks::ALL_BLOCKS.len() > 100); // Should have many blocks

    // Test shapes count
    assert!(!blocks::shapes::SHAPES.is_empty());
}

#[test]
fn test_blocks_edge_cases() {
    // Test edge cases and error conditions

    // Test invalid ID
    assert!(blocks::Block::by_id(u32::MAX).is_none());

    // Test invalid names
    assert!(blocks::Block::by_name("").is_none());
    assert!(blocks::Block::by_name("definitely_not_a_block").is_none());
    assert!(blocks::Block::by_name("   ").is_none());

    // Test case sensitivity
    let _stone_lower = blocks::Block::by_name("stone").unwrap();
    let _stone_upper = blocks::Block::by_name("STONE").unwrap();
    let _stone_mixed = blocks::Block::by_name("Stone").unwrap();

    // Depending on implementation, case sensitivity might vary
    // We just verify that at least one works
}

#[test]
fn test_blocks_consistency() {
    // Test consistency across different access methods

    // Get stone through different methods
    let direct_stone = blocks::STONE;
    let submodule_stone = blocks::stone::STONE;
    let id_lookup = blocks::Block::by_id(1).unwrap();
    let name_lookup = blocks::Block::by_name("stone").unwrap();

    // All should have the same properties
    assert_eq!(direct_stone.id, submodule_stone.id);
    assert_eq!(direct_stone.id, id_lookup.id);
    assert_eq!(direct_stone.id, name_lookup.id);

    assert_eq!(direct_stone.name, submodule_stone.name);
    assert_eq!(direct_stone.name, id_lookup.name);
    assert_eq!(direct_stone.name, name_lookup.name);

    // Properties should be consistent
    assert_eq!(direct_stone.hardness, submodule_stone.hardness);
    assert_eq!(direct_stone.hardness, id_lookup.hardness);
    assert_eq!(direct_stone.hardness, name_lookup.hardness);
}

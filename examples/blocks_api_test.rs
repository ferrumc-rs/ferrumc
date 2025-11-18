use ferrumc_data::blocks;

fn main() {
    println!("=== Testing Improved Blocks API ===\n");

    // Test direct access: blocks::STONE instead of blocks::stone::STONE
    println!("Direct Access:");
    println!("  Air: ID={}, name={}", blocks::AIR.id, blocks::AIR.name);
    println!(
        "  Stone: ID={}, name={}",
        blocks::STONE.id,
        blocks::STONE.name
    );
    println!(
        "  Grass: ID={}, name={}",
        blocks::GRASS_BLOCK.id,
        blocks::GRASS_BLOCK.name
    );
    println!(
        "  Water: ID={}, name={}",
        blocks::WATER.id,
        blocks::WATER.name
    );

    // Test ID lookup
    println!("\nID Lookup:");
    if let Some(block) = blocks::Block::by_id(1) {
        println!(
            "  Block ID 1: {} (hardness: {})",
            block.name, block.hardness
        );
    }

    if let Some(block) = blocks::Block::by_id(35) {
        println!(
            "  Block ID 35: {} (hardness: {})",
            block.name, block.hardness
        );
    }

    // Test string lookup
    println!("\nString Lookup:");
    if let Some(block) = blocks::Block::by_name("stone") {
        println!(
            "  stone: ID={}, blast_resistance={}",
            block.id, block.blast_resistance
        );
    } else {
        println!("  stone: not found");
    }

    if let Some(block) = blocks::Block::by_name("diamond_ore") {
        println!(
            "  diamond_ore: ID={}, hardness={}",
            block.id, block.hardness
        );
    } else {
        println!("  diamond_ore: not found");
    }

    if let Some(block) = blocks::Block::by_name("bedrock") {
        println!("  bedrock: ID={}, hardness={}", block.id, block.hardness);
    } else {
        println!("  bedrock: not found");
    }

    // Test that both APIs still work
    println!("\nBoth APIs Work:");
    println!("  blocks::STONE: ID={}", blocks::STONE.id);
    println!("  blocks::stone::STONE: ID={}", blocks::stone::STONE.id);
    println!(
        "  Both are the same: {}",
        blocks::STONE.id == blocks::stone::STONE.id
    );

    // Test some interesting blocks
    println!("\nInteresting Blocks:");
    println!(
        "  Diamond Ore: hardness={}, blast_resistance={}",
        blocks::DIAMOND_ORE.hardness,
        blocks::DIAMOND_ORE.blast_resistance
    );
    println!(
        "  Obsidian: hardness={}, blast_resistance={}",
        blocks::OBSIDIAN.hardness,
        blocks::OBSIDIAN.blast_resistance
    );
    println!(
        "  Bedrock: hardness={}, blast_resistance={}",
        blocks::BEDROCK.hardness,
        blocks::BEDROCK.blast_resistance
    );

    println!("\nTotal blocks available: {}", blocks::ALL_BLOCKS.len());
    println!("Total shapes available: {}", blocks::shapes::SHAPES.len());

    println!("\n=== API Test Complete ===");
}


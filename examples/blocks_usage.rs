use ferrumc_data::blocks;

fn main() {
    // Access specific blocks by their constants (NEW API)
    let air_block = blocks::AIR;
    let stone_block = blocks::STONE;
    let grass_block = blocks::GRASS_BLOCK;
    
    println!("Air block: ID={}, name={}", air_block.id, air_block.name);
    println!("Stone block: ID={}, name={}", stone_block.id, stone_block.name);
    println!("Grass block: ID={}, name={}", grass_block.id, grass_block.name);
    
    // Access block properties
    println!("Stone hardness: {}", stone_block.hardness);
    println!("Grass slipperiness: {}", grass_block.slipperiness);
    
    // Use the lookup functions (NEW API)
    if let Some(block_by_id) = blocks::Block::by_id(1) {
        println!("Block with ID 1: {}", block_by_id.name);
    }
    
    if let Some(block_by_name) = blocks::Block::by_name("stone") {
        println!("Found stone by name: ID={}", block_by_name.id);
    }
    
    // Access all blocks array
    println!("Total blocks available: {}", blocks::ALL_BLOCKS.len());
    
    // Access block states (still using old API for states)
    if let Some(first_state) = blocks::air::STATES.first() {
        println!("Air state ID: {}, piston behavior: {}", 
                 first_state.id, first_state.piston_behavior);
    }
    
    // Access shapes
    println!("Available shapes: {}", blocks::SHAPES.len());
    if let Some(first_shape) = blocks::SHAPES.first() {
        println!("First shape bounds: {:?} to {:?}", first_shape.min, first_shape.max);
    }
    
    // Test that both APIs still work
    println!("\nBoth APIs work:");
    println!("  blocks::STONE: ID={}", blocks::STONE.id);
    println!("  blocks::stone::STONE: ID={}", blocks::stone::STONE.id);
}
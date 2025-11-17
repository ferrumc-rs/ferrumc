use ferrumc_data::blocks;

fn main() {
    // Access specific blocks by their constants
    let air_block = blocks::air::AIR;
    let stone_block = blocks::stone::STONE;
    let grass_block = blocks::grass_block::GRASS_BLOCK;
    
    println!("Air block: ID={}, name={}", air_block.id, air_block.name);
    println!("Stone block: ID={}, name={}", stone_block.id, stone_block.name);
    println!("Grass block: ID={}, name={}", grass_block.id, grass_block.name);
    
    // Access block properties
    println!("Stone hardness: {}", stone_block.hardness);
    println!("Grass slipperiness: {}", grass_block.slipperiness);
    
    // Use the lookup functions
    if let Some(block_by_id) = blocks::types::Block::by_id(1) {
        println!("Block with ID 1: {}", block_by_id.name);
    }
    
    if let Some(block_by_name) = blocks::types::Block::by_name("minecraft:stone") {
        println!("Found stone by name: ID={}", block_by_name.id);
    }
    
    // Access all blocks array
    println!("Total blocks available: {}", blocks::ALL_BLOCKS.len());
    
    // Access block states
    if let Some(first_state) = blocks::air::STATES.first() {
        println!("Air state ID: {}, piston behavior: {}", 
                 first_state.id, first_state.piston_behavior);
    }
    
    // Access shapes
    println!("Available shapes: {}", blocks::shapes::SHAPES.len());
    if let Some(first_shape) = blocks::shapes::SHAPES.first() {
        println!("First shape bounds: {:?} to {:?}", first_shape.min, first_shape.max);
    }
}
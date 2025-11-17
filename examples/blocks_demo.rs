// This example shows how to use the generated block data
// Run with: cargo run --example blocks_demo

use ferrumc_data::blocks;

fn main() {
    println!("=== Minecraft Blocks Demo ===\n");
    
    // Access specific blocks
    let air = blocks::air::AIR;
    let stone = blocks::stone::STONE;
    let grass = blocks::grass_block::GRASS_BLOCK;
    let water = blocks::water::WATER;
    
    println!("Basic Blocks:");
    println!("  Air: ID={}, Name='{}', Hardness={}", 
             air.id, air.name, air.hardness);
    println!("  Stone: ID={}, Name='{}', Hardness={}", 
             stone.id, stone.name, stone.hardness);
    println!("  Grass: ID={}, Name='{}', Hardness={}", 
             grass.id, grass.name, grass.hardness);
    println!("  Water: ID={}, Name='{}', Hardness={}", 
             water.id, water.name, water.hardness);
    
    println!("\nBlock Properties:");
    println!("  Stone slipperiness: {}", stone.slipperiness);
    println!("  Grass slipperiness: {}", grass.slipperiness);
    println!("  Water velocity multiplier: {}", water.velocity_multiplier);
    
    // Check block states
    println!("\nBlock States:");
    let air_states = blocks::air::STATES;
    let water_states = blocks::water::STATES;
    
    println!("  Air has {} state(s)", air_states.len());
    if let Some(state) = air_states.first() {
        println!("    First state: ID={}, Piston='{}'", 
                 state.id, state.piston_behavior);
    }
    
    println!("  Water has {} state(s)", water_states.len());
    if water_states.len() > 1 {
        println!("    Water has multiple states (leveling, flowing, etc.)");
    }
    
    // Check shapes
    println!("\nCollision Shapes:");
    let shapes = blocks::shapes::SHAPES;
    println!("  Total shapes available: {}", shapes.len());
    
    if let Some(shape) = shapes.first() {
        println!("  First shape: {:?} to {:?}", shape.min, shape.max);
    }
    
    // Show some interesting blocks
    println!("\nInteresting Blocks:");
    
    // Check if we have access to various block types
    let gold_ore = blocks::gold_ore::GOLD_ORE;
    let diamond_ore = blocks::diamond_ore::DIAMOND_ORE;
    let bedrock = blocks::bedrock::BEDROCK;
    let glass = blocks::glass::GLASS;
    
    let interesting_blocks = vec![
        ("Gold Ore", gold_ore),
        ("Diamond Ore", diamond_ore),
        ("Bedrock", bedrock),
        ("Glass", glass),
    ];
    
    for (name, block) in interesting_blocks {
        println!("  {}: ID={}, Hardness={}, Blast Resistance={}", 
                 name, block.id, block.hardness, block.blast_resistance);
    }
    
    println!("\n=== Demo Complete ===");
}
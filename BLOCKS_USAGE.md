# How to Use Generated Blocks

The `ferrumc-data` crate generates block data from Minecraft's data files. Here's how to use it:

## Importing

```rust
use ferrumc_data::blocks;
```

## Accessing Specific Blocks

Each block has its own module with constants:

```rust
// Access specific blocks
let air_block = blocks::air::AIR;
let stone_block = blocks::stone::STONE;
let grass_block = blocks::grass_block::GRASS_BLOCK;

// Access block properties
println!("Air: ID={}, hardness={}", air_block.id, air_block.hardness);
println!("Stone: ID={}, hardness={}", stone_block.id, stone_block.hardness);
println!("Grass: slipperiness={}", grass_block.slipperiness);
```

## Block Properties

Each block has the following properties:
- `id: u32` - Block ID
- `name: &'static str` - Block name (e.g., "minecraft:air")
- `translation_key: &'static str` - Translation key
- `hardness: f32` - Block hardness
- `blast_resistance: f32` - Blast resistance
- `slipperiness: f32` - Slipperiness factor
- `velocity_multiplier: f32` - Velocity multiplier
- `jump_velocity_multiplier: f32` - Jump velocity multiplier
- `luminance: u32` - Light level emitted
- `item_id: u32` - Corresponding item ID

## Block States

Many blocks have multiple states:

```rust
// Access block states
let air_states = blocks::air::STATES;
if let Some(first_state) = air_states.first() {
    println!("Air state ID: {}, piston behavior: {}", 
             first_state.id, first_state.piston_behavior);
}
```

Each block state has:
- `id: u32` - State ID
- `luminance: u32` - Light level
- `piston_behavior: &'static str` - How pistons interact with this block
- `collision_shapes: &'static [u32]` - Indices to collision shapes
- `outline_shapes: &'static [u32]` - Indices to outline shapes

## Shapes

Blocks reference collision and outline shapes:

```rust
// Access all shapes
let shapes = blocks::shapes::SHAPES;
println!("Available shapes: {}", shapes.len());

if let Some(first_shape) = shapes.first() {
    println!("First shape bounds: {:?} to {:?}", 
             first_shape.min, first_shape.max);
}
```

Each shape has:
- `min: [f64; 3]` - Minimum corner coordinates
- `max: [f64; 3]` - Maximum corner coordinates

## Available Blocks

Here are some of the available block modules:
- `blocks::air` - Air block
- `blocks::stone` - Stone variants
- `blocks::grass_block` - Grass and dirt blocks
- `blocks::water` - Water
- `blocks::lava` - Lava
- `blocks::oak_planks` - Wood planks
- `blocks::glass` - Glass blocks
- `blocks::gold_ore` - Ore blocks
- And many more...

## Example Usage

```rust
use ferrumc_data::blocks;

fn main() {
    // Get some common blocks
    let air = blocks::air::AIR;
    let stone = blocks::stone::STONE;
    let dirt = blocks::dirt::DIRT;
    
    // Check block properties
    println!("Block comparison:");
    println!("  Air - ID: {}, Hardness: {}", air.id, air.hardness);
    println!("  Stone - ID: {}, Hardness: {}", stone.id, stone.hardness);
    println!("  Dirt - ID: {}, Hardness: {}", dirt.id, dirt.hardness);
    
    // Check if blocks are breakable
    if stone.hardness > 0.0 {
        println!("Stone is breakable!");
    }
    
    // Access block states
    let water_states = blocks::water::STATES;
    println!("Water has {} states", water_states.len());
    
    // Access shapes for collision detection
    let shapes = blocks::shapes::SHAPES;
    println!("Total shapes available: {}", shapes.len());
}
```

## Notes

- All block constants are compile-time constants, so they're very efficient
- Block states are static slices, also available at compile time
- The data is generated from Minecraft's actual registry data
- Block names use the Minecraft resource location format (e.g., "minecraft:stone")
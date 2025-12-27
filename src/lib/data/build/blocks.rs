use std::fs;
use std::path::PathBuf;

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
pub struct BlockData {
    blocks: Vec<Block>,
    shapes: Vec<Shape>,
}

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
pub struct Block {
    id: u32,
    name: String,
    translation_key: String,
    slipperiness: f32,
    velocity_multiplier: f32,
    jump_velocity_multiplier: f32,
    hardness: f32,
    blast_resistance: f32,
    item_id: u32,
    properties: Vec<Property>,
    default_state_id: u32,
    states: Vec<State>,
}

// Properties are actually integers in the JSON format
pub type Property = i32;

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
pub struct State {
    id: u32,
    state_flags: u32,
    side_flags: u32,
    instrument: String,
    luminance: u32,
    piston_behavior: String,
    hardness: f32,
    collision_shapes: Vec<u32>,
    outline_shapes: Vec<u32>,
}

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
pub struct Shape {
    min: [f64; 3],
    max: [f64; 3],
}

fn sanitize_name(name: &str) -> String {
    name.replace("minecraft:", "").replace(':', "_")
}

fn format_float(f: f32) -> String {
    if f.fract() == 0.0 {
        format!("{}.0", f)
    } else {
        f.to_string()
    }
}

pub fn build() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=../../../assets/extracted/blocks.json");

    let out_dir = std::env::var("OUT_DIR")?;
    let blocks_dir = PathBuf::from(out_dir.clone()).join("blocks");

    // Create blocks directory
    fs::create_dir_all(&blocks_dir)?;

    let json_content = fs::read_to_string("../../../assets/extracted/blocks.json")?;
    let data: BlockData = serde_json::from_str(&json_content)?;

    // Create types.rs
    let mut types_content = String::new();
    types_content.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n");
    types_content.push_str("pub struct BlockId(pub u32);\n\n");

    types_content.push_str("#[derive(Debug, Clone, Copy)]\n");
    types_content.push_str("pub struct Block {\n");
    types_content.push_str("    pub id: u32,\n");
    types_content.push_str("    pub name: &'static str,\n");
    types_content.push_str("    pub translation_key: &'static str,\n");
    types_content.push_str("    pub hardness: f32,\n");
    types_content.push_str("    pub blast_resistance: f32,\n");
    types_content.push_str("    pub slipperiness: f32,\n");
    types_content.push_str("    pub velocity_multiplier: f32,\n");
    types_content.push_str("    pub jump_velocity_multiplier: f32,\n");
    types_content.push_str("    pub luminance: u32,\n");
    types_content.push_str("    pub item_id: u32,\n");
    types_content.push_str("}\n\n");

    types_content.push_str("#[derive(Debug, Clone, Copy)]\n");
    types_content.push_str("pub struct Shape {\n");
    types_content.push_str("    pub min_x: f64,\n");
    types_content.push_str("    pub min_y: f64,\n");
    types_content.push_str("    pub min_z: f64,\n");
    types_content.push_str("    pub max_x: f64,\n");
    types_content.push_str("    pub max_y: f64,\n");
    types_content.push_str("    pub max_z: f64,\n");
    types_content.push_str("}\n\n");

    types_content.push_str("#[derive(Debug, Clone, Copy)]\n");
    types_content.push_str("pub struct BlockState {\n");
    types_content.push_str("    pub id: u32,\n");
    types_content.push_str("    pub luminance: u32,\n");
    types_content.push_str("    pub piston_behavior: &'static str,\n");
    types_content.push_str("    pub collision_shapes: &'static [u32],\n");
    types_content.push_str("    pub outline_shapes: &'static [u32],\n");
    types_content.push_str("}\n\n");

    fs::write(blocks_dir.join("types.rs"), types_content)?;

    // Create shapes.rs
    let mut shapes_content = String::new();
    shapes_content.push_str("use super::types::Shape;\n\n");
    shapes_content.push_str("pub const SHAPES: &[Shape] = &[\n");
    for shape in &data.shapes {
        shapes_content.push_str(&format!(
            "    Shape {{ min_x: {:.1}, min_y: {:.1}, min_z: {:.1}, max_x: {:.1}, max_y: {:.1}, max_z: {:.1} }},\n",
            shape.min[0], shape.min[1], shape.min[2], shape.max[0], shape.max[1], shape.max[2]
        ));
    }
    shapes_content.push_str("];\n");
    fs::write(blocks_dir.join("shapes.rs"), shapes_content)?;

    // Create individual block files
    for block in &data.blocks {
        let sanitized_name = sanitize_name(&block.name);
        let file_name = format!("{}.rs", sanitized_name);

        let mut content = String::new();
        content.push_str("use super::types::{Block, BlockState};\n\n");

        // Block constant
        content.push_str(&format!(
            "pub const {}: Block = Block {{\n",
            sanitized_name.to_uppercase()
        ));
        content.push_str(&format!("    id: {},\n", block.id));
        content.push_str(&format!("    name: \"{}\",\n", block.name));
        content.push_str(&format!(
            "    translation_key: \"{}\",\n",
            block.translation_key
        ));
        content.push_str(&format!(
            "    hardness: {},\n",
            format_float(block.hardness)
        ));
        content.push_str(&format!(
            "    blast_resistance: {},\n",
            format_float(block.blast_resistance)
        ));
        content.push_str(&format!(
            "    slipperiness: {},\n",
            format_float(block.slipperiness)
        ));
        content.push_str(&format!(
            "    velocity_multiplier: {},\n",
            format_float(block.velocity_multiplier)
        ));
        content.push_str(&format!(
            "    jump_velocity_multiplier: {},\n",
            format_float(block.jump_velocity_multiplier)
        ));

        // Use first state for basic properties
        let first_state = &block.states[0];
        content.push_str(&format!("    luminance: {},\n", first_state.luminance));
        content.push_str(&format!("    item_id: {},\n", block.item_id));
        content.push_str("};\n\n");

        // States
        if !block.states.is_empty() {
            content.push_str("pub const STATES: &[BlockState] = &[\n");
            for state in &block.states {
                content.push_str("    BlockState {\n");
                content.push_str(&format!("        id: {},\n", state.id));
                content.push_str(&format!("        luminance: {},\n", state.luminance));
                content.push_str(&format!(
                    "        piston_behavior: \"{}\",\n",
                    state.piston_behavior
                ));

                if !state.collision_shapes.is_empty() {
                    content.push_str("        collision_shapes: &[");
                    for (i, shape_id) in state.collision_shapes.iter().enumerate() {
                        if i > 0 {
                            content.push_str(", ");
                        }
                        content.push_str(&format!("{}", shape_id));
                    }
                    content.push_str("],\n");
                } else {
                    content.push_str("        collision_shapes: &[],\n");
                }

                if !state.outline_shapes.is_empty() {
                    content.push_str("        outline_shapes: &[");
                    for (i, shape_id) in state.outline_shapes.iter().enumerate() {
                        if i > 0 {
                            content.push_str(", ");
                        }
                        content.push_str(&format!("{}", shape_id));
                    }
                    content.push_str("],\n");
                } else {
                    content.push_str("        outline_shapes: &[],\n");
                }

                content.push_str("    },\n");
            }
            content.push_str("];\n\n");
        }

        fs::write(blocks_dir.join(&file_name), content)?;
    }

    // Create mod.rs
    let mut mod_content = String::new();
    mod_content.push_str("pub mod types;\n");
    mod_content.push_str("pub mod shapes;\n\n");

    // Add individual block modules
    for block in &data.blocks {
        let sanitized_name = sanitize_name(&block.name);
        mod_content.push_str(&format!("pub mod {};\n", sanitized_name));
    }

    mod_content.push('\n');

    // Block lookup array
    mod_content.push_str("pub const ALL_BLOCKS: &[Block] = &[\n");
    for block in &data.blocks {
        let sanitized_name = sanitize_name(&block.name);
        mod_content.push_str(&format!(
            "    {}::{},\n",
            sanitized_name,
            sanitized_name.to_uppercase()
        ));
    }
    mod_content.push_str("];\n\n");

    // Re-exports for direct access (blocks::STONE instead of blocks::stone::STONE)
    mod_content.push_str("// Re-exports for direct access to block constants\n");
    for block in &data.blocks {
        let sanitized_name = sanitize_name(&block.name);
        mod_content.push_str(&format!(
            "pub use {}::{};\n",
            sanitized_name,
            sanitized_name.to_uppercase()
        ));
    }
    mod_content.push('\n');

    // Re-export types and lookup functions
    mod_content.push_str("// Re-export types and lookup functions\n");
    mod_content.push_str("pub use types::{Block, BlockState, Shape};\n");
    mod_content.push_str("pub use shapes::SHAPES;\n\n");

    // Lookup functions
    mod_content.push_str("impl Block {\n");
    mod_content.push_str("    pub fn by_id(id: u32) -> Option<&'static Block> {\n");
    mod_content.push_str("        ALL_BLOCKS.get(id as usize)\n");
    mod_content.push_str("    }\n\n");

    mod_content.push_str("    pub fn by_name(name: &str) -> Option<&'static Block> {\n");
    mod_content.push_str("        match &*name.to_lowercase() {\n");
    for block in &data.blocks {
        let sanitized_name = sanitize_name(&block.name);
        mod_content.push_str(&format!(
            "            \"{}\" => Some(&{}::{}),\n",
            block.name,
            sanitized_name,
            sanitized_name.to_uppercase()
        ));
    }
    mod_content.push_str("            _ => None,\n");
    mod_content.push_str("        }\n");
    mod_content.push_str("    }\n");
    mod_content.push_str("}\n");

    fs::write(blocks_dir.join("mod.rs"), mod_content)?;

    // Also create a blocks.rs file in the OUT_DIR that includes the module
    let blocks_rs_content = format!(
        r#"#[path = r"{}/blocks/mod.rs"]
pub mod blocks;"#,
        out_dir
    );
    fs::write(
        PathBuf::from(&out_dir.clone()).join("blocks.rs"),
        blocks_rs_content,
    )?;

    println!("Generated {} blocks in individual files", data.blocks.len());

    Ok(())
}

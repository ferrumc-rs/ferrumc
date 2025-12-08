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
    types_content.push_str(
        "#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockId(pub u32);

#[derive(Debug, Clone, Copy)]
pub struct Block {
    pub id: u32,
    pub name: &'static str,
    pub translation_key: &'static str,
    pub hardness: f32,
    pub blast_resistance: f32,
    pub slipperiness: f32,
    pub velocity_multiplier: f32,
    pub jump_velocity_multiplier: f32,
    pub luminance: u32,
    pub item_id: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct Shape {
    pub min_x: f64,
    pub min_y: f64,
    pub min_z: f64,
    pub max_x: f64,
    pub max_y: f64,
    pub max_z: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct BlockState {
    pub id: u32,
    pub luminance: u32,
    pub piston_behavior: &'static str,
    pub collision_shapes: &'static [u32],
    pub outline_shapes: &'static [u32],
}

",
    );
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

        // Block constant
        content.push_str(&format!(
            "use super::types::{{Block, BlockState}};

pub const {ident}: Block = Block {{
    id: {id},
    name: \"{name}\",
    translation_key: \"{translation_key}\",
    hardness: {hardness},
    blast_resistance: {blast_resistance},
    slipperiness: {slipperiness},
    velocity_multiplier: {velocity_multiplier},
    jump_velocity_multiplier: {jump_velocity_multiplier},
    luminance: {luminance},
    item_id: {item_id},
}};\n\n",
            ident = sanitized_name.to_uppercase(),
            id = block.id,
            name = block.name,
            translation_key = block.translation_key,
            hardness = format_float(block.hardness),
            blast_resistance = format_float(block.blast_resistance),
            slipperiness = format_float(block.slipperiness),
            velocity_multiplier = format_float(block.velocity_multiplier),
            jump_velocity_multiplier = format_float(block.jump_velocity_multiplier),
            luminance = block.states[0].luminance,
            item_id = block.item_id
        ));

        // States
        if !block.states.is_empty() {
            content.push_str("pub const STATES: &[BlockState] = &[");
            for state in &block.states {
                content.push_str("");
                content.push_str(&format!(
                    "
    BlockState {{
        id: {id},
        luminance: {luminance},
        piston_behavior: \"{piston_behavior}\",
        collision_shapes: &[{collision_shapes}],
        outline_shapes: &[{outline_shapes}],
    }},",
                    id = state.id,
                    luminance = state.luminance,
                    piston_behavior = state.piston_behavior,
                    collision_shapes = state
                        .collision_shapes
                        .iter()
                        .map(|shape_id| shape_id.to_string())
                        .collect::<Vec<_>>()
                        .join(", "),
                    outline_shapes = state
                        .outline_shapes
                        .iter()
                        .map(|shape_id| shape_id.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
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
    mod_content.push_str(
        "pub use types::{Block, BlockState, Shape};
pub use shapes::SHAPES;

impl Block {
    pub const fn try_from_id(id: u32) -> Option<&'static Block> {
        let id = id as usize;
        if id < ALL_BLOCKS.len() {
            Some(&ALL_BLOCKS[id])
        } else {
            None
        }
    }
    pub const fn try_from_name(name: &str) -> Option<&'static Block> {
        let name = crate::helpers::strip_prefix_or_self(name, \"minecraft:\");
        match name {
",
    );
    for block in &data.blocks {
        let sanitized_name = sanitize_name(&block.name);
        mod_content.push_str(&format!(
            "            \"{}\" => Some(&{}::{}),\n",
            block.name,
            sanitized_name,
            sanitized_name.to_uppercase()
        ));
    }
    mod_content.push_str(
        "
            _ => None,
        }
    }
}
",
    );
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

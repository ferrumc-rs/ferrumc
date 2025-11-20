use heck::{ToShoutySnakeCase, ToUpperCamelCase};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use serde::Deserialize;
use std::{
    collections::BTreeMap,
    fs,
    path::Path,
    process::{Command, Stdio},
    io::Write,
};
use syn::{Ident, LitInt, LitStr};

// Simple block structure for generation
#[derive(Deserialize, Clone, Debug)]
pub struct Block {
    pub id: u16,
    pub name: String,
    pub translation_key: String,
    pub hardness: f32,
    pub blast_resistance: f32,
    pub item_id: u16,
    pub slipperiness: f32,
    pub velocity_multiplier: f32,
    pub jump_velocity_multiplier: f32,
    pub default_state_id: u16,
    pub states: Vec<BlockState>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct BlockState {
    pub id: u16,
    pub state_flags: u16,
    pub side_flags: u8,
    pub instrument: String,
    pub luminance: u8,
    pub piston_behavior: String,
    pub hardness: f32,
    pub collision_shapes: Vec<u16>,
    pub outline_shapes: Vec<u16>,
    pub opacity: Option<u8>,
    pub block_entity_type: Option<u16>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CollisionShape {
    pub min: [f64; 3],
    pub max: [f64; 3],
}

#[derive(Deserialize, Clone, Debug)]
pub struct BlockAssets {
    pub blocks: Vec<Block>,
    pub shapes: Vec<CollisionShape>,
    pub block_entity_types: Vec<String>,
}

fn const_block_name_from_block_name(block: &str) -> String {
    block.to_shouty_snake_case()
}

pub(crate) fn build() -> TokenStream {
    println!("cargo:rerun-if-changed=../../../assets/extracted/blocks.json");

    let blocks_assets: BlockAssets =
        serde_json::from_str(&fs::read_to_string("../../../assets/extracted/blocks.json").unwrap())
            .expect("Failed to parse blocks.json");

    // Create blocks directory
    let blocks_dir = Path::new("src/generated/blocks");
    fs::create_dir_all(blocks_dir).expect("Failed to create blocks directory");

    // Generate simple types file
    let types_content = r#"
use std::collections::BTreeMap;
use phf;

#[derive(Clone, Copy, Debug)]
pub struct Flammable {
    pub spread_chance: u8,
    pub burn_chance: u8,
}

#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub enum Instrument {
    HARP, BASS_DRUM, SNARE, CLICK, BASS, FLUTE, BELL, GUITAR,
    CHIME, XYLOPHONE, IRON_XYLOPHONE, COW_BELL, DIDGERIDOO,
    BIT, BANJO, PLING, HAT, ZOMBIE, SKELETON, CREEPER,
    DRAGON, WITHER_SKELETON, PIGLIN, CUSTOM_HEAD,
}

#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub enum PistonBehavior {
    Normal, Destroy, Block, Ignore, PushOnly,
}

#[derive(Clone, Debug)]
pub struct BlockState {
    pub id: u16,
    pub state_flags: u16,
    pub side_flags: u8,
    pub instrument: Instrument,
    pub luminance: u8,
    pub piston_behavior: PistonBehavior,
    pub hardness: f32,
    pub collision_shapes: &'static [u16],
    pub outline_shapes: &'static [u16],
    pub opacity: u8,
    pub block_entity_type: u16,
}

#[derive(Clone, Debug)]
pub struct Block {
    pub id: u16,
    pub name: &'static str,
    pub translation_key: &'static str,
    pub hardness: f32,
    pub blast_resistance: f32,
    pub slipperiness: f32,
    pub velocity_multiplier: f32,
    pub jump_velocity_multiplier: f32,
    pub item_id: u16,
    pub default_state: &'static BlockState,
    pub states: &'static [BlockState],
    pub flammable: Option<Flammable>,
}

impl BlockState {
    pub fn from_id(id: u16) -> &'static Self {
        Block::STATE_FROM_STATE_ID[id as usize]
    }

    pub fn from_id_with_block(id: u16) -> (&'static Block, &'static Self) {
        let block = Block::from_state_id(id);
        let state = Block::STATE_FROM_STATE_ID[id as usize];
        (block, state)
    }

    pub const fn is_solid(&self) -> bool {
        self.opacity != 0
    }

    pub const fn is_air(&self) -> bool {
        self.id == 0
    }
}
"#;

    fs::write(blocks_dir.join("types.rs"), types_content)
        .expect("Failed to write types.rs");

    // Generate collision shapes
    let mut shapes_content = String::from(
        r#"
#[derive(Clone, Copy, Debug)]
pub struct CollisionShape {
    pub min: [f64; 3],
    pub max: [f64; 3],
}

pub const COLLISION_SHAPES: &[CollisionShape] = &[
"#
    );

    for shape in &blocks_assets.shapes {
        shapes_content.push_str(&format!(
            "    CollisionShape {{ min: [{}, {}, {}], max: [{}, {}, {}] }},\n",
            shape.min[0], shape.min[1], shape.min[2],
            shape.max[0], shape.max[1], shape.max[2]
        ));
    }

    shapes_content.push_str("];\n");

    fs::write(blocks_dir.join("collision_shapes.rs"), shapes_content)
        .expect("Failed to write collision_shapes.rs");

    // Generate block entity types
    let mut entity_types_content = String::from("pub const BLOCK_ENTITY_TYPES: &[&str] = &[\n");
    for entity_type in &blocks_assets.block_entity_types {
        entity_types_content.push_str(&format!("    \"{}\",\n", entity_type));
    }
    entity_types_content.push_str("];\n");

    fs::write(blocks_dir.join("block_entity_types.rs"), entity_types_content)
        .expect("Failed to write block_entity_types.rs");

    // Generate block constants
    let mut constants_content = String::from("use super::types::*;\n\n");
    
    for block in &blocks_assets.blocks {
        let const_name = const_block_name_from_block_name(&block.name);
        constants_content.push_str(&format!(
            r#"pub const {}: Block = Block {{
    id: {},
    name: "{}",
    translation_key: "{}",
    hardness: {},
    blast_resistance: {},
    slipperiness: {},
    velocity_multiplier: {},
    jump_velocity_multiplier: {},
    item_id: {},
    default_state: &{}::DEFAULT_STATE,
    states: &{}::STATES,
    flammable: None,
};"#,
            const_name,
            block.id,
            block.name,
            block.translation_key,
            block.hardness,
            block.blast_resistance,
            block.slipperiness,
            block.velocity_multiplier,
            block.jump_velocity_multiplier,
            block.item_id,
            const_name,
            const_name
        ));
        constants_content.push('\n');
        constants_content.push('\n');
    }

    fs::write(blocks_dir.join("block_constants.rs"), constants_content)
        .expect("Failed to write block_constants.rs");

    // Generate main implementation
    let mut impl_content = String::from(
        r#"
use phf;
use super::types::*;
use super::collision_shapes::*;
use super::block_entity_types::*;

impl Block {
"#
    );

    // Generate from_name map
    impl_content.push_str("    const BLOCK_FROM_NAME_MAP: phf::Map<&'static str, &'static Block> = phf::phf_map! {\n");
    for block in &blocks_assets.blocks {
        let const_name = const_block_name_from_block_name(&block.name);
        impl_content.push_str(&format!("        \"{}\" => &{},\n", block.name, const_name));
    }
    impl_content.push_str("    };\n\n");

    // Generate methods
    impl_content.push_str(&format!(
        r#"    pub fn from_registry_key(name: &str) -> Option<&'static Self> {{
        Self::BLOCK_FROM_NAME_MAP.get(name).copied()
    }}

    pub fn from_name(name: &str) -> Option<&'static Self> {{
        let key = name.strip_prefix("minecraft:").unwrap_or(name);
        Self::BLOCK_FROM_NAME_MAP.get(key).copied()
    }}

    pub const fn from_id(id: u16) -> &'static Self {{
        match id {{
"#
    ));

    for block in &blocks_assets.blocks {
        let const_name = const_block_name_from_block_name(&block.name);
        impl_content.push_str(&format!("            {} => &{},\n", block.id, const_name));
    }

    impl_content.push_str(
        r#"            _ => &AIR,
        }
    }

    pub fn from_state_id(id: u16) -> &'static Self {
        Self::from_id(id) // Simplified for now
    }

    pub const fn from_item_id(id: u16) -> Option<&'static Self> {
        match id {
"#
    );

    // Generate item mappings (unique items only)
    let mut seen_items = std::collections::HashSet::new();
    for block in &blocks_assets.blocks {
        if seen_items.insert(block.item_id) {
            let const_name = const_block_name_from_block_name(&block.name);
            impl_content.push_str(&format!("            {} => Some(&{}),\n", block.item_id, const_name));
        }
    }

    impl_content.push_str(
        r#"            _ => None,
        }
    }
}
"#
    );

    fs::write(blocks_dir.join("block_impl.rs"), impl_content)
        .expect("Failed to write block_impl.rs");

    // Create mod.rs
    let mut mod_content = String::new();
    mod_content.push_str("pub mod types;\n");
    mod_content.push_str("pub mod collision_shapes;\n");
    mod_content.push_str("pub mod block_entity_types;\n");
    mod_content.push_str("pub mod block_constants;\n");
    mod_content.push_str("pub mod block_impl;\n");
    mod_content.push_str("\n");
    mod_content.push_str("// Re-export main types\n");
    mod_content.push_str("pub use types::*;\n");
    mod_content.push_str("pub use collision_shapes::*;\n");
    mod_content.push_str("pub use block_entity_types::*;\n");
    mod_content.push_str("pub use block_constants::*;\n");
    mod_content.push_str("pub use block_impl::*;\n");

    fs::write(blocks_dir.join("mod.rs"), mod_content)
        .expect("Failed to write mod.rs");

    quote! {}
}
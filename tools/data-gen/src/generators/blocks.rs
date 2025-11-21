use crate::utils;
use heck::ToShoutySnakeCase;
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::io::Write;
use std::path::Path;

// --- 1. Structures for Vanilla Report (IDs) ---
// Source: generated/reports/blocks.json
// Structure: "minecraft:stone": { "states": [...] }
type BlockReport = BTreeMap<String, BlockDefinition>;

#[derive(Deserialize)]
struct BlockDefinition {
    #[serde(default)]
    states: Vec<BlockStateEntry>,
}

#[derive(Deserialize)]
struct BlockStateEntry {
    id: u32,
    #[serde(default)]
    default: bool,
}

// --- 2. Structures for Physics Data (Extracted) ---
// Source: temp_gen/physics.json
// Structure: { "minecraft:stone": { "hardness": 1.5, "resistance": 6.0 } }
// Note: The Java dumper outputs a Map, so we use HashMap directly.

#[derive(Deserialize)]
struct PhysicsEntry {
    hardness: f32,
    resistance: f32,
    friction: f32,
    speed_factor: f32,
    jump_factor: f32,
    light_emission: u8,
    is_air: bool,
    is_solid: bool,
}

type PhysicsReport = HashMap<String, PhysicsEntry>;

// --- 3. Generator Logic ---

pub fn generate(ids_path: &Path, physics_path: &Path, output: &Path) {
    println!("   ... Parsing IDs from {:?}", ids_path);
    let ids_content = fs::read_to_string(ids_path).expect("Failed to read blocks.json");
    let ids_report: BlockReport =
        serde_json::from_str(&ids_content).expect("Failed to parse blocks.json");

    println!("   ... Parsing Physics from {:?}", physics_path);
    let physics_content = fs::read_to_string(physics_path).expect("Failed to read physics.json");
    let physics_report: PhysicsReport =
        serde_json::from_str(&physics_content).expect("Failed to parse physics.json");

    let mut file = utils::create_file(output);

    // Write Header
    writeln!(file, "use ferrumc_core::world::block_data::BlockData;").unwrap();
    writeln!(file, "use std::collections::BTreeMap;").unwrap();
    writeln!(file, "").unwrap();

    let mut match_arms = String::new();

    // Iterate over the Vanilla Report (Source of Truth for existence)
    for (name, def) in ids_report {
        let clean_name = name.replace("minecraft:", "");
        let const_name = clean_name.to_shouty_snake_case();

        // Handle Rust keywords if a block is named "struct" or "match" (unlikely but safe)
        let safe_const_name = if const_name == "TYPE" || const_name == "MATCH" {
            format!("{}_BLOCK", const_name)
        } else {
            const_name
        };

        // 1. Get Default State ID
        let default_state = def.states.iter().find(|s| s.default).or(def.states.first());
        let default_id = default_state.map(|s| s.id).unwrap_or(0);

        // 2. Get Physics Data
        // We use a default struct if data is missing to prevent crashes
        let phys = physics_report.get(&name).unwrap_or(&PhysicsEntry {
            hardness: 0.0,
            resistance: 0.0,
            friction: 0.6, // Standard block friction
            speed_factor: 1.0,
            jump_factor: 1.0,
            light_emission: 0,
            is_air: false,
            is_solid: true,
        });

        // 3. Write Constant
        writeln!(
            file,
            "pub const {}: BlockData = BlockData {{",
            safe_const_name
        )
        .unwrap();
        writeln!(file, "    name: \"{}\".to_string(),", name).unwrap();
        writeln!(file, "    default_state_id: {},", default_id).unwrap();

        // Write new fields
        writeln!(file, "    hardness: {:.2},", phys.hardness).unwrap();
        writeln!(file, "    blast_resistance: {:.2},", phys.resistance).unwrap();
        writeln!(file, "    friction: {:.2},", phys.friction).unwrap();
        writeln!(file, "    speed_factor: {:.2},", phys.speed_factor).unwrap();
        writeln!(file, "    jump_factor: {:.2},", phys.jump_factor).unwrap();
        writeln!(file, "    light_emission: {},", phys.light_emission).unwrap();
        writeln!(file, "    is_air: {},", phys.is_air).unwrap();
        writeln!(file, "    is_solid: {},", phys.is_solid).unwrap();
        writeln!(file, "}};").unwrap();
        writeln!(file, "").unwrap();

        // 4. Add to Match Arms
        // We match against the UNPREFIXED name to be safe ("stone"),
        // but we can also match fully qualified if we want.
        // Based on your previous bug, let's support the short name.
        match_arms.push_str(&format!(
            "        \"{}\" => Some(&{}),\n",
            clean_name, safe_const_name
        ));
    }

    // Write Lookup Function
    writeln!(
        file,
        "/// Looks up a BlockData reference by its short name (e.g. \"stone\")."
    )
    .unwrap();
    writeln!(
        file,
        "pub fn get_block_by_name(name: &str) -> Option<&'static BlockData> {{"
    )
    .unwrap();

    // Helper to strip prefix if passed
    writeln!(
        file,
        "    let name = name.strip_prefix(\"minecraft:\").unwrap_or(name);"
    )
    .unwrap();

    writeln!(file, "    match name {{").unwrap();
    file.write_all(match_arms.as_bytes()).unwrap();
    writeln!(file, "        _ => None,").unwrap();
    writeln!(file, "    }}").unwrap();
    writeln!(file, "}}").unwrap();
}

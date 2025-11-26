use crate::utils;
use heck::ToShoutySnakeCase;
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::io::Write;
use std::path::Path;

// --- Structs (Same as before) ---

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

#[derive(Deserialize, Clone, Default)]
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

// --- Generator Logic ---

pub fn generate(ids_path: &Path, physics_path: &Path, output: &Path) {
    println!("   ... Parsing IDs from {:?}", ids_path);
    let ids_content = fs::read_to_string(ids_path).expect("Failed to read blocks.json");
    let ids_report: BlockReport =
        serde_json::from_str(&ids_content).expect("Failed to parse blocks.json");

    println!("   ... Parsing Physics from {:?}", physics_path);
    let physics_content = fs::read_to_string(physics_path).expect("Failed to read physics.json");
    let physics_report: PhysicsReport =
        serde_json::from_str(&physics_content).expect("Failed to parse physics.json");

    // Create the single file
    let mut file = utils::create_file(output);

    writeln!(file, "#![allow(clippy::excessive_precision)]").unwrap();

    // Header
    writeln!(file, "use ferrumc_core::world::block_data::BlockData;").unwrap();
    writeln!(file, "").unwrap();

    let mut name_match_arms = String::new();
    let mut id_match_arms = String::new();

    for (name, def) in ids_report {
        let clean_name = name.replace("minecraft:", "");
        let const_name = clean_name.to_shouty_snake_case();

        let safe_const_name = if const_name == "TYPE" || const_name == "MATCH" {
            format!("{}_BLOCK", const_name)
        } else {
            const_name
        };

        // 1. Get Defaults
        let default_state = def.states.iter().find(|s| s.default).or(def.states.first());
        let default_id = default_state.map(|s| s.id).unwrap_or(0);

        // 2. Get Physics
        let phys = physics_report.get(&name).cloned().unwrap_or_default();

        // 3. Write Constant
        writeln!(
            file,
            "pub const {}: BlockData = BlockData {{",
            safe_const_name
        )
        .unwrap();
        writeln!(file, "    name: \"{}\",", name).unwrap(); // using &'static str
        writeln!(file, "    default_state_id: {},", default_id).unwrap();
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

        // 4. Add to Name Lookup (Match against simple name "stone")
        name_match_arms.push_str(&format!(
            "        \"{}\" => Some(&{}),\n",
            clean_name, safe_const_name
        ));

        // 5. Add to ID Lookup (Map ALL state IDs to this block)
        for state in &def.states {
            id_match_arms.push_str(&format!(
                "        {} => Some(&{}),\n",
                state.id, safe_const_name
            ));
        }
    }

    // Write Name Lookup
    writeln!(
        file,
        "/// Looks up BlockData by name (e.g. \"stone\" or \"minecraft:stone\")."
    )
    .unwrap();
    writeln!(
        file,
        "pub fn get_block_by_name(name: &str) -> Option<&'static BlockData> {{"
    )
    .unwrap();
    writeln!(
        file,
        "    let name = name.strip_prefix(\"minecraft:\").unwrap_or(name);"
    )
    .unwrap();
    writeln!(file, "    match name {{").unwrap();
    file.write_all(name_match_arms.as_bytes()).unwrap();
    writeln!(file, "        _ => None,").unwrap();
    writeln!(file, "    }}").unwrap();
    writeln!(file, "}}").unwrap();

    // Write ID Lookup
    writeln!(file, "").unwrap();
    writeln!(file, "/// Looks up BlockData by Protocol State ID.").unwrap();
    writeln!(
        file,
        "pub fn get_block_by_id(id: u32) -> Option<&'static BlockData> {{"
    )
    .unwrap();
    writeln!(file, "    match id {{").unwrap();
    file.write_all(id_match_arms.as_bytes()).unwrap();
    writeln!(file, "        _ => None,").unwrap();
    writeln!(file, "    }}").unwrap();
    writeln!(file, "}}").unwrap();
}

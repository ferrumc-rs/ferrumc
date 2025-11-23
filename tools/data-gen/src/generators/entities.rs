use crate::utils;
use heck::ToShoutySnakeCase;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Deserialize)]
struct EntityEntry {
    id: u32,
    width: f32,
    height: f32,
    summonable: bool,
    fire_immune: bool,
    can_spawn_far: bool,
    category: String,
}

type EntityReport = HashMap<String, EntityEntry>;

pub fn generate(input: &Path, output: &Path) {
    println!("   ... Parsing Entities from {:?}", input);
    let content = fs::read_to_string(input).expect("Failed to read entities.json");
    let report: EntityReport =
        serde_json::from_str(&content).expect("Failed to parse entities.json");

    let mut file = utils::create_file(output);

    writeln!(file, "use ferrumc_core::entities::entity_data::*;").unwrap();
    writeln!(file, "").unwrap();

    let mut name_match_arms = String::new();
    let mut id_match_arms = String::new();

    // Sort for stable output
    let mut sorted_keys: Vec<_> = report.keys().collect();
    sorted_keys.sort();

    for key in sorted_keys {
        let entry = &report[key];
        let clean_name = key.replace("minecraft:", "");
        let const_name = clean_name.to_shouty_snake_case();

        // Map Category Enum
        let cat_enum = match entry.category.as_str() {
            "MONSTER" => "EntityCategory::Monster",
            "CREATURE" => "EntityCategory::Creature",
            "AMBIENT" => "EntityCategory::Ambient",
            "AXOLOTLS" => "EntityCategory::Axolotls",
            "UNDERGROUND_WATER_CREATURE" => "EntityCategory::UndergroundWaterCreature",
            "WATER_CREATURE" => "EntityCategory::WaterCreature",
            "WATER_AMBIENT" => "EntityCategory::WaterAmbient",
            _ => "EntityCategory::Misc",
        };

        writeln!(file, "pub const {}: EntityData = EntityData {{", const_name).unwrap();
        writeln!(file, "    name: \"{}\",", key).unwrap();
        writeln!(file, "    protocol_id: {},", entry.id).unwrap();
        writeln!(file, "    width: {:.4},", entry.width).unwrap();
        writeln!(file, "    height: {:.4},", entry.height).unwrap();
        writeln!(file, "    eye_height: {:.4},", entry.height * 0.85).unwrap(); // Approx
        writeln!(file, "    summonable: {},", entry.summonable).unwrap();
        writeln!(file, "    fire_immune: {},", entry.fire_immune).unwrap();
        writeln!(
            file,
            "    can_spawn_far_from_player: {},",
            entry.can_spawn_far
        )
        .unwrap();
        writeln!(file, "    category: {},", cat_enum).unwrap();

        // Defaults for now until we extract them fully
        writeln!(file, "    limit_per_chunk: 0,").unwrap();
        writeln!(
            file,
            "    spawn_restriction_location: SpawnLocation::Unrestricted,"
        )
        .unwrap();
        writeln!(file, "    spawn_restriction_heightmap: HeightMap::None,").unwrap();
        writeln!(file, "    max_health: None,").unwrap();
        writeln!(file, "    attackable: true,").unwrap();

        writeln!(file, "}};").unwrap();
        writeln!(file, "").unwrap();

        // Add to name lookup
        // Matches "minecraft:zombie" -> zombie
        name_match_arms.push_str(&format!(
            "        \"{}\" => Some(&{}),\n",
            clean_name, const_name
        ));

        // Add to ID lookup
        // Matches 102 or whatever -> zombie
        id_match_arms.push_str(&format!("        {} => Some(&{}),\n", entry.id, const_name));
    }

    // --- Write Name Lookup Function ---
    writeln!(
        file,
        "/// Looks up EntityData by name (e.g. \"minecraft:zombie\" or \"zombie\")."
    )
    .unwrap();
    writeln!(
        file,
        "pub fn get_entity_by_name(name: &str) -> Option<&'static EntityData> {{"
    )
    .unwrap();
    writeln!(
        file,
        "    // Normalize input to full name if needed, or just match on full name"
    )
    .unwrap();
    let _ = writeln!(file, "    let name = if !name.contains(':') {{ format!(\"minecraft:{{}}\", name) }} else {{ name.to_string() }};");
    writeln!(file, "    match name.as_str() {{").unwrap();
    file.write_all(name_match_arms.as_bytes()).unwrap();
    writeln!(file, "        _ => None,").unwrap();
    writeln!(file, "    }}").unwrap();
    writeln!(file, "}}").unwrap();

    // --- Write ID Lookup Function ---
    writeln!(file, "").unwrap();
    writeln!(file, "/// Looks up EntityData by Protocol ID.").unwrap();
    writeln!(
        file,
        "pub fn get_entity_by_id(id: u32) -> Option<&'static EntityData> {{"
    )
    .unwrap();
    writeln!(file, "    match id {{").unwrap();
    file.write_all(id_match_arms.as_bytes()).unwrap();
    writeln!(file, "        _ => None,").unwrap();
    writeln!(file, "    }}").unwrap();
    writeln!(file, "}}").unwrap();
}

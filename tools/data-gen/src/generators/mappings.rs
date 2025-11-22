use crate::utils;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Deserialize)]
struct MappingReport {
    placement: BTreeMap<String, u32>, // "ItemId": BlockStateId
    lookup: BTreeMap<String, u32>,    // "BlockStateId": ItemId
}

pub fn generate(input_path: &Path, output: &Path) {
    println!("   ... Parsing Mappings from {:?}", input_path);
    let content = fs::read_to_string(input_path).expect("Failed to read mappings.json");
    let report: MappingReport =
        serde_json::from_str(&content).expect("Failed to parse mappings.json");

    let mut file = utils::create_file(output);

    // --- 1. Placement (Item -> Block) ---
    let mut place_arms = String::new();
    for (item_id_str, block_id) in report.placement {
        place_arms.push_str(&format!("        {} => Some({}),\n", item_id_str, block_id));
    }

    writeln!(
        file,
        "/// Maps an Item Protocol ID to a default Block State ID."
    )
    .unwrap();
    writeln!(
        file,
        "pub fn get_block_id_from_item_id(item_id: u32) -> Option<u32> {{"
    )
    .unwrap();
    writeln!(file, "    match item_id {{").unwrap();
    file.write_all(place_arms.as_bytes()).unwrap();
    writeln!(file, "        _ => None,").unwrap();
    writeln!(file, "    }}").unwrap();
    writeln!(file, "}}").unwrap();
    writeln!(file, "").unwrap();

    // --- 2. Lookup (Block -> Item) ---
    let mut lookup_arms = String::new();
    for (block_id_str, item_id) in report.lookup {
        lookup_arms.push_str(&format!("        {} => Some({}),\n", block_id_str, item_id));
    }

    writeln!(
        file,
        "/// Maps a Block State Protocol ID to its corresponding Item Protocol ID."
    )
    .unwrap();
    writeln!(file, "/// Used for 'Pick Block' functionality.").unwrap();
    writeln!(
        file,
        "pub fn get_item_id_from_block_id(block_id: u32) -> Option<u32> {{"
    )
    .unwrap();
    writeln!(file, "    match block_id {{").unwrap();
    file.write_all(lookup_arms.as_bytes()).unwrap();
    writeln!(file, "        _ => None,").unwrap();
    writeln!(file, "    }}").unwrap();
    writeln!(file, "}}").unwrap();
}

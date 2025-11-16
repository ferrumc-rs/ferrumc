use phf_codegen::Map;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

// --- 1. Define strong types for our JSON data ---

#[derive(Deserialize)]
struct BlockStateEntry {
    name: String,
}

#[derive(Deserialize)]
struct ItemEntry {
    protocol_id: i32,
}

// Type for registries.json: "minecraft:item" -> "entries" -> "minecraft:stone" -> ItemEntry
#[derive(Deserialize)]
struct RegistryRoot {
    #[serde(rename = "minecraft:item")]
    item: ItemRegistry,
}

#[derive(Deserialize)]
struct ItemRegistry {
    entries: HashMap<String, ItemEntry>,
}

// --- 2. The Main Build Function ---
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../../assets/data/registries.json");
    println!("cargo:rerun-if-changed=../../../assets/data/blockstates.json");
    println!("cargo:rerun-if-changed=../../../assets/data/item_to_block_mapping.json");

    // --- 3. Load and parse all files ---
    let registry_str = fs::read_to_string("../../../assets/data/registries.json").unwrap();
    let registry: RegistryRoot = serde_json::from_str(&registry_str).unwrap();

    let bs_str = fs::read_to_string("../../../assets/data/blockstates.json").unwrap();
    let blockstates: HashMap<String, BlockStateEntry> = serde_json::from_str(&bs_str).unwrap();

    let i2b_str = fs::read_to_string("../../../assets/data/item_to_block_mapping.json").unwrap();
    let item_to_block: HashMap<String, String> = serde_json::from_str(&i2b_str).unwrap();

    // --- 4. Get the output path ---
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("registry_data.rs");
    let mut file = fs::File::create(&dest_path).unwrap();

    // --- 5. Generate `phf::Map` for ItemName -> Protocol_ID ---
    write!(
        file,
        "static ITEM_NAME_TO_ID: phf::Map<&'static str, i32> = "
    )
    .unwrap();
    let mut item_map = Map::new();
    for (name, entry) in &registry.item.entries {
        // Use & to borrow
        item_map.entry(name, &entry.protocol_id.to_string());
    }
    writeln!(file, "{};\n", item_map.build()).unwrap();

    // --- 5b. Generate reverse map: Protocol_ID -> ItemName ---
    write!(
        file,
        "static ITEM_ID_TO_NAME: phf::Map<i32, &'static str> = "
    )
    .unwrap();
    let mut item_id_map = Map::new();
    for (name, entry) in &registry.item.entries {
        // Here, the key is the i32 protocol_id
        item_id_map.entry(entry.protocol_id, &format!("r#\"{}\"#", name));
    }
    writeln!(file, "{};\n", item_id_map.build()).unwrap();

    // --- 6. Generate `phf::Map` for BlockStateID -> Block Name ---
    write!(
        file,
        "static BLOCKSTATE_ID_TO_NAME: phf::Map<&'static str, &'static str> = "
    )
    .unwrap();
    let mut bs_map = Map::new();
    for (id, data) in blockstates {
        // Use `entry` to build a valid raw string literal for the value
        bs_map.entry(id, &format!("r#\"{}\"#", data.name));
    }
    writeln!(file, "{};\n", bs_map.build()).unwrap();

    // --- 7. Generate `phf::Map` for ItemID (str) -> BlockStateID (str) ---
    write!(
        file,
        "static ITEM_ID_STR_TO_BLOCKSTATE_ID_STR: phf::Map<&'static str, &'static str> = "
    )
    .unwrap();
    let mut i2b_map = Map::new();
    for (item_id_str, block_state_id_str) in item_to_block {
        i2b_map.entry(item_id_str, &format!("r#\"{}\"#", block_state_id_str));
    }
    writeln!(file, "{};\n", i2b_map.build()).unwrap();
}

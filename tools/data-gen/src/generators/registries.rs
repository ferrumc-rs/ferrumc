use crate::utils;
use serde_json::Value;
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn generate(input_path: &Path, output_path: &Path) {
    println!("   ... Parsing Registries from {:?}", input_path);
    let content = fs::read_to_string(input_path).expect("Failed to read registries.json");
    let root: Value = serde_json::from_str(&content).expect("Failed to parse registries.json");

    let mut file = utils::create_file(output_path);

    writeln!(
        file,
        "/// Look up a registry Protocol ID by its dot-separated path."
    )
    .unwrap();
    writeln!(
        file,
        "/// Example: `get_registry_id(\"minecraft:item.entries.minecraft:apple\")`"
    )
    .unwrap();
    writeln!(file, "pub fn get_registry_id(path: &str) -> Option<u32> {{").unwrap();
    writeln!(file, "    match path {{").unwrap();

    // Start recursive traversal
    traverse("", &root, &mut file);

    writeln!(file, "        _ => None,").unwrap();
    writeln!(file, "    }}").unwrap();
    writeln!(file, "}}").unwrap();
}

fn traverse(current_path: &str, value: &Value, file: &mut fs::File) {
    if let Some(obj) = value.as_object() {
        // 1. Check if this node has an ID
        if let Some(id) = obj.get("protocol_id").and_then(|v| v.as_u64()) {
            // Remove leading dot if present
            let path_key = if current_path.starts_with('.') {
                &current_path[1..]
            } else {
                current_path
            };
            // Cast to u32 since protocol IDs rarely exceed 4 billion
            writeln!(file, "        \"{}\" => Some({}),", path_key, id as u32).unwrap();
        }

        // 2. Recurse into children
        for (key, val) in obj {
            if key == "protocol_id" {
                continue;
            }
            let new_path = format!("{}.{}", current_path, key);
            traverse(&new_path, val, file);
        }
    }
}

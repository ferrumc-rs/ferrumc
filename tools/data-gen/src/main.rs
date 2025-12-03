mod generators;
mod setup;
mod utils;

use std::env;
use std::fs;

fn main() {
    println!("Starting FerrumC Data Generator");

    // 1. Setup Paths
    let root_dir = env::current_dir().unwrap();
    let project_root = if root_dir.ends_with("data-gen") {
        root_dir.parent().unwrap().parent().unwrap().to_path_buf()
    } else {
        root_dir
    };

    let temp_dir = project_root.join("temp_gen");
    let output_base = project_root.join("src/lib/registry/src/generated");

    // 2. Prepare (Smart)
    setup::prepare_directories(&temp_dir, &output_base);

    // 3. Vanilla Pipeline (IDs)
    // The functions inside now handle skipping if files exist
    let vanilla_jar = setup::download_server_jar(&temp_dir);
    setup::run_java_generator(&vanilla_jar, &temp_dir);
    let reports_dir = temp_dir.join("generated/reports");

    // 4. Extract Pipeline
    let remapped_jar = setup::prepare_remapped_jar(&temp_dir);
    let physics_json = setup::extract_blocks(&remapped_jar, &temp_dir);
    let mappings_json = setup::extract_mappings(&remapped_jar, &temp_dir);
    let entities_json = setup::extract_entities(&remapped_jar, &temp_dir);

    // 5. Generate Code

    // Blocks
    let blocks_output_file = output_base.join("blocks.rs");

    generators::blocks::generate(
        &reports_dir.join("blocks.json"),
        &physics_json,
        &blocks_output_file,
    );

    // Items
    let items_output_file = output_base.join("items.rs");

    generators::items::generate(
        &reports_dir.join("items.json"),
        &reports_dir.join("registries.json"),
        &items_output_file,
    );

    // Mappings
    println!("Generating Item->Block Mappings...");
    generators::mappings::generate(&mappings_json, &output_base.join("mappings.rs"));

    // Entities
    println!("Generating Entites Mappings...");
    generators::entities::generate(&entities_json, &output_base.join("entities.rs"));

    // Generate Packet IDs
    println!("Generating Packet IDs...");

    let packet_ids_output = project_root.join("src/lib/protocol/src/ids.rs");

    if let Some(parent) = packet_ids_output.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    generators::packets::generate(&reports_dir.join("packets.json"), &packet_ids_output);

    // Generate Registries
    println!("Generating General Registry Lookups...");

    let registry_output = output_base.join("registries.rs");

    generators::registries::generate(&reports_dir.join("registries.json"), &registry_output);

    // Create Mod File
    utils::write_mod_file(&output_base);

    println!("Done! (Cached files kept in temp_gen)");
}

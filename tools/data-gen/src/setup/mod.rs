pub mod decompiler;
pub mod extractors;
pub mod vanilla;

pub use decompiler::prepare_remapped_jar;
pub use extractors::{extract_blocks, extract_entities, extract_mappings};
pub use vanilla::{download_server_jar, run_java_generator};

pub const MC_VERSION: &str = "1.21.8";
pub const MANIFEST_URL: &str = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

use std::fs;
use std::path::Path;

/// Ensures that the temporary and output directories exist.
/// This does NOT delete existing files, allowing for caching.
pub fn prepare_directories(temp_dir: &Path, output_dir: &Path) {
    // 1. Temp Directory (Cache)
    if !temp_dir.exists() {
        println!("Creating temp directory: {:?}", temp_dir);
        fs::create_dir_all(temp_dir).expect("Failed to create temp directory");
    }

    // 2. Output Directory (Generated Code)
    if !output_dir.exists() {
        println!("Creating output directory: {:?}", output_dir);
        fs::create_dir_all(output_dir).expect("Failed to create output directory");
    }
}

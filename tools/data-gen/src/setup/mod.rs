pub mod physics;
pub mod vanilla;

use std::fs;
use std::path::Path;

pub use physics::{extract_physics_data, prepare_remapped_jar};
pub use vanilla::{download_server_jar, run_java_generator};

pub const MC_VERSION: &str = "1.21.1";
pub const MANIFEST_URL: &str = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

pub fn prepare_directories(temp_dir: &Path, output_dir: &Path) {
    // FIX: Only create if missing. Do NOT remove existing.
    if !temp_dir.exists() {
        fs::create_dir_all(temp_dir).unwrap();
    }
    // Output dir we might want to clean to remove stale generated files?
    // Or just ensure it exists. Let's just ensure existence.
    if !output_dir.exists() {
        fs::create_dir_all(output_dir).unwrap();
    }
}

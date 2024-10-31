use std::path::PathBuf;
use tracing::error;
use crate::errors::WorldError;
use crate::World;
use rayon::prelude::*;
use ferrumc_anvil::load_anvil_file;
use ferrumc_nbt::FromNbt;
use crate::vanilla_chunk_format::VanillaChunk;

/// This function is used to check if the import path is valid. It checks if the path exists, if it
/// is a file, if the region folder exists, if the region folder is a file, and if the region folder
/// is empty.
fn check_paths_validity(import_dir: PathBuf) -> Result<(), WorldError> {
    if !import_dir.exists() {
        error!("Import path does not exist: {}", import_dir.to_string_lossy());
        return Err(WorldError::InvalidImportPath(import_dir.to_string_lossy().to_string()));
    }
    if import_dir.is_file() {
        error!("Import path is a file: {}", import_dir.to_string_lossy());
        return Err(WorldError::InvalidImportPath(import_dir.to_string_lossy().to_string()));
    }
    if !import_dir.join("region").exists() {
        error!("Import path does not contain a region folder: {}", import_dir.to_string_lossy());
        return Err(WorldError::InvalidImportPath(import_dir.to_string_lossy().to_string()));
    }
    if import_dir.join("region").is_file() {
        error!("Import path's region folder is a file: {}", import_dir.to_string_lossy());
        return Err(WorldError::InvalidImportPath(import_dir.to_string_lossy().to_string()));
    }
    if let Ok(dir) = import_dir.join("region").read_dir() {
        if dir.count() == 0 {
            error!("Import path's region folder is empty: {}", import_dir.to_string_lossy());
            return Err(WorldError::InvalidImportPath(import_dir.to_string_lossy().to_string()));
        }
    } else {
        error!("Could not read import path's region folder: {}", import_dir.to_string_lossy());
        return Err(WorldError::InvalidImportPath(import_dir.to_string_lossy().to_string()));
    }
    Ok(())
}

impl World {
    // We can actually have this sync since this is run at startup and the program exits after, so
    // no other task are being run. This also makes it easier to work with rayon since rayon doesn't
    // play very nice with async in my experience.
    pub fn import(&self, import_dir: PathBuf, database_dir: PathBuf) -> Result<(), WorldError> {
        // Check if the import path is valid. We can assume the database path is valid since we
        // checked it in the config validity check.
        check_paths_validity(import_dir)?;
        let regions_dir = database_dir.join("region").read_dir()?;
        regions_dir.par_bridge().for_each(|region_file| {
            match region_file {
                Ok(dir_entry) => {
                    if dir_entry.path().is_dir() {
                        error!("Region file is a directory: {}", dir_entry.path().to_string_lossy());
                    } else {
                        let file_path = dir_entry.path();
                        let Ok(anvil_file) = load_anvil_file(file_path) else {
                            error!("Could not load region file: {}", file_path.to_string_lossy());
                            return;
                        };
                        let locations = anvil_file.get_locations();
                        locations.chunks(96).par_bridge().for_each(|sub_locations| {
                            sub_locations.iter().for_each(|location| {
                                if let Some(chunk) = anvil_file.get_chunk_from_location(*location) {
                                    match VanillaChunk::from_bytes(&chunk) {
                                        Ok(vanilla_chunk) => {
                                            self.save_chunk(vanilla_chunk).unwrap()
                                        }
                                        Err(e) => {
                                            error!("Could not convert chunk to vanilla format: {}", e);
                                        }
                                    }
                                }
                            });
                        });
                    }
                }
                Err(e) => {
                    error!("Could not read region file: {}", e);
                }
            }
        });
        Ok(())
    }
}

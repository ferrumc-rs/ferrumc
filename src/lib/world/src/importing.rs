use crate::errors::WorldError;
use crate::vanilla_chunk_format::VanillaChunk;
use crate::World;
use ferrumc_anvil::load_anvil_file;

use ferrumc_threadpool::ThreadPool;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tracing::{error, info};

impl World {
    fn get_chunk_count(&self, import_dir: &Path) -> Result<u64, WorldError> {
        info!("Counting chunks in import directory...");
        let regions_dir = import_dir.join("region").read_dir()?;
        let chunk_count = AtomicU64::new(0);

        regions_dir
            .par_bridge()
            .try_for_each(|region_file| -> Result<(), WorldError> {
                let entry = region_file?;
                if entry.path().is_dir() {
                    return Ok(());
                }

                if let Ok(anvil_file) = load_anvil_file(entry.path()) {
                    chunk_count
                        .fetch_add(anvil_file.get_locations().len() as u64, Ordering::Relaxed);
                }
                Ok(())
            })?;

        Ok(chunk_count.load(Ordering::Relaxed))
    }

    pub fn import(
        &mut self,
        import_dir: PathBuf,
        threadpool: ThreadPool,
    ) -> Result<(), WorldError> {
        check_paths_validity(&import_dir)?;

        let total_chunks = self.get_chunk_count(&import_dir)?;
        let progress_style = ProgressStyle::default_bar()
            .template("[{elapsed_precise}/{eta_precise} eta] {bar:40.cyan/blue} {percent}%, {pos:>7}/{len:7}, {msg}")
            .unwrap();

        let progress = ProgressBar::new(total_chunks);
        progress.set_style(progress_style);

        progress.set_message("Setting up database and preparing import...");

        self.storage_backend.create_table("chunks")?;

        let start = std::time::Instant::now();

        let regions_dir = import_dir.join("region").read_dir()?;

        let mut batch = threadpool.batch();

        let arc_self = Arc::new(self.clone());

        progress.set_message("Importing chunks...");

        for region_result in regions_dir {
            let region_entry = region_result?;
            if region_entry.path().is_dir() {
                continue;
            }

            let anvil_file = match load_anvil_file(region_entry.path()) {
                Ok(file) => file,
                Err(e) => {
                    error!(
                        "Failed to load region file {}: {}",
                        region_entry.path().display(),
                        e
                    );
                    continue;
                }
            };

            let locations = anvil_file.get_locations();
            let location_count = locations.len();

            for (index, location) in locations.iter().enumerate() {
                if let Ok(Some(chunk_data)) = anvil_file.get_chunk_from_location(*location) {
                    if let Ok(vanilla_chunk) = VanillaChunk::from_bytes(&chunk_data) {
                        batch.execute({
                            let self_clone = arc_self.clone();
                            let progress = progress.clone();
                            move || {
                                let res =
                                    self_clone.save_chunk(vanilla_chunk.to_custom_format()?.into());
                                progress.inc(1);
                                if index == location_count - 1 {
                                    self_clone.storage_backend.flush()?;
                                }
                                res
                            }
                        })
                    }
                }
            }
        }

        for result in batch.wait() {
            match result {
                Ok(_) => {}
                Err(e) => {
                    error!("Error saving chunk: {}", e);
                }
            }
        }

        progress.finish_with_message("Import complete");

        arc_self.storage_backend.flush()?;

        info!(
            "Imported {} chunks in {:?}",
            progress.position(),
            start.elapsed()
        );

        Ok(())
    }
}

fn check_paths_validity(import_dir: &Path) -> Result<(), WorldError> {
    if !import_dir.exists() {
        return Err(WorldError::InvalidImportPath(
            import_dir.display().to_string(),
        ));
    }
    if import_dir.is_file() {
        return Err(WorldError::InvalidImportPath(
            import_dir.display().to_string(),
        ));
    }

    let region_dir = import_dir.join("region");
    if !region_dir.exists() || !region_dir.is_dir() {
        return Err(WorldError::InvalidImportPath(
            import_dir.display().to_string(),
        ));
    }

    match region_dir.read_dir() {
        Ok(dir) => {
            if dir.count() == 0 {
                return Err(WorldError::NoRegionFiles);
            }
        }
        Err(_) => {
            return Err(WorldError::InvalidImportPath(
                import_dir.display().to_string(),
            ));
        }
    }

    Ok(())
}

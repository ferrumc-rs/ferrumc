use crate::db_functions::save_chunk_internal_batch;
use crate::errors::WorldError;
use crate::vanilla_chunk_format::VanillaChunk;
use crate::Chunk;
use crate::World;
use ferrumc_anvil::load_anvil_file;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU16, AtomicU32, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use tracing::{error, info};

impl World {
    fn process_chunk_batch(
        &self,
        chunks: Vec<VanillaChunk>,
        progress: Arc<ProgressBar>,
    ) -> Result<(), WorldError> {
        let chunk_objects: Vec<Chunk> = chunks
            .into_iter()
            .filter_map(|chunk| chunk.to_custom_format().ok())
            .collect();

        let mut success_count = 0;
        if let Ok(()) = save_chunk_internal_batch(self, &chunk_objects) {
            success_count = chunk_objects.len();
        }

        progress.inc(success_count.try_into().unwrap());

        Ok(())
    }

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
        batch_size: usize,
        max_concurrent_tasks: usize,
    ) -> Result<(), WorldError> {
        check_paths_validity(&import_dir)?;

        let total_chunks = self.get_chunk_count(&import_dir)?;
        let progress_style = ProgressStyle::default_bar()
            .template("[{elapsed_precise}/{eta_precise} eta] {bar:40.cyan/blue} {percent}%, {pos:>7}/{len:7}, {per_sec}, {msg}")
            .unwrap();

        let progress = Arc::new(ProgressBar::new(total_chunks));
        progress.set_style(progress_style);

        self.storage_backend.create_table("chunks".to_string())?;

        info!("Starting chunk import...");
        let start = std::time::Instant::now();

        let regions_dir = import_dir.join("region").read_dir()?;
        let mut current_batch = Vec::with_capacity(batch_size);

        let remaining_tasks = Arc::new(AtomicU32::new(0));

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

            for location in anvil_file.get_locations() {
                let remaining_tasks_clone = remaining_tasks.clone();
                if let Ok(Some(chunk_data)) = anvil_file.get_chunk_from_location(location) {
                    if let Ok(vanilla_chunk) = VanillaChunk::from_bytes(&chunk_data) {
                        current_batch.push(vanilla_chunk);

                        if current_batch.len() >= batch_size {
                            let batch = std::mem::replace(
                                &mut current_batch,
                                Vec::with_capacity(batch_size),
                            );
                            let progress_clone = Arc::clone(&progress);
                            let self_clone = self.clone();

                            let remaining_tasks_clone = remaining_tasks_clone.clone();

                            std::thread::spawn(move || {
                                remaining_tasks_clone.fetch_add(1, Ordering::Relaxed);
                                if let Err(e) =
                                    self_clone.process_chunk_batch(batch, progress_clone)
                                {
                                    error!("Batch processing error: {}", e);
                                }
                            });

                            progress.set_message(format!(
                                "tasks: {}",
                                remaining_tasks.clone().load(Ordering::Relaxed)
                            ));
                        }
                    }
                }
            }
        }

        if !current_batch.is_empty() {
            let progress_clone = Arc::clone(&progress);
            let self_clone = self.clone();
            let remaining_tasks_clone = remaining_tasks.clone();

            std::thread::spawn(move || {
                remaining_tasks_clone.fetch_add(1, Ordering::Relaxed);
                if let Err(e) = self_clone.process_chunk_batch(current_batch, progress_clone) {
                    error!("Final batch processing error: {}", e);
                }
            });
        }

        while remaining_tasks.load(Ordering::Relaxed) > 0 {
            progress.set_message(format!(
                "tasks: {}",
                remaining_tasks.clone().load(Ordering::Relaxed)
            ));
            thread::sleep(std::time::Duration::from_secs(1));
        }

        self.sync()?;

        progress.finish();

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

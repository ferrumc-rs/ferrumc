use crate::state::GlobalState;
use crate::utils::prelude::*;
use crate::world::chunk_format::Chunk;
use fastanvil::{ChunkData, Region};
use nbt_lib::NBTDeserializeBytes;
use rayon::prelude::{IntoParallelRefMutIterator, ParallelIterator};
use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io::Cursor;
use std::path::PathBuf;
use tokio::sync::{AcquireError};
use tracing::{debug, info, trace, warn};


#[derive(thiserror::Error, Debug)]
pub enum ImportingError {
    SemaphoreError(#[from] AcquireError),
}

mod impl_err {
    use super::*;
    impl Display for ImportingError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                ImportingError::SemaphoreError(e) => write!(f, "Semaphore error: {:?}", e),
            }
        }
    }
}

/// since this is just used to import chunks, it doesn't need to be optimized much
pub async fn import_regions(state: GlobalState) -> Result<()> {
    info!("Starting import...");
    let start = std::time::Instant::now();

    let import_dir = get_import_dir()?;
    debug!("Importing from {}", import_dir.display());

    let mut regions = get_regions(&import_dir).await?;
    info!("Found {} region files", regions.len());

    let chunks = regions
        .par_iter_mut()
        .map(|region| region.iter().filter_map(|x| x.ok()).collect::<Vec<ChunkData>>())
        .map(|chunks| {
            chunks.into_iter().filter_map(|chunk| {
                let mut chunk = Chunk::read_from_bytes(&mut Cursor::new(chunk.data)).ok()?;
                chunk.dimension = Some("overworld".to_string());

                Some(chunk)
            }).collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    info!("Found {} chunks", chunks.iter().map(|x| x.len()).sum::<usize>());

    // Configure the semaphore to allow 8 concurrent tasks
    // let semaphore = Arc::new(Semaphore::new(8));

    insert_chunks(state, chunks.into_iter().flatten().collect::<Vec<_>>()).await?;
    /*let futures = chunks.into_iter().map(|chunk_batch| {
        let state = Arc::clone(&state);
        // let semaphore = Arc::clone(&semaphore);
        tokio::spawn(async move {
            insert_chunks(state as GlobalState, chunk_batch/*, semaphore*/).await
        })
    });

    let mut futures = futures.collect::<futures::stream::FuturesUnordered<_>>();

    while let Some(result) = futures.next().await {
        result??;
    }*/

    info!("Import complete in {:?}", start.elapsed());
    Ok(())
}

const ROOT_ENV: &str = "FERRUMC_ROOT";
fn get_import_dir() -> Result<PathBuf> {
    let path = match env::var(ROOT_ENV) {
        Ok(path) => PathBuf::from(path),
        Err(_) => {
            let exe_path = std::env::current_exe()?;
            exe_path
                .parent()
                .ok_or(Error::Generic("Failed to get exe directory".to_string()))?
                .to_path_buf()
        }
    };

    Ok(path.join("import"))
}
async fn get_regions(import_dir: &PathBuf) -> Result<Vec<Region<File>>> {
    let mut files = tokio::fs::read_dir(import_dir).await?;

    let mut regions = Vec::new();

    while let Some(file_path) = files.next_entry().await? {
        if !file_path.path().is_file() || file_path.path().extension() != Some("mca".as_ref()) {
            continue;
        }
        let file = File::open(file_path.path())?;
        let Ok(region) = Region::from_stream(file) else {
            warn!("(Skipped) Could not read region file: {}", file_path.path().display());
            continue;
        };

        regions.push(region);
    }


    Ok(regions)
}
async fn insert_chunks(state: GlobalState, chunks: Vec<Chunk>/*, semaphore: Arc<Semaphore>*/) -> Result<()> {
    /*let permit = semaphore.acquire().await
        .map_err(|e| Error::from(ImportingError::SemaphoreError(e)))?;*/

    debug!("Inserting {} chunks", chunks.len());

    let size = chunks.len();
    trace!("processed {} chunks. now batch inserting", size);
    state.database.batch_insert(chunks).await?;
    // drop(permit);

    info!("Inserted {} chunks", size);


    Ok(())
}
use std::env;
use std::fs::File;
use std::io::Cursor;
use std::path::PathBuf;
use std::process::exit;
use fastanvil::{Region};
use crate::utils::prelude::*;
use indicatif::ProgressBar;
use nbt_lib::NBTDeserializeBytes;
use rayon::prelude::*;
use tracing::{debug, error, info, trace, warn};

use crate::state::GlobalState;
use crate::world::chunk_format::Chunk;


fn format_time(millis: u64) -> String {
    if millis < 1000 {
        format!("{}ms", millis)
    } else if millis < 60_000 {
        format!("{}s {}ms", millis / 1000, millis % 1000)
    } else if millis < 3_600_000 {
        format!(
            "{}m {}s {}ms",
            millis / 60_000,
            (millis % 60_000) / 1000,
            millis % 1000
        )
    } else {
        format!(
            "{}h {}m {}s {}ms",
            millis / 3_600_000,
            (millis % 3_600_000) / 60_000,
            (millis % 60_000) / 1000,
            millis % 1000
        )
    }
}

async fn get_total_chunks(dir: PathBuf) -> Result<usize> {
    /*let mut region_files = tokio::fs::read_dir(dir).await?;
    let mut total_chunks = 0;
    while let Some(dir_file) = region_files.next_entry().await? {
        let file = std::fs::File::open(dir_file.path())?;
        match fastanvil::Region::from_stream(file).as_mut() {
            Ok(region) => {
                total_chunks += region.iter().count();
            }
            Err(e) => {
                error!(
                    "Could not read region file {}: {}",
                    dir_file.file_name().to_str().unwrap(),
                    e
                );
                exit(1);
            }
        }
    }
    Ok(total_chunks)*/
    let mut files = tokio::fs::read_dir(dir).await?;

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

    let chunks = regions
        .into_par_iter()
        .map(|mut region: Region<File>| region.iter().count())
        .sum();

    Ok(chunks)
        /*.map(|region| region.iter().filter_map(|x| x.ok()).collect::<Vec<ChunkData>>())
        .map(|chunks| {
            chunks.into_iter().filter_map(|chunk| {
                let mut chunk = Chunk::read_from_bytes(&mut Cursor::new(chunk.data)).ok()?;
                chunk.dimension = Some("overworld".to_string());

                Some(chunk)
            }).collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();*/

    // Ok(regions)
}

/// since this is just used to import chunks, it doesn't need to be optimized much
pub async fn import_regions(state: GlobalState) -> Result<()> {
    let dir = if env::var("FERRUMC_ROOT").is_ok() {
        PathBuf::from(env::var("FERRUMC_ROOT").unwrap()).join("import")
    } else {
        PathBuf::from(
            env::current_exe()
                .unwrap()
                .parent()
                .ok_or(Error::Generic("Failed to get exe directory".to_string()))?
                .join("import"),
        )
    };

    debug!("Starting import from {:?}", dir);
    let start = std::time::Instant::now();
    // We aren't, but I can't think of a better way to say "Counting chunks" without it sounding
    // super slow
    info!("Counting chunks...");
    let total_chunks = get_total_chunks(dir.clone()).await?;
    let mut region_files = if tokio::fs::read_dir(dir.clone()).await.is_ok() {
        tokio::fs::read_dir(dir).await?
    } else {
        error!("Could not read the imports directory");
        return Ok(());
    };
    info!("Importing {} chunks", total_chunks);
    info!("This could take a while if there are a lot of chunks to import!");
    let bar = ProgressBar::new(total_chunks as u64);
    while let Some(dir_file) = region_files.next_entry().await? {
        let file = File::open(dir_file.path())?;
        let mut region = Region::from_stream(file)?;

        let mut queued_chunks = Vec::new();
        let mut pending_chunks = 0u8;

        for chunk in region.iter() {
            let Ok(chunk) = chunk else {
                warn!(
                    "Could not read chunk {}",
                    dir_file.file_name().to_str().unwrap()
                );
                bar.abandon_with_message(format!(
                    "Chunk {} failed to import",
                    dir_file.file_name().to_str().unwrap()
                ));
                exit(1);
            };

            let chunk_raw = chunk.data;

            let chunk_nbt = Chunk::read_from_bytes(&mut Cursor::new(chunk_raw));

            if chunk_nbt.is_err() {
                warn!(
                    "Could not read chunk {} {}",
                    chunk_nbt.as_ref().unwrap_err(),
                    dir_file.file_name().to_str().unwrap()
                );
                bar.abandon_with_message(format!(
                    "Chunk {} failed to import",
                    dir_file.file_name().to_str().unwrap()
                ));
                exit(1);
            }
            let mut final_chunk = chunk_nbt.unwrap();
            // final_chunk.convert_to_net_mode().unwrap();
            match final_chunk.convert_to_net_mode() {
                Ok(_) => {}
                Err(e) => {
                    warn!(
                        "Could not convert chunk {} {} to networkmode: {}",
                        final_chunk.x_pos, final_chunk.z_pos, e
                    );
                    bar.abandon_with_message(format!(
                        "Chunk {} {} failed to import",
                        final_chunk.x_pos, final_chunk.z_pos
                    ));
                    exit(1);
                }
            }
            let x = final_chunk.x_pos.clone();
            let z = final_chunk.z_pos.clone();
            final_chunk.dimension = Some("overworld".to_string());

            trace!("Inserting chunk {}, {}", x, z);

            queued_chunks.push(final_chunk);
            pending_chunks += 1;

            if pending_chunks == 150 {
                let res = state.database.batch_insert(queued_chunks).await;
                if res.is_err() {
                    error!(
                        "Could not insert chunk {} {}: {}",
                        x,
                        z,
                        res.as_ref().unwrap_err()
                    );
                    bar.abandon_with_message(format!("Chunk {} {} failed to import", x, z));
                    exit(1);
                }
                queued_chunks = Vec::new();
                pending_chunks = 0;
            }
            bar.inc(1);
            bar.set_message(format!("{} {}", x, z));

            trace!("Imported chunk {} {}", x, z);
        }
        if !queued_chunks.is_empty() {
            let res = state.database.batch_insert(queued_chunks).await;
            if res.is_err() {
                error!(
                    "Could not insert chunk {}: {}",
                    dir_file.file_name().to_str().unwrap(),
                    res.as_ref().unwrap_err()
                );
                bar.abandon_with_message(format!(
                    "Chunk {} failed to import",
                    dir_file.file_name().to_str().unwrap()
                ));
                exit(1);
            }
        }
    }
    bar.abandon_with_message(format!("{} chunks imported!", total_chunks));
    info!(
        "Imported {} chunks in {} seconds",
        total_chunks,
        format_time(start.elapsed().as_millis() as u64)
    );
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::create_state;
    use crate::utils::setup_logger;
    use tokio::net::TcpListener;
    use crate::utils::prelude::*;

    #[tokio::test]
    #[ignore]
    async fn get_chunk_at() -> Result<()> {
        // set environment variable "FERRUMC_ROOT" to the root of the ferrumc project
        setup_logger()?;
        let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
        let state = create_state(listener).await.unwrap();

        let chunk = state
            .database
            .get_chunk(0, 0, "overworld".to_string())
            .await?
            .unwrap();

        println!("{:#?}", chunk);

        Ok(())
    }
}



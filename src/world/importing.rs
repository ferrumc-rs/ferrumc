use std::path::PathBuf;

use tracing::{error, info, warn};

use crate::state::GlobalState;
use crate::world::chunkformat::Chunk;

/// since this is just used to import chunks, it doesn't need to be optimized much
pub async fn import_regions(
    dir: PathBuf,
    state: GlobalState,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut region_files = if tokio::fs::read_dir(dir.clone()).await.is_ok() {
        tokio::fs::read_dir(dir).await?
    } else {
        error!("Could not read the imports directory");
        return Ok(());
    };
    while let Some(dirfile) = region_files.next_entry().await? {
        let file = std::fs::File::open(dirfile.path())?;
        let mut region = fastanvil::Region::from_stream(file)?;
        for chunk in region.iter() {
            let chunk = chunk?.data;
            let chunk_nbt: Chunk = fastnbt::from_bytes(&chunk)?;
            let x = chunk_nbt.x_pos.clone();
            let z = chunk_nbt.z_pos.clone();
            let record = state
                .read()
                .await
                .database
                .insert_chunk(chunk_nbt)
                .await
                .unwrap();

            match record {
                false => {
                    info!("Chunk {} {} added to database", x, z);
                }
                true => {
                    warn!("Could not add chunk {} {} to database", x, z);
                }
            }
        }
        info!(
            "Finished importing region file {}",
            dirfile.file_name().to_str().unwrap()
        );
    }
    Ok(())
}

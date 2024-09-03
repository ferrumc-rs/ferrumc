use tracing::debug;

use crate::state::GlobalState;
use crate::utils::binary_utils::read_n_bits_u16;
use crate::utils::error::Error;

pub async fn read_block(
    state: GlobalState,
    x: i32,
    y: i32,
    z: i32,
    dimension: String,
) -> Result<String, Error> {
    let (chunk_x, chunk_z) = (x / 16, z / 16);
    debug!("Getting chunk: {} {}", chunk_x, chunk_z);
    let chunk = state
        .database
        .get_chunk(chunk_x, chunk_z, dimension)
        .await?;
    if !chunk.is_some() {
        return Err(Error::ChunkNotFound(chunk_x, chunk_z));
    }
    let chunk = chunk.unwrap();
    if !chunk.sections.is_some() {
        return Err(Error::Generic(format!(
            "Chunk {} {} does not have any sections",
            chunk_x, chunk_z
        )));
    }
    let section = chunk
        .sections
        .as_ref()
        .unwrap()
        .iter()
        .find(|section| section.y == (y / 16) as i8)
        .unwrap();

    if !section.block_states.as_ref().unwrap().palette.is_some() {
        return Err(Error::Generic(format!(
            "Section {} does not have any palette",
            y / 16
        )));
    }

    let palette = section
        .block_states
        .as_ref()
        .unwrap()
        .palette
        .as_ref()
        .unwrap();
    // If the palette only has one block, we can just return that
    if palette.len() == 1 {
        return Ok(section
            .block_states
            .as_ref()
            .unwrap()
            .palette
            .as_ref()
            .unwrap()[0]
            .name
            .clone());
    }
    println!("Palette: {:#?}", palette);
    if !section.block_states.is_some() {
        return Err(Error::Generic(format!(
            "Section {} does not have any block states",
            y / 16
        )));
    }
    if !section.block_states.as_ref().unwrap().data.is_some() {
        return Err(Error::Generic(format!(
            "Section {} does not have any block states data",
            y / 16
        )));
    }
    let bits_per_block = section
        .block_states
        .as_ref()
        .unwrap()
        .data
        .as_ref()
        .unwrap()
        .len()
        * 64
        / 4096;

    let index = (y % 16) * 256 + (z % 16) * 16 + (x % 16);
    let specific_index = (index * bits_per_block as i32) / 64;
    if let Some(target_long) = &section
        .block_states
        .as_ref()
        .unwrap()
        .data
        .as_ref()
        .unwrap()
        .get(specific_index as usize)
    {
        let block_index = read_n_bits_u16(
            *target_long,
            (index as usize * bits_per_block) % 64,
            bits_per_block,
        )?;
        Ok(palette[block_index as usize].name.clone())
    } else {
        Err(Error::Generic(format!(
            "Could not find block at index {}",
            index
        )))
    }
}

#[cfg(test)]
mod tests {
    use tokio::net::TcpListener;
    use tracing::info;

    use crate::utils::setup_logger;
    use crate::world::blocks::read_block;

    #[tokio::test]
    async fn test_reading() {
        setup_logger().unwrap();
        let state = crate::create_state(TcpListener::bind("0.0.0.0:0").await.unwrap())
            .await
            .unwrap();
        info!(
            "{}",
            read_block(state, -537, 69, 51, "overworld".to_string())
                .await
                .unwrap()
        );
    }
}

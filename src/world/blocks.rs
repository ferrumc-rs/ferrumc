use tracing::info;

use crate::state::GlobalState;
use crate::utils::error::Error;

pub async fn read_block(
    state: GlobalState,
    x: i32,
    y: i32,
    z: i32,
    dimension: String,
) -> Result<String, Error> {
    let (chunk_x, chunk_z) = (x / 16, z / 16);
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
    let first = section
        .block_states
        .as_ref()
        .unwrap()
        .data
        .as_ref()
        .unwrap()
        .get(0)
        .unwrap();
    let bit_per_block = first.to_be_bytes().get(0).unwrap().clone();

    info!("bit_per_block: {}", bit_per_block);

    Ok("balls".to_string())
}

#[cfg(test)]
mod tests {
    use tokio::net::TcpListener;

    use crate::utils::setup_logger;
    use crate::world::blocks::read_block;

    #[tokio::test]
    async fn test_reading() {
        setup_logger().unwrap();
        let state = crate::create_state(TcpListener::bind("0.0.0.0:0").await.unwrap())
            .await
            .unwrap();
        read_block(state, 0, 50, 0, "overworld".to_string())
            .await
            .unwrap();
    }
}

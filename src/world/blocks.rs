use tracing::info;

use crate::state::GlobalState;
use crate::utils::error::Error;
use crate::world::chunkformat::Chunk;

pub async fn read_block(
    chunk: &Chunk,
    x: i32,
    y: i32,
    z: i32,
) -> Result<String, Error> {
    let (chunk_x, chunk_z) = (x / 16, z / 16);
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
        let chunk = state
            .database
            .get_chunk(0, 0, "overworld")
            .await
            .unwrap()
            .unwrap();
        read_block(&chunk, 0, 50, 0, "overworld").await.unwrap();
    }
}

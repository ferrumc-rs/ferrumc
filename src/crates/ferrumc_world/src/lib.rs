use std::io::Cursor;

use simdnbt::owned::Nbt;

use ferrumc_utils::error::Error;

#[cfg(test)]
mod tests {
    use crate::load_chunk;

    #[tokio::test]
    async fn test_load_chunks() {
        assert!(load_chunk(0, 0).await.is_ok());
        assert!(load_chunk(-5000, 4025).await.is_err());
        assert_eq!(
            load_chunk(0, 0)
                .await
                .unwrap()
                .data
                .unwrap()
                .get("yPos")
                .unwrap()
                .int()
                .unwrap(),
            -4
        );
    }
}

pub struct Chunk {
    pub x: i32,
    pub z: i32,
    pub data: Nbt,
}

pub async fn load_chunk(x: i32, z: i32) -> Result<Chunk, Error> {
    // TODO: Replace with database call when that is all set up
    let region_area = (
        (x as f64 / 32.0).floor() as i32,
        (z as f64 / 32.0).floor() as i32,
    );
    let region_file = std::fs::File::open("dummyregion.mca")?;
    let mut region = fastanvil::Region::from_stream(region_file).unwrap();
    let raw_chunk_data = region
        .read_chunk(x as usize, z as usize)
        .map_err(|e| {
            Error::Generic(format!(
                "Unable to read chunk {} {} from region {} {} ",
                x, z, region_area.0, region_area.1
            ))
        })?
        .expect(
            format!(
                "Chunk {} {} not found in region {} {}",
                x, z, region_area.0, region_area.1
            )
            .as_str(),
        );
    let decoded_chunk = simdnbt::owned::read(&mut Cursor::new(&*raw_chunk_data)).unwrap();
    Ok(Chunk {
        x,
        z,
        data: decoded_chunk,
    })
}


use crate::utils::error::Error;
use crate::world::chunkformat::Chunk;

pub(crate) mod chunkformat;
pub mod importing;
pub mod sweattypalms_impl;
#[cfg(test)]
mod tests {
    use std::io::Write;

    use fastnbt::Value;

    use crate::world::load_chunk;

    #[tokio::test]
    async fn dump_region_to_json() {
        let f = std::fs::File::open("./dummyregion.mca").unwrap();
        let mut reader = fastanvil::Region::from_stream(f).unwrap();
        let chunk = reader.read_chunk(0, 0).unwrap().unwrap();
        let chunk_nbt: Value = fastnbt::from_bytes(&chunk).unwrap();
        let mut outfile = std::fs::File::create("chunk.json").unwrap();
        let raw_nbt = serde_json::ser::to_vec(&chunk_nbt).unwrap();
        outfile.write_all(&*raw_nbt).unwrap()
    }

    #[tokio::test]
    async fn chunk_to_struct() {
        let chunk = load_chunk(0, 0).await.unwrap();
        assert_eq!(chunk.x_pos, 0);
        assert_eq!(chunk.z_pos, 0);
        assert_eq!(chunk.y_pos, -4);
    }
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
        .map_err(|_| {
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
    fastnbt::from_bytes(&raw_chunk_data).map_err(|_| {
        Error::Generic(format!(
            "Unable to parse chunk {} {} from region {} {} ",
            x, z, region_area.0, region_area.1
        ))
    })
}

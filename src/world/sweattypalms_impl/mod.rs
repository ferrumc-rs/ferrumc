pub mod types;

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Cursor;

    use fastanvil::Region;
    use simdnbt::borrow::read;
    use simdnbt::Deserialize;

    use crate::utils::prelude::*;
    use crate::world::sweattypalms_impl::types::Chunk;

    // NOTE: Will be slower than `release` build. (Approx 10x slower LMAO)
    #[test]
    fn test_all_regions() -> Result<()> {
        let start = std::time::Instant::now();

        let mut chunks = Vec::<Chunk>::new();
        let mut region = get_region("test_region.mca")?;

        region.iter().for_each(|item| {
            let chunk_data = item.expect("Failed to read chunk data");
            let nbt = read(&mut Cursor::new(&chunk_data.data))
                .expect("Failed to parse chunk data^1")
                .unwrap();
            let data = Chunk::from_nbt(&nbt).expect("Failed to parse chunk data^2");
            chunks.push(data);
        });

        println!("There are {} chunks in the region file", chunks.len());
        println!(
            "Time taken to read and parse all chunks: {:?}",
            start.elapsed()
        );

        Ok(())
    }

    #[test]
    fn test_one_chunk() -> Result<()> {
        let start = std::time::Instant::now();
        let mut region = get_region("test_region.mca")?;
        let chunk_data = region
            .read_chunk(15, 30)
            .expect("Failed to read chunk data")
            .unwrap();
        let nbt = read(&mut Cursor::new(&chunk_data))
            .expect("Failed to parse chunk data^1")
            .unwrap();
        Chunk::from_nbt(&nbt).expect("Failed to parse chunk data^2");

        // println!("{:?}", data);
        println!(
            "Time taken to read and parse one chunk: {:?}",
            start.elapsed()
        );

        Ok(())
    }

    fn get_region(file: &str) -> Result<Region<File>> {
        let region_file = File::open(file)?;
        let reader = Region::from_stream(region_file)?;

        Ok(reader)
    }
}

use std::collections::HashMap;
use std::io::BufReader;
use fastanvil::Region;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::Value;
use crate::world::{nbtstructs};
use crate::world::nbtstructs::{Chunk, SeriableRegion};


#[allow(dead_code)]
#[derive(Debug, serde::Deserialize)]
struct GenericData {

    #[serde(flatten, rename = "Data")]
    data: HashMap<String, Value>
}
#[test]
fn test_world() {
    let file = std::fs::File::open("C:\\Users\\ReCor\\Documents\\Paper\\world\\level.dat").unwrap();
    let bufreader = BufReader::new(file);
    let reader = flate2::read::MultiGzDecoder::new(bufreader);
    let _: nbtstructs::WorldData = fastnbt::from_reader(reader).unwrap();
}

#[test]
fn load_region() {
    let file = std::fs::File::open("C:\\Users\\ReCor\\Documents\\Paper\\world\\region\\r.0.0.mca").unwrap();
    let mut region = Region::from_stream(file).unwrap();
    for chunk in region.iter().flatten() {
        println!("Chunk {}, {} loaded", chunk.x, chunk.z);
    }
}



#[test]
pub fn read_chunks() {
    let start = std::time::Instant::now();
    let mut last = std::time::Instant::now();
    let file = std::fs::File::open("C:\\Users\\ReCor\\Documents\\Paper\\world\\region\\r.0.0.mca").unwrap();
    let buf = BufReader::new(file);
    println!("File load time: {:?}", last.elapsed());
    last = std::time::Instant::now();
    let mut region = Region::from_stream(buf).unwrap();
    let chunks: Vec<Chunk> = region.iter().flatten().map(|chunk_data| fastnbt::from_bytes::<Chunk>(&*chunk_data.data).unwrap()).collect();

    // region.iter().for_each(
    //     |chunk_data|
    //         {
    //             match chunk_data {
    //                 Ok(chunk_data) => { fastnbt::from_bytes::<Chunk>(&*chunk_data.data).unwrap(); () }
    //                 Err(_) => ()
    //             }
    //         }
    // );
    println!("Decode time: {:?}", last.elapsed());
    last = std::time::Instant::now();
    let serial_region = SeriableRegion { chunks };
    let mut serial_buffer = flexbuffers::FlexbufferSerializer::new();
    serial_region.serialize(&mut serial_buffer).unwrap();
    println!("Serialize time: {:?}", last.elapsed());
    last = std::time::Instant::now();
    let compressed = lz4_flex::compress(&serial_buffer.view());
    println!("Compressed size: {}", compressed.len());
    println!("Uncompressed size: {}", serial_buffer.take_buffer().len());
    println!("Compress time: {:?}", last.elapsed());
    println!("Time taken: {:?}", start.elapsed());
}
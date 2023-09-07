use std::collections::HashMap;
use std::io::{Cursor, Write};
use fastanvil::{ChunkData, complete, CurrentJavaChunk, Error, JavaChunk, Region};
use fastnbt::{from_bytes, LongArray};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::io::{BufStream, BufWriter};

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize)]
struct GenericData {

    #[serde(flatten, rename = "Data")]
    data: HashMap<String, Value>
}
#[test]
fn test_world() {
    let file = std::fs::File::open("C:\\Users\\ReCor\\Documents\\Paper\\world\\level.dat").unwrap();
    let bufreader = std::io::BufReader::new(file);
    let reader = flate2::read::MultiGzDecoder::new(bufreader);
    let _: crate::world::nbtstructs::WorldData = fastnbt::from_reader(reader).unwrap();
}

#[test]
fn load_region() {
    let file = std::fs::File::open("C:\\Users\\ReCor\\Documents\\Paper\\world\\region\\r.0.0.mca").unwrap();
    // let bufreader = std::io::BufReader::new(file);
    // let reader = flate2::read::GzDecoder::new(bufreader);
    let mut region = fastanvil::Region::from_stream(file).unwrap();
    for chunk in region.iter().flatten() {
        println!("Chunk {}, {} loaded", chunk.x, chunk.z);
    }
}

#[derive(Serialize, Deserialize)]
struct Chunk {
    sections: Vec<Section>,

    #[serde(flatten)]
    other: HashMap<String, Value>,
}


#[derive(Serialize, Deserialize)]
struct Section {
    block_states: Option<Blockstates>,
    #[serde(flatten)]
    other: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct Blockstates {
    palette: Vec<PaletteItem>,
    data: Option<LongArray>,
    #[serde(flatten)]
    other: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct PaletteItem {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Properties")]
    properties: Option<Value>,
}



#[test]
fn read_chunks() {
    let buf = Cursor::new(Vec::new());
    let file = std::fs::File::open("C:\\Users\\ReCor\\Documents\\Paper\\world\\region\\r.0.0.mca").unwrap();
    let mut dumpfile = std::fs::File::options().write(true).open("C:\\Users\\ReCor\\Documents\\Code\\Rust\\ferrumc\\dump.json").unwrap();
    let mut newregion = Region::new(buf).unwrap();
    let mut region = Region::from_stream(file).unwrap();
    for chunkdata in region.iter().flatten() {
        let chunk = fastnbt::from_bytes::<fastanvil::CurrentJavaChunk>(&*chunkdata.data).unwrap();
        println!("Status: {}", chunk.status);

    }
}
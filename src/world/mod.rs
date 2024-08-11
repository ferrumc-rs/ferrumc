use serde_derive::{Deserialize, Serialize};

use crate::state::GlobalState;
use crate::utils::error::Error;
use crate::world::chunkformat::Chunk;

pub mod chunkformat;
pub mod importing;
pub mod sweattypalms_impl;

#[derive(Deserialize, Serialize)]
pub struct ProtoChunk {
    pub x: i64,
    pub z: i64,
    pub data: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use fastnbt::Value;

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
}

pub async fn load_chunk(_state: GlobalState, x: i32, z: i32) -> Result<Chunk, Error> {
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

/// Since we don't know the exact amount of bytes, the first byte is the number of u8s in the last i64,
/// so we know when to stop reading bytes from the last i64
pub async fn encode_bytes_to_i64(bytes: Vec<u8>) -> Vec<i64> {
    let remaining = bytes.len() % 8;
    let mut i64s = Vec::new();
    let mut u8s_per_i64 = 0;
    let mut i64 = 0;
    i64s.push(remaining as i64);
    for u8 in bytes.iter().take(bytes.len() - remaining) {
        i64 |= *u8 as i64;
        u8s_per_i64 += 1;
        if u8s_per_i64 == 8 {
            i64s.push(i64);
            i64 = 0;
            u8s_per_i64 = 0;
        } else {
            i64 <<= 8;
        }
    }
    i64s
}

pub async fn decode_i64_to_bytes(i64s: Vec<i64>) -> Vec<u8> {
    let mut bytes = Vec::new();
    let remaining = i64s[0] as usize;
    for i64 in i64s.iter().skip(1) {
        let mut i64 = *i64;
        for _ in 0..8 {
            let byte = (i64 & 0xFF) as u8;
            bytes.push(byte);
            i64 >>= 8;
        }
    }
    bytes.truncate(bytes.len() - remaining);
    bytes
}

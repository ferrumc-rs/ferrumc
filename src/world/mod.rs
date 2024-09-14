pub mod blocks;
pub mod chunk_format;
pub mod conversions;
pub mod importing;

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

#[cfg(test)]
mod tests {
    use std::io::Write;

    use fastnbt::Value;

    #[tokio::test]
    #[ignore]
    async fn dump_region_to_json() {
        let f = std::fs::File::open("./dummyregion.mca").unwrap();
        let mut reader = fastanvil::Region::from_stream(f).unwrap();
        let chunk = reader.read_chunk(0, 0).unwrap().unwrap();
        let chunk_nbt: Value = fastnbt::from_bytes(&chunk).unwrap();
        let mut outfile = std::fs::File::create("chunk.json").unwrap();
        let raw_nbt = serde_json::ser::to_vec(&chunk_nbt).unwrap();
        outfile.write_all(&raw_nbt).unwrap()
    }
}

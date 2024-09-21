#![cfg(test)]

#[test]
fn basic_parsing() {
    let data: Vec<u8> = vec![
        10, 0, 3, b'H', b'i', b'i', // TagCompound("Hii")
        1, 0, 3, b'K', b'e', b'y', 1, // TagByte("Key", 1)
        3, 0, 3, b'd', b'a', b't', 0, 0, 0, 0, // TagInt("dat", 0)
        0, // End
    ];

    let mut parser = crate::de::borrow::NbtTape::new(data.as_slice());
    parser.parse();
}

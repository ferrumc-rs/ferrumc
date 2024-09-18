#[allow(unused)]
fn coming_soon() {
    unimplemented!(" :) ")
}

#[cfg(test)]
#[test]
fn test() {
    let some_test_nbt: [u8; 21] = [
        10, 0, 2, b'H', b'i', // compound: (2) "Hi"
        8, 0, 3, b'I', b'\'', b'm', 0, 7, b'f', b'e', b'r', b'r', b'u', b'm',
        b'c', // string: (3) "I'm" (7) "ferrumc"
        0,    // End tag
    ];

    let mut nbt = ferrumc_nbt::de::NbtParser::new(&some_test_nbt);
    let tapes = nbt.parse().unwrap();

    let viewer = ferrumc_nbt::de::NbtTokenView::new(tapes, 0);

    let compound = viewer.as_compound().expect("Expected a compound");

    let hi = compound.get("I'm").expect("Expected a key named 'Hi'");

    let value = hi.value().unwrap();
    if let ferrumc_nbt::de::NbtToken::String(value) = value {
        assert_eq!(*value, "ferrumc")
    } else {
        panic!("Expected a string")
    }
}

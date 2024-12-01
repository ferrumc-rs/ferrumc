use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};

#[test]
fn hashmaps() {
    let map = maplit::hashmap! {
        "key1".to_string() => "value1".to_string(),
        "key2".to_string() => "value2".to_string(),
        "key3".to_string() => "value3".to_string(),
        "key4".to_string() => "value4".to_string(),
    };

    let encoded = {
        let mut buffer = Vec::new();
        map.encode(&mut buffer, &NetEncodeOpts::None).unwrap();

        buffer
    };

    let decoded = {
        let mut buffer = encoded.as_slice();
        std::collections::HashMap::<String, String>::decode(&mut buffer, &NetDecodeOpts::None)
            .unwrap()
    };

    assert_eq!(map, decoded);
}

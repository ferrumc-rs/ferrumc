use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};

#[test]
fn hashmaps() {
    let mut map = std::collections::HashMap::new();
    map.insert("key".to_string(), "value".to_string());
    
    let encoded = {
        let mut buffer = Vec::new();
        map.encode(&mut buffer, &NetEncodeOpts::None).unwrap();
        
        buffer
    };

    let decoded = {
        let mut buffer = encoded.as_slice();
        std::collections::HashMap::<String, String>::decode(&mut buffer, &NetDecodeOpts::None).unwrap()
    };
    
    assert_eq!(map, decoded);
}
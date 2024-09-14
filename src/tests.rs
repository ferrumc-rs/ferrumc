mod chunk_stuff;
mod nbt_de;
mod nbt_ser;
pub mod query;

use std::io::Cursor;

use ferrumc_codec::network_types::varint::VarInt;

use ferrumc_macros::NetDecode;

#[tokio::test]
async fn test_macro_decode() {
    #[derive(NetDecode, Default)]
    struct Handshake {
        protocol_version: VarInt,
        server_address: String,
        server_port: u16,
        next_state: VarInt,
    }
    let mut data = Cursor::new(vec![
        0xFB, 0x05, 0x09, 0x31, 0x32, 0x37, 0x2E, 0x30, 0x2E, 0x30, 0x2E, 0x31, 0x63, 0xDD, 0x01,
    ]);
    let handshake = Handshake::net_decode(&mut data).await.unwrap();
    assert_eq!(handshake.protocol_version, VarInt::new(763));
    assert_eq!(handshake.server_address, "127.0.0.1".to_string());
    assert_eq!(handshake.server_port, 25565);
    assert_eq!(handshake.next_state, VarInt::new(1));
}
/*
#[tokio::test]
async fn test_nbt_decode() {
    #[derive(Serialize, Deserialize, Clone, Debug)]
    struct Test {
        test: i32,
        nested: Option<Nested>,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    struct Nested {
        second_test: i8,
    }

    let structed = Test {
        test: 1,
        nested: Some(Nested { second_test: 2 }),
        // nested: None,
    };

    let data = fastnbt::to_bytes(&structed).unwrap();

    // let decoded = Test::decode(data).unwrap();
    // println!("{:?}", decoded);

    let simdnbt_read = simdnbt::borrow::read(&mut std::io::Cursor::new(data.as_slice()))
        .unwrap()
        .unwrap();

    let test = <i32 >::decode_from_base(&simdnbt_read, "test").unwrap();
    /*let compound = simdnbt_read.compound("nested");
    let nested: Option<Nested> = match compound {
        Some(compound) => <Option<Nested> as NBTDecodable>::decode_from_compound(&compound, "").unwrap(),
        None => <Option<Nested> as Default>::default(),
    };*/
    let nested = match simdnbt_read.compound("nested") {
        Some(compound) => {
            <Option<Nested> as NBTDecodable>::decode_from_compound(&compound, "").unwrap()
        }
        None => <Option<Nested> as Default>::default(),
    };

    let decoded = Test { test, nested };

    println!("{:?}", decoded);

    /*if compound.is_none() {
        nested = <Option<Nested> as Default>::default();
    }else */
    /*let nested = <Option<Nested> as NBTDecodable>::decode_from_compound(&simdnbt_read, "nested").unwrap();*/
}*/

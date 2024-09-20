use ferrumc_nbt::{NBTSerializable, NBTSerializeOptions, NbtToken, NbtTokenViewExt};
use std::collections::HashMap;

const BASE_PATH: &str = env!("CARGO_MANIFEST_DIR");

#[test]
fn basic_compound_ser() {
    let mut map = HashMap::new();
    map.insert("hello".to_string(), 42);

    let mut buf = Vec::new();
    map.serialize(&mut buf, &NBTSerializeOptions::WithHeader("test"));

    let mut parser = ferrumc_nbt::NbtParser::new(&buf);
    let tokens = parser.parse().unwrap().to_viewer();
    let compound = tokens.as_compound().unwrap();
    let hello = compound.get("hello").unwrap();
    let value = hello.value().unwrap();


    assert_eq!(value, &NbtToken::Int(42));
}

#[test]
fn derive_macro() {
    use ferrumc_macros::NBTSerialize;
    use ferrumc_nbt::NBTSerializable;

    #[derive(NBTSerialize)]
    struct Test {
        hello: i32,
        world: i32,
    }

    let test = Test {
        hello: 1,
        world: 2,
    };

    let mut buf = Vec::new();
    // test.serialize(&mut buf, &ferrumc_nbt::NBTSerializeOptions::WithHeader("test"));
    test.serialize_with_header(&mut buf);

    let mut parser = ferrumc_nbt::NbtParser::new(&buf);

    println!("{:?}", parser.parse().unwrap());
}

#[test]
fn derive_macro_nested() {
    use ferrumc_macros::NBTSerialize;
    use ferrumc_nbt::NBTSerializable;

    #[derive(NBTSerialize)]
    struct Test {
        hello: i32,
        world: i32,
    }

    #[derive(NBTSerialize)]
    struct Test2 {
        test: Test,
    }

    let test = Test {
        hello: 1,
        world: 2,
    };

    let test2 = Test2 {
        test,
    };

    let mut buf = Vec::new();
    test2.serialize_with_header(&mut buf);

    let path = format!("{}/{}", BASE_PATH, "nested_compound.nbt");

    std::fs::write(path, buf).unwrap();
}
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

    let test = Test { hello: 1, world: 2 };

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

    let test = Test { hello: 1, world: 2 };

    let test2 = Test2 { test };

    let mut buf = Vec::new();
    test2.serialize_with_header(&mut buf);

    let mut parser = ferrumc_nbt::NbtParser::new(&buf);

    let tapes = parser.parse().unwrap();
    let root = ferrumc_nbt::NbtCompoundView::new(tapes, 0);
    let test = root.get("test").unwrap();
    let test = test.as_compound().unwrap();
    let hello = test.get("hello").unwrap();
    let world = test.get("world").unwrap();

    assert_eq!(hello.value().unwrap(), &NbtToken::Int(1));
    assert_eq!(world.value().unwrap(), &NbtToken::Int(2));

    dbg!(hello.value());
    dbg!(world.value());
}

#[test]
fn derive_macro_nested_with_list() {
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
        list: Vec<i32>,
        another_list: Vec<Test>,
    }

    let test = Test { hello: 1, world: 2 };

    let test2 = Test2 {
        test,
        list: vec![1, 2, 3],
        another_list: vec![
            Test { hello: 1, world: 2 },
            Test { hello: 3, world: 4 },
            Test { hello: 5, world: 6 },
        ],
    };

    let mut buf = Vec::new();
    test2.serialize_with_header(&mut buf);

    let mut parser = ferrumc_nbt::NbtParser::new(&buf);
    let tapes = parser.parse().unwrap();

    let root = ferrumc_nbt::NbtCompoundView::new(tapes, 0);
    let test = root.get("test").unwrap();
    let test = test.as_compound().unwrap();
    let hello = test.get("hello").unwrap();
    let world = test.get("world").unwrap();

    assert_eq!(hello.value().unwrap(), &NbtToken::Int(1));
    assert_eq!(world.value().unwrap(), &NbtToken::Int(2));

    let list = root.get("list").unwrap();
    let list = list.as_list().unwrap();
    // let list: &[u8] = list.get_appropriate_list();
    todo!()
}

use ferrumc_macros::NBTDeserialize;
use ferrumc_nbt::{FromNbt, NBTSerializable, NBTSerializeOptions};
use std::collections::HashMap;

#[test]
fn basic_compound_ser() {
    let mut map = HashMap::new();
    map.insert("hello".to_string(), 42);

    let mut buf = Vec::new();
    map.serialize(&mut buf, &NBTSerializeOptions::WithHeader("test"));
}

#[test]
fn derive_macro() {
    use ferrumc_macros::NBTSerialize;

    #[derive(NBTSerialize)]
    struct Test {
        hello: i32,
        world: i32,
        some_list: Vec<i32>,
    }

    let test = Test {
        hello: 1,
        world: 2,
        some_list: vec![1, 2, 3],
    };

    let buf = test.serialize_with_header();

    let mut parser = ferrumc_nbt::de::borrow::NbtTape::new(&buf);
    parser.parse();

    let some_list = parser.get("some_list").unwrap();
    // let some_list : &[i32] = parser.unpack_list_sliced(some_list).unwrap();
    let some_list: Vec<i32> = parser.unpack_list(some_list).unwrap();

    assert_eq!(some_list, vec![1, 2, 3]);
}

#[test]
fn derive_macro_nested() {
    use ferrumc_macros::NBTSerialize;

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

    let buf = test2.serialize_with_header();

    let mut parser = ferrumc_nbt::de::borrow::NbtTape::new(&buf);
    parser.parse();

    let test = parser.get("test").unwrap();
    let hello = test.get("hello").unwrap();
    let world = test.get("world").unwrap();

    let hello = <i32 as FromNbt>::from_nbt(&parser, hello).unwrap();
    let world = <i32 as FromNbt>::from_nbt(&parser, world).unwrap();

    assert_eq!(hello, 1);
    assert_eq!(world, 2);
}

#[test]
fn derive_macro_nested_with_list() {
    use ferrumc_macros::NBTSerialize;

    #[derive(NBTSerialize, Debug, PartialEq, NBTDeserialize)]
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

    let buf = test2.serialize_with_header();

    let mut parser = ferrumc_nbt::de::borrow::NbtTape::new(&buf);
    parser.parse();

    let test = parser.get("test").unwrap();
    let hello = test.get("hello").unwrap();
    let world = test.get("world").unwrap();
    let list = parser.get("list").unwrap();
    let another_list = parser.get("another_list").unwrap();

    let hello = <i32 as FromNbt>::from_nbt(&parser, hello).unwrap();
    let world = <i32 as FromNbt>::from_nbt(&parser, world).unwrap();
    let list = <Vec<i32> as FromNbt>::from_nbt(&parser, list).unwrap();
    let another_list = <Vec<Test> as FromNbt>::from_nbt(&parser, another_list).unwrap();

    assert_eq!(hello, 1);
    assert_eq!(world, 2);
    assert_eq!(list, vec![1, 2, 3]);
    assert_eq!(
        another_list,
        vec![
            Test { hello: 1, world: 2 },
            Test { hello: 3, world: 4 },
            Test { hello: 5, world: 6 },
        ]
    );
}

#[test]
fn very_basic_derive() {
    use ferrumc_macros::NBTSerialize;

    // Define the struct
    #[derive(NBTSerialize, NBTDeserialize, PartialEq, Debug)]
    struct Test {
        hello: i32,
        world: i32,
    }

    // Create the struct
    let test = Test { hello: 1, world: 2 };

    // Serialize the struct
    let buf = test.serialize_with_header();

    // Deserialize the struct
    let test = Test::from_bytes(&buf).unwrap();

    assert_eq!(test, Test { hello: 1, world: 2 });
}

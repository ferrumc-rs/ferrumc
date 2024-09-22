#![cfg(test)]

use ferrumc_macros::NBTSerialize;
use ferrumc_nbt::de::converter::FromNbt;

#[test]
#[ignore]
fn test_basic_get_functions() {
    // let data = include_bytes!("../../../../.etc/TheAIguy_.nbt");
    // let data = ferrumc_nbt::decompress_gzip(data).unwrap();

    // let mut parser = ferrumc_nbt::de::borrow::NbtTape::new(data.as_slice());
    // parser.parse();

    // let recipe_book = parser.get("recipeBook").unwrap();
    // let recipes = recipe_book.get_element("recipes").unwrap();
    // let recipes: Vec<String> = recipes.as_list(&parser).unwrap();
    // println!("{:?}", recipes);
}

#[test]
#[ignore]
fn test_derive() {
    #[derive(NBTSerialize)]
    struct BasicStruct {
        hello: String,
        world: Two,
        list: Vec<Three>,
    }

    #[derive(NBTSerialize)]
    struct Two {
        a: i32,
        b: i32,
        list: Vec<i32>,
    }

    #[derive(NBTSerialize, Debug)]
    struct Three {
        l: i32,
    }

    let some_struct = BasicStruct {
        hello: "Hello".to_string(),
        world: Two {
            a: 1,
            b: 2,
            list: vec![1, 2, 3],
        },
        list: vec![Three { l: 1 }, Three { l: 2 }],
    };

    let mut buffer = Vec::new();

    some_struct.serialize_with_header(&mut buffer);

    let base_path = r#"D:\Minecraft\framework\ferrumc\ferrumc-2_0\ferrumc\.etc\tests"#;
    std::fs::write(format!("{}/test_derive.nbt", base_path), buffer).unwrap();
}

#[test]
#[ignore]
fn basic_ser() {}

#[test]
#[ignore]
fn create_derive_outline() {
    let data = include_bytes!("../../../../.etc/bigtest.nbt");
    let data = ferrumc_nbt::decompress_gzip(data).unwrap();

    let mut tape = ferrumc_nbt::de::borrow::NbtTape::new(data.as_slice());
    tape.parse();

    mod structs {
        #![allow(dead_code)]

        use ferrumc_nbt::de::borrow::{NbtTape, NbtTapeElement};
        use ferrumc_nbt::de::converter::FromNbt;

        #[derive(Debug)]
        pub(super) struct BigTest {
            byte_test: i8,
            short_test: i16,
            int_test: i32,
            float_test: f32,
            long_test: i64,
            double_test: f64,
            string_test: String,
            byte_array_test: Vec<i8>,
            list_test_long: Vec<i64>,
            nested_compound_test: NestedCompound,
            list_test_compound: Vec<DatedValue>,
        }

        impl FromNbt<'_> for BigTest {
            fn from_nbt(tapes: &NbtTape, element: &NbtTapeElement) -> ferrumc_nbt::Result<Self> {
                let byte_test = <i8 as FromNbt>::from_nbt(tapes, element.get("byteTest").unwrap())?;
                let short_test = <i16 as FromNbt>::from_nbt(tapes, element.get("shortTest").unwrap())?;
                let int_test = <i32 as FromNbt>::from_nbt(tapes, element.get("intTest").unwrap())?;
                let float_test = <f32 as FromNbt>::from_nbt(tapes, element.get("floatTest").unwrap())?;
                let long_test = <i64 as FromNbt>::from_nbt(tapes, element.get("longTest").unwrap())?;
                let double_test = <f64 as FromNbt>::from_nbt(tapes, element.get("doubleTest").unwrap())?;
                let string_test = <String as FromNbt>::from_nbt(tapes, element.get("stringTest").unwrap())?;
                let byte_array_test = <Vec<i8> as FromNbt>::from_nbt(tapes, element.get("byteArrayTest (the first 1000 values of (n*n*255+n*7)%100, starting with n=0 (0, 62, 34, 16, 8, ...))").unwrap())?;
                let list_test_long = <Vec<i64> as FromNbt>::from_nbt(tapes, element.get("listTest (long)").unwrap())?;
                let nested_compound_test = <NestedCompound as FromNbt>::from_nbt(tapes, element.get("nested compound test").unwrap())?;
                let list_test_compound = <Vec<DatedValue> as FromNbt>::from_nbt(tapes, element.get("listTest (compound)").unwrap())?;

                Ok(BigTest {
                    byte_test,
                    short_test,
                    int_test,
                    float_test,
                    long_test,
                    double_test,
                    string_test,
                    byte_array_test,
                    list_test_long,
                    nested_compound_test,
                    list_test_compound,
                })
            }
        }

        #[derive(Debug)]
        pub(super) struct NestedCompound{
            egg: NameValue,
            ham: NameValue,
        }

        impl FromNbt<'_> for NestedCompound {
            fn from_nbt(tapes: &NbtTape, element: &NbtTapeElement) -> ferrumc_nbt::Result<Self> {
                let egg = <NameValue as FromNbt>::from_nbt(tapes, element.get("egg").unwrap())?;
                let ham = <NameValue as FromNbt>::from_nbt(tapes, element.get("ham").unwrap())?;

                Ok(NestedCompound { egg, ham })
            }
        }

        #[derive(Debug)]
        pub(super) struct NameValue {
            name: String,
            value: f32,
        }

        impl FromNbt<'_> for NameValue {
            fn from_nbt(tapes: &NbtTape, element: &NbtTapeElement) -> ferrumc_nbt::Result<Self> {
                let name = <String as FromNbt>::from_nbt(tapes, element.get("name").unwrap())?;
                let value = <f32 as FromNbt>::from_nbt(tapes, element.get("value").unwrap())?;

                Ok(NameValue { name, value })
            }
        }

        #[derive(Debug)]
        pub(super) struct DatedValue {
            name: String,
            created_on: i64,
        }

        impl FromNbt<'_> for DatedValue {
            fn from_nbt(tapes: &NbtTape, element: &NbtTapeElement) -> ferrumc_nbt::Result<Self> {
                let name = <String as FromNbt>::from_nbt(tapes, element.get("name").unwrap())?;
                let created_on = <i64 as FromNbt>::from_nbt(tapes, element.get("created-on").unwrap())?;

                Ok(DatedValue { name, created_on })
            }
        }
    }

    let root = tape.root.as_ref().map(|(_, b)| b).unwrap();

    let big_test = <structs::BigTest as FromNbt>::from_nbt(&tape, root).unwrap();

    drop(data);

    println!("{:#?}", big_test);
}

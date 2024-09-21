#![cfg(test)]

use ferrumc_macros::NBTSerialize;

#[test]
#[ignore]
fn test_basic_get_functions() {
    let data = include_bytes!("../../../../.etc/TheAIguy_.nbt");
    let data = ferrumc_nbt::decompress_gzip(data).unwrap();

    let mut parser = ferrumc_nbt::de::borrow::NbtTape::new(data.as_slice());
    parser.parse();

    let recipe_book = parser.get("recipeBook").unwrap();
    let recipes = recipe_book.get_element("recipes").unwrap();
    let recipes: Vec<String> = recipes.as_list(&parser).unwrap();
    println!("{:?}", recipes);
}

#[test]
#[ignore]
fn test_derive() {
    #[derive(NBTSerialize)]
    struct BasicStruct {
        hello: String,
        world: Two,
        list: Vec<Three>
    }
    
    #[derive(NBTSerialize)]
    struct Two {
        a: i32,
        b: i32,
        list: Vec<i32>
    }
    
    #[derive(NBTSerialize)]
    struct Three {
        l: i32,
    }
    
    let some_struct = BasicStruct {
        hello: "Hello".to_string(),
        world: Two {
            a: 1,
            b: 2,
            list: vec![1, 2, 3]
        },
        list: vec![Three { l: 1 }, Three { l: 2 }]
    };
    
    let mut buffer = Vec::new();
    
    some_struct.serialize_with_header(&mut buffer);
    
    let base_path = r#"D:\Minecraft\framework\ferrumc\ferrumc-2_0\ferrumc\.etc\tests"#;
    std::fs::write(format!("{}/test_derive.nbt", base_path), buffer).unwrap();
}

#[test]
#[ignore]
fn basic_ser() {
    
}
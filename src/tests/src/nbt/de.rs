#![cfg(test)]

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

/*#[test]
#[ignore]
fn test_the_ai_guy_nbt() {
    let data = include_bytes!("../../../../.etc/TheAIguy_.nbt");
    let data = NbtParser::decompress(data).unwrap();
    let mut parser = NbtParser::new(data.as_slice()).clone();

    let tapes = parser.parse().unwrap();

    let root = NbtCompoundView::new(tapes, 0);

    let dim = root.get("Dimension").unwrap();
    let dim: String = String::from_token(dim).unwrap();

    dbg!(dim);
}

#[test]
#[ignore]
fn hello_world() {
    let data = include_bytes!("../../../../.etc/hello_world.nbt");
    let data = NbtParser::decompress(data).unwrap();
    let mut parser = NbtParser::new(data.as_slice()).clone();

    let tapes = parser.parse().unwrap();

    let root = NbtCompoundView::new(tapes, 0);
    let name = root.get("name").unwrap();
    let name: String = String::from_token(name).unwrap();
    dbg!(name);
}

#[test]
#[ignore]
fn bigtest() {
    let data = include_bytes!("../../../../.etc/bigtest.nbt");
    let data = NbtParser::decompress(data).unwrap();
    let mut parser = NbtParser::new(data.as_slice()).clone();

    let tapes = parser.parse().unwrap();

    let root = NbtCompoundView::new(tapes, 0);
    let name = root.get("listTest (long)").unwrap();

    dbg!(name.value());
}

#[test]
#[ignore]
fn nested_compound() {
    let data = include_bytes!("../../../../.etc/tests/nested_compound.nbt");
    let data = NbtParser::decompress(data).unwrap();
    let mut parser = NbtParser::new(data.as_slice()).clone();

    let tapes = parser.parse().unwrap();

    let root = NbtCompoundView::new(tapes, 0);
    let name = root.get("test").unwrap();

    dbg!(name.value());
}
*/

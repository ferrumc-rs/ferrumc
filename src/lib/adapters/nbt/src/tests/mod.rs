use crate::de::NbtParser;

#[test]
fn test_basic() {
    let data = include_bytes!("../../../../../../.etc/hello_world.nbt");

    let mut parser = NbtParser::new(data);
    let parsed = parser.parse();

    println!("{:?}", parsed);
}

#[test]
fn test_big_test() {
    let data = include_bytes!("../../../../../../.etc/bigtest.nbt");
/*
    let mut gzip = libflate::gzip::Decoder::new(&data[..]).unwrap();
    let mut data = vec![];
    std::io::copy(&mut gzip, &mut data).unwrap();
    let mut parser = NbtParser::new(&data);
*/

    let data = NbtParser::decompress(data).unwrap();
    let mut parser = NbtParser::new(&data);

    let parsed = parser.parse();

    println!("{:?}", parsed);
}

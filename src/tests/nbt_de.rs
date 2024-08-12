use nbt_lib::nbt_spec::deserializer::{NBTTagIdentity, read_tag};

#[test]
fn try_read() {
    // base => ../../../../ (root of the repository)
    let file_bytes = std::fs::read(".etc/nbt_lib_validation.nbt").unwrap();

    let mut cursor = std::io::Cursor::new(file_bytes);

    let mut nbt_tag = read_tag(&mut cursor).unwrap();

    // println!("{:#?}", nbt_tag);

    // just map out how the derive macro would work
    let root_name = "ImTheRoot"; // or struct name

    let mut compound = nbt_tag.get(root_name).unwrap();
    // let im_a_child_byte = let  compound.get("im_a_child_byte").unwrap();
    let NBTTagIdentity::Byte(im_a_byte) = compound.get("im_a_byte").unwrap() else {
        panic!("Expected byte, got {:?}", compound.get("im_a_byte").unwrap());
    };
    let NBTTagIdentity::String(im_a_string) = compound.get("im_a_string").unwrap() else {
        panic!("Expected string, got {:?}", compound.get("im_a_string").unwrap());
    };

    let NBTTagIdentity::Compound(im_a_compound) = compound.get("im_a_compound").unwrap() else {
        panic!("Expected compound, got {:?}", compound.get("im_a_compound").unwrap());
    };

    let NBTTagIdentity::String(im_a_child_byte) = im_a_compound.get("im_a_child_byte").unwrap() else {
        panic!("Expected string, got {:?}", im_a_compound.get("im_a_child_byte").unwrap());
    };



    println!("im_a_byte: {}, im_a_string: {}", im_a_byte, im_a_string);
}
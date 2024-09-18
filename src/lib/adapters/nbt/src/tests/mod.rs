use crate::de::{NbtCompoundView, NbtParser};
use crate::errors::NBTError;

#[test]
fn bigtest() -> Result<(), NBTError> {
    let data = include_bytes!("../../../../../../.etc/bigtest.nbt");
    let data = NbtParser::decompress(data)?;
    let data = data.as_slice();

    let mut parser = NbtParser::new(data);
    let tape = parser.parse()?;

    let root_view = NbtCompoundView::new(tape, 0);

    // println!("{root_view:?}");

    let nested_compound = root_view.get("nested compound test").unwrap();

    println!("{:?}", nested_compound.value());

    Ok(())
}

#[test]
fn the_algui() {
    let data = include_bytes!("../../../../../../.etc/TheAIguy_.nbt");
    let data = NbtParser::decompress(data).unwrap();

    let data = data.as_slice();

    let mut parser = NbtParser::new(data);
    let tape = parser.parse().unwrap();

    let root_view = NbtCompoundView::new(tape, 0);

    println!("{root_view:?}");
}

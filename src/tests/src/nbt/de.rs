#![cfg(test)]

use ferrumc_nbt::{NbtCompoundView, NbtParser};

#[test]
#[ignore]
fn test_the_ai_guy_nbt() {
    
    let data = include_bytes!("../../../../.etc/TheAIguy_.nbt");
    let data = NbtParser::decompress(data).unwrap();
    let mut parser = NbtParser::new(data.as_slice()).clone();

    let tapes = parser.parse().unwrap();
    
    let root = NbtCompoundView::new(tapes, 0);
    dbg!(root);
}
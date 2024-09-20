#![cfg(test)]

use ferrumc_nbt::{NbtParser, NbtTokenView};

#[test]
#[ignore]
fn test_the_ai_guy_nbt() {
    let mut parser: NbtParser;

    {
        let data = include_bytes!("../../../../.etc/TheAIguy_.nbt");
        let data = NbtParser::decompress(data).unwrap();
        parser = NbtParser::new(data.as_slice()).clone();
    }

    let tapes = parser.parse().unwrap();
    
}
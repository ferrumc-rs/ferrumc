use std::{hint::black_box, io::Cursor};

fn main() {
    let src = black_box(include_bytes!("../tests/complex_player.dat"));
    let mut decoded_src_decoder = flate2::read::GzDecoder::new(&src[..]);
    let mut input = Vec::new();
    if std::io::Read::read_to_end(&mut decoded_src_decoder, &mut input).is_err() {
        // oh probably wasn't gzipped then
        input = src.to_vec();
    }
    let input = input.as_slice();

    let nbt = simdnbt::owned::read(&mut Cursor::new(input))
        .unwrap()
        .unwrap();

    let mut out = Vec::new();
    nbt.write(&mut out);
    black_box(out);
}

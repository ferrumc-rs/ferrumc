use std::{
    fs::File,
    io::{Cursor, Read},
};

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use flate2::read::GzDecoder;

fn bench_file(filename: &str, c: &mut Criterion) {
    let mut file = File::open(format!("tests/{filename}")).unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    let mut src = &contents[..];

    // decode the original src so most of the time isn't spent on unzipping
    let mut src_decoder = GzDecoder::new(&mut src);
    let mut input = Vec::new();
    if src_decoder.read_to_end(&mut input).is_err() {
        // oh probably wasn't gzipped then
        input = contents;
    }

    let mut input_stream = Cursor::new(&input[..]);

    let mut group = c.benchmark_group(format!("nbt_borrow/{filename}"));
    group.throughput(Throughput::Bytes(input.len() as u64));

    group.bench_function("Decode", |b| {
        b.iter(|| {
            black_box(simdnbt::borrow::read(&mut input_stream).unwrap());
            input_stream.set_position(0);
        })
    });

    let nbt = simdnbt::borrow::read(&mut input_stream).unwrap().unwrap();
    // group.bench_function("Get", |b| {
    //     b.iter(|| {
    //         let level = nbt.compound("abilities").unwrap();
    //         for (k, _) in level.iter() {
    //             black_box(level.get(black_box(&k.to_str())));
    //         }
    //     })
    // });
    group.finish();
}

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn bench(c: &mut Criterion) {
    bench_file("hypixel.nbt", c);
    bench_file("complex_player.dat", c);
    bench_file("level.dat", c);
    bench_file("bigtest.nbt", c);
    bench_file("simple_player.dat", c);

    // bench_file("stringtest.nbt", c);
    // bench_file("inttest16.nbt", c);

    // bench_file("inttest1023.nbt", c);
    // bench_file("inttest3.nbt", c);
}

criterion_group!(nbt, bench);
criterion_main!(nbt);

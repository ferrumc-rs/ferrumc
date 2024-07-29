use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use flate2::read::GzDecoder;
use std::{
    fs::File,
    io::{Cursor, Read},
};

fn bench_file(filename: &str, c: &mut Criterion) {
    let mut file = File::open(format!("tests/{filename}")).unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    let mut src = &contents[..];

    // decode the original src so most of the time isn't spent on unzipping
    let mut decoded_src_decoder = GzDecoder::new(&mut src);
    let mut decoded_src = Vec::new();
    if decoded_src_decoder.read_to_end(&mut decoded_src).is_err() {
        // oh probably wasn't gzipped then
        decoded_src = contents;
    }

    let mut decoded_src_stream = Cursor::new(&decoded_src[..]);

    let mut group = c.benchmark_group(format!("nbt_owned/{filename}"));

    group.throughput(Throughput::Bytes(decoded_src.len() as u64));

    group.bench_function("Decode", |b| {
        b.iter(|| {
            black_box(simdnbt::owned::read(&mut decoded_src_stream).unwrap());
            decoded_src_stream.set_position(0);
        })
    });

    let nbt = simdnbt::owned::read(&mut decoded_src_stream)
        .unwrap()
        .unwrap();
    group.bench_function("Get", |b| {
        b.iter(|| {
            let level = nbt.compound("abilities").unwrap();
            for (k, _) in level.iter() {
                black_box(level.get(black_box(&k.to_str())));
            }
        })
    });
    group.bench_function("Encode", |b| {
        b.iter(|| {
            let mut out = Vec::new();
            nbt.write(&mut out);
            black_box(out);
        })
    });
    group.finish();
}

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn bench(c: &mut Criterion) {
    bench_file("bigtest.nbt", c);
    bench_file("simple_player.dat", c);
    bench_file("complex_player.dat", c);
    bench_file("level.dat", c);

    // bench_file("stringtest.nbt", c);
    // bench_file("inttest16.nbt", c);

    // bench_file("inttest1023.nbt", c);
    // bench_file("inttest3.nbt", c);
}

criterion_group!(nbt, bench);
criterion_main!(nbt);

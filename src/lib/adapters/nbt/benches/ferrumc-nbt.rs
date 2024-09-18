#![feature(portable_simd)]

use std::io::Cursor;
use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use fastnbt::Value;
use ferrumc_nbt::de::{NbtParser, NbtTag};

fn bench_ferrumc_nbt(data: &[u8]) {
    let mut nbt_parser = NbtParser::new(data);
    let (_name, nbt) = nbt_parser.parse().unwrap();
    let nbt = black_box(nbt);
    if let NbtTag::Compound(root) = nbt {
        assert!(root.get("nested compound test").is_some());
    }
}

fn bench_v2_lib(data: &[u8]) {
    let nbt = ferrumc_nbt::v2_parser::read_tag(&mut Cursor::new(data.to_vec())).unwrap();
    let mut nbt = black_box(nbt);

    assert!(nbt.get("Level").unwrap().get("nested compound test").is_some());
}

fn bench_simdnbt(data: &[u8]) {
    let nbt = simdnbt::borrow::read(&mut Cursor::new(data)).unwrap();
    let nbt = black_box(nbt).unwrap();
    assert!(nbt.get("nested compound test").is_some());
}

fn bench_simdnbt_owned(data: &[u8]) {
    let nbt = simdnbt::owned::read(&mut Cursor::new(data)).unwrap();
    let nbt = black_box(nbt).unwrap();
    assert!(nbt.get("nested compound test").is_some());
}

fn fastnbt(data: &[u8]) {
    let nbt: Value = fastnbt::from_bytes(data).unwrap();
    let nbt = black_box(nbt);
    
    if let Value::Compound(root) = nbt {
        assert!(root.contains_key("nested compound test"));
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let data = include_bytes!("../../../../../.etc/bigtest.nbt");
    let data = NbtParser::decompress(data).unwrap();
    let data = data.as_slice();

    let mut group = c.benchmark_group("NBT Parsing");
    group.throughput(Throughput::Bytes(data.len() as u64));
    group.bench_function("ferrumc nbt", |b| b.iter(|| bench_ferrumc_nbt(black_box(data))));
    group.bench_function("v2 lib", |b| b.iter(|| bench_v2_lib(black_box(data))));
    group.bench_function("simdnbt borrow", |b| b.iter(|| bench_simdnbt(black_box(data))));
    group.bench_function("simdnbt owned", |b| b.iter(|| bench_simdnbt_owned(black_box(data))));
    group.bench_function("fastnbt", |b| b.iter(|| fastnbt(black_box(data))));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

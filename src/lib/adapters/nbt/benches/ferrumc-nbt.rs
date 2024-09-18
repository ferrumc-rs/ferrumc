#![feature(portable_simd)]

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use fastnbt::Value;
use ferrumc_nbt::de::NbtParser;
use nbt as hematite_nbt;
use std::io::Cursor;

fn bench_ferrumc_nbt(data: &[u8]) {
    let mut parser = NbtParser::new(data);
    assert!(parser.parse().is_ok());
}

fn bench_simdnbt(data: &[u8]) {
    let nbt = simdnbt::borrow::read(&mut Cursor::new(data)).unwrap();
    assert!(nbt.is_some())
}

fn bench_simdnbt_owned(data: &[u8]) {
    let nbt = simdnbt::owned::read(&mut Cursor::new(data)).unwrap();
    assert!(nbt.is_some())
}

fn ussr_nbt_borrow(data: &[u8]) {
    let nbt = black_box(ussr_nbt::borrow::Nbt::read(&mut Cursor::new(data)).unwrap());
    black_box(nbt);
}

fn ussr_nbt_owned(data: &[u8]) {
    let nbt = black_box(ussr_nbt::owned::Nbt::read(&mut Cursor::new(data)).unwrap());
    black_box(nbt);
}

fn fastnbt(data: &[u8]) {
    let nbt: Value = black_box(fastnbt::from_reader(&mut Cursor::new(data)).unwrap());
    black_box(nbt);
}

fn crab_nbt(data: &[u8]) {
    let nbt = crab_nbt::Nbt::read(&mut Cursor::new(data)).unwrap();
    black_box(nbt);
}

fn hematite_nbt(data: &[u8]) {
    let nbt = hematite_nbt::Blob::from_reader(&mut Cursor::new(data)).unwrap();
    black_box(nbt);
}

fn criterion_benchmark(c: &mut Criterion) {
    let data = include_bytes!("../../../../../.etc/TheAIguy_.nbt");
    let data = NbtParser::decompress(data).unwrap();
    let data = data.as_slice();

    let mut group = c.benchmark_group("NBT Parsing");
    group.throughput(Throughput::Bytes(data.len() as u64));
    group.bench_function("FerrumC NBT", |b| {
        b.iter(|| bench_ferrumc_nbt(black_box(data)))
    });
    group.bench_function("simdnbt borrow", |b| {
        b.iter(|| bench_simdnbt(black_box(data)))
    });
    group.bench_function("simdnbt owned", |b| {
        b.iter(|| bench_simdnbt_owned(black_box(data)))
    });
    group.bench_function("fastnbt", |b| b.iter(|| fastnbt(black_box(data))));
    group.bench_function("ussr_nbt owned", |b| {
        b.iter(|| ussr_nbt_owned(black_box(data)))
    });
    group.bench_function("ussr_nbt borrow", |b| {
        b.iter(|| ussr_nbt_borrow(black_box(data)))
    });
    group.bench_function("crab_nbt", |b| b.iter(|| crab_nbt(black_box(data))));
    group.bench_function("hematite_nbt", |b| b.iter(|| hematite_nbt(black_box(data))));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

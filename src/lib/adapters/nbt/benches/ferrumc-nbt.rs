#![feature(portable_simd)]

use std::fs::{OpenOptions, Permissions};
use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use fastnbt::Value;
use nbt as hematite_nbt;
use std::io::Cursor;
use fastanvil::Region;

fn bench_ferrumc_nbt(data: &[u8]) {
    let mut parser = ferrumc_nbt::de::borrow::NbtTape::new(data);
    parser.parse();

    black_box(parser);
}

fn bench_simdnbt(data: &[u8]) {
    let nbt = simdnbt::borrow::read(&mut Cursor::new(data)).unwrap();
    assert!(nbt.is_some());
}

fn bench_simdnbt_owned(data: &[u8]) {
    let nbt = simdnbt::owned::read(&mut Cursor::new(data)).unwrap();
    assert!(nbt.is_some());
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
    // let cursor = Cursor::new(include_bytes!("../../../../../.etc/benches/region/r.0.0.mca"));
    // let file = std::fs::File::open(r#"D:\Minecraft\framework\ferrumc\ferrumc-2_0\ferrumc\.etc\benches\region\r.0.0.mca"#).unwrap();
    
    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .open(r#"D:\Minecraft\framework\ferrumc\ferrumc-2_0\ferrumc\.etc\benches\region\r.0.0.mca"#)
        .unwrap();
    
    
    let mut region = Region::new(file).unwrap();
    let chunk = region.iter().next().unwrap().unwrap();
    let data = chunk.data.as_slice();
    // let data = include_bytes!("../../../../../.etc/benches/registry_data.nbt");
    let data = ferrumc_nbt::decompress_gzip(data).unwrap();
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

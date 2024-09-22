use criterion::{black_box, criterion_group, criterion_main};
use ferrumc_storage::Compressor;
use ferrumc_storage::compressors::zstd::ZstdCompressor;
use ferrumc_utils::root;

fn zstd_compress(data: &[u8]) {
    let compressor = ZstdCompressor::new(6);
    let compressed = compressor.compress(data).unwrap();
    black_box(compressed);
}

fn gzip_compress(data: &[u8]) {
    let compressor = ferrumc_storage::compressors::gzip::GzipCompressor::new(6);
    let compressed = compressor.compress(data).unwrap();
    black_box(compressed);
}

fn criterion_benchmark(c: &mut criterion::Criterion) {
    let data = std::fs::read(root!(".etc/codec.nbt")).unwrap();
    let data = data.as_slice();

    let mut group = c.benchmark_group("Compression");
    group.throughput(criterion::Throughput::Bytes(data.len() as u64));
    group.bench_function("Zstd", |b| b.iter(|| zstd_compress(black_box(data))));
    group.bench_function("Gzip", |b| b.iter(|| gzip_compress(black_box(data))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
use criterion::{criterion_group, criterion_main};
mod compression;
mod db;

fn storage_benches(c: &mut criterion::Criterion) {
    db::db_benches(c);
    compression::compression_benchmarks(c);
}
criterion_group!(storage_bench, storage_benches);
criterion_main!(storage_bench);

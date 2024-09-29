use criterion::{criterion_group, criterion_main};
mod databases;
mod compression;

criterion_group!(compression_benches, compression::compression_benchmarks);
criterion_group!(db_benches, databases::database_benchmarks);
criterion_main!(db_benches);
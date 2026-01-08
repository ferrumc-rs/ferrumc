mod chunk;
mod edit_bench;

use criterion::{criterion_group, criterion_main};
fn world_benches(c: &mut criterion::Criterion) {
    edit_bench::bench_edits(c);
    chunk::bench_chunks(c);
}
criterion_group!(world_bench, world_benches);
criterion_main!(world_bench);

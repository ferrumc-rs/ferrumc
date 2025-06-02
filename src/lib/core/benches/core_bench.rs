use criterion::{criterion_group, criterion_main, Criterion};
mod collisions;

fn bench(c: &mut Criterion) {
    collisions::bench_collides(c);
}

criterion_group!(benches, bench);
criterion_main!(benches);

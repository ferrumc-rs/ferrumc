use criterion::{Criterion, criterion_group, criterion_main};
mod collisions;

fn bench(c: &mut Criterion) {
    collisions::bench_collides(c);
}

criterion_group!(benches, bench);
criterion_main!(benches);

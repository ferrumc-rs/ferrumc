use criterion::{criterion_group, criterion_main, Criterion};

mod packets;
fn bench_encoding(c: &mut Criterion) {
    let mut group = c.benchmark_group("encoding packets");

    packets::bench_packets(&mut group);

    group.finish();
}

criterion_main!(benches);
criterion_group!(benches, bench_encoding);
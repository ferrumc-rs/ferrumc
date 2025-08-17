use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use std::hint::black_box;

fn bench_lookup(c: &mut Criterion) {

    // run once to load the registry

    black_box(ferrumc_registry::lookup("minecraft:item/entries/minecraft:cobblestone/protocol_id"));

    let mut group = c.benchmark_group("registry_lookup");

    group.throughput(Throughput::Elements(1));

    group.bench_function("lookup_cobblestone", |b| {
        b.iter(|| {
            // black_box to prevent compiler optimizations
            black_box(ferrumc_registry::lookup("minecraft:item/entries/minecraft:cobblestone/protocol_id"))
        })
    });

    group.finish()
}

criterion_group!(benches, bench_lookup);
criterion_main!(benches);

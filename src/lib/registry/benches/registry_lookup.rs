use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use std::hint::black_box;

/// Benchmarks the item to/from protocol_ID maps
fn bench_item_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("registry_item_lookup");
    group.throughput(Throughput::Elements(1));

    group.bench_function("get_item_by_id (apple)", |b| {
        b.iter(|| black_box(ferrumc_registry::get_item_by_id(800)))
    });

    group.bench_function("get_item_by_name (cobblestone)", |b| {
        b.iter(|| black_box(ferrumc_registry::get_item_by_name("minecraft:cobblestone")))
    });
    group.finish();
}

/// Benchmarks the to/from protocol_ID maps
fn bench_blockstate_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("registry_blockstate_lookup");
    group.throughput(Throughput::Elements(1));

    group.bench_function("get_block_by_id (stone)", |b| {
        b.iter(|| black_box(ferrumc_registry::get_block_by_id(1)))
    });

    group.bench_function("lookup_blockstate_name (grass)", |b| {
        b.iter(|| black_box(ferrumc_registry::get_block_by_name("minecraft:grass")))
    });
    group.finish();
}

/// Benchmarks the block->item ID map
fn bench_item_to_block_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("registry_item_to_block_lookup");
    group.throughput(Throughput::Elements(1));

    group.bench_function("lookup_item_to_block_id (stone)", |b| {
        b.iter(|| {
            // Assuming item "1" (stone) maps to a block
            black_box(ferrumc_registry::get_block_id_from_item_id(1))
        })
    });
    group.finish();
}

// Register all the new benchmark functions
criterion_group!(
    benches,
    bench_item_lookup,
    bench_blockstate_lookup,
    bench_item_to_block_lookup
);
criterion_main!(benches);

use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use std::hint::black_box;

/// Helper function to run `init()` once.
/// In our new compile-time setup, this is a no-op,
/// but it's good practice to keep it.
fn setup() {
    ferrumc_registry::init();
}

/// Benchmarks the `ITEM_NAME_TO_ID` map (from registries.json)
fn bench_item_lookup(c: &mut Criterion) {
    setup();
    let mut group = c.benchmark_group("registry_item_lookup");
    group.throughput(Throughput::Elements(1));

    group.bench_function("lookup_item_protocol_id (apple)", |b| {
        b.iter(|| black_box(ferrumc_registry::lookup_item_protocol_id("minecraft:apple")))
    });

    group.bench_function("lookup_item_protocol_id (cobblestone)", |b| {
        b.iter(|| {
            black_box(ferrumc_registry::lookup_item_protocol_id(
                "minecraft:cobblestone",
            ))
        })
    });
    group.finish();
}

/// Benchmarks the `BLOCKSTATE_ID_TO_NAME` map (from blockstates.json)
fn bench_blockstate_lookup(c: &mut Criterion) {
    setup();
    let mut group = c.benchmark_group("registry_blockstate_lookup");
    group.throughput(Throughput::Elements(1));

    group.bench_function("lookup_blockstate_name (stone)", |b| {
        b.iter(|| black_box(ferrumc_registry::lookup_blockstate_name("1")))
    });

    group.bench_function("lookup_blockstate_name (grass)", |b| {
        b.iter(|| black_box(ferrumc_registry::lookup_blockstate_name("9")))
    });
    group.finish();
}

/// Benchmarks the `ITEM_ID_STR_TO_BLOCKSTATE_ID_STR` map
fn bench_item_to_block_lookup(c: &mut Criterion) {
    setup();
    let mut group = c.benchmark_group("registry_item_to_block_lookup");
    group.throughput(Throughput::Elements(1));

    group.bench_function("lookup_item_to_block_id_str (stone)", |b| {
        b.iter(|| {
            // Assuming item "1" (stone) maps to a block
            black_box(ferrumc_registry::lookup_item_to_block_id_str("1"))
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

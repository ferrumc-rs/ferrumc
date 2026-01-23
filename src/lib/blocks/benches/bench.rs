use criterion::{criterion_group, criterion_main, Criterion};
use ferrumc_blocks::mappings::BLOCK_MAPPINGS;



fn benches(c: &mut Criterion) {
    fn raw_func_dispatch_ctrl() {}

    c.bench_function("Call execution", |b| {
        b.iter(|| {
            BLOCK_MAPPINGS[12052].test()
        });
    });

    c.bench_function("Control raw fn execution", |b| {
        b.iter(|| {
            raw_func_dispatch_ctrl()
        });
    });
}

criterion_group!(block_benches, benches);
criterion_main!(block_benches);
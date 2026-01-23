use criterion::{criterion_group, criterion_main, Criterion};
use ferrumc_blocks::mappings::BLOCK_MAPPINGS;



fn benches(c: &mut Criterion) {
    c.bench_function("Call execution", |b| {
        b.iter(|| {
            BLOCK_MAPPINGS[34].test()
        });
    });
}

criterion_group!(block_benches, benches);
criterion_main!(block_benches);
use std::hint::black_box;

use criterion::Criterion;
use ferrumc_world::World;

pub(crate) fn bench_cache(c: &mut Criterion) {
    let backend_path = std::env::current_dir()
        .unwrap()
        .join("../../../target/debug/world");
    let mut group = c.benchmark_group("world_load");
    group.bench_function("Load chunk 1,1 uncached", |b| {
        b.iter_batched(
            || World::new(&backend_path),
            |world| world.load_chunk(black_box(1), black_box(1), black_box("overworld")),
            criterion::BatchSize::PerIteration,
        );
    });
    group.bench_function("Load block 1,1 uncached", |b| {
        b.iter_batched(
            || World::new(&backend_path),
            |world| {
                world.get_block_and_fetch(
                    black_box(1),
                    black_box(1),
                    black_box(1),
                    black_box("overworld"),
                )
            },
            criterion::BatchSize::PerIteration,
        );
    });
    let world = World::new(backend_path);
    let load_chunk = || {
        world.load_chunk(1, 1, "overworld").expect(
            "Failed to load chunk. If it's a bitcode error, chances are the chunk format \
             has changed since last generating a world so you'll need to regenerate",
        )
    };
    _ = load_chunk();
    group.bench_function("Load chunk 1,1 cached", |b| {
        b.iter(|| world.load_chunk(black_box(1), black_box(1), black_box("overworld")))
    });
    group.bench_function("Load block 1,1 cached", |b| {
        b.iter(|| {
            world.get_block_and_fetch(
                black_box(1),
                black_box(1),
                black_box(1),
                black_box("overworld"),
            )
        });
    });
}

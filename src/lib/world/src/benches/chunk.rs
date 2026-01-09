use criterion::Criterion;
use ferrumc_macros::block;
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::chunk::Chunk;
use ferrumc_world::pos::ChunkBlockPos;
use std::hint::black_box;

pub fn bench_chunks(c: &mut Criterion) {
    let mut chunk = Chunk::new_empty();

    for x in 0..16 {
        for z in 0..16 {
            for y in -64..320 {
                chunk.set_block(
                    ChunkBlockPos::new(x, y, z),
                    BlockStateId::new(rand::random_range(0..20000)),
                );
            }
        }
    }

    c.bench_function("new chunk system - write", |b| {
        b.iter(|| {
            let mut chunk = Chunk::new_empty();

            for x in 0..16 {
                for z in 0..16 {
                    for y in -64..320 {
                        chunk.set_block(ChunkBlockPos::new(x, y, z), block!("stone"));
                    }
                }
            }

            black_box(chunk);
        });
    });

    c.bench_function("new chunk system - write random", |b| {
        b.iter(|| {
            let mut chunk = Chunk::new_empty();

            for x in 0..16 {
                for z in 0..16 {
                    for y in -64..320 {
                        chunk.set_block(
                            ChunkBlockPos::new(x, y, z),
                            BlockStateId::new(rand::random_range(0..20000)),
                        );
                    }
                }
            }

            black_box(chunk);
        });
    });

    c.bench_function("new chunk system - read", |b| {
        b.iter(|| {
            for x in 0..16 {
                for z in 0..16 {
                    for y in -64..320 {
                        let a = chunk.get_block(ChunkBlockPos::new(x, y, z));
                        black_box(a);
                    }
                }
            }
        });
    });
}

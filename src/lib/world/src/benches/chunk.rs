use criterion::Criterion;
use ferrumc_world::chunk::Chunk;
use ferrumc_world::pos::ChunkBlockPos;

pub fn bench_chunks(c: &mut Criterion) {
    let mut chunk = Chunk::new_empty();

    for x in 0..16 {
        for z in 0..16 {
            for y in -64..256 {
                chunk.set_block(ChunkBlockPos::new(x, y, z), 130);
            }
        }
    }

    c.bench_function("new chunk system - write", |b| {
        b.iter(|| {
            let mut chunk = Chunk::new_empty();

            for x in 0..16 {
                for z in 0..16 {
                    for y in -64..256 {
                        chunk.set_block(ChunkBlockPos::new(x, y, z), 130);
                    }
                }
            }
        });
    });

    c.bench_function("new chunk system - read", |b| {
        b.iter(|| {
            for x in 0..16 {
                for z in 0..16 {
                    for y in -64..256 {
                        let a = chunk.get_block(ChunkBlockPos::new(x, y, z));
                        assert_eq!(a, 130)
                    }
                }
            }
        });
    });
}

use criterion::{Criterion, Throughput};
use ferrumc_macros::block;
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::chunk_format::Chunk;
use rand::Rng;
use std::hint::black_box;

fn get_rand_in_range(min: i32, max: i32) -> i32 {
    let mut rng = rand::rng();
    rng.random_range(min..=max)
}

pub(crate) fn bench_edits(c: &mut Criterion) {
    let chunk_data = std::fs::read("../../../.etc/raw_chunk.dat").unwrap();
    let chunk: Chunk = bitcode::decode(&chunk_data).unwrap();

    let mut read_group = c.benchmark_group("edit_read");

    read_group.throughput(Throughput::Elements(1));

    read_group.bench_function("Read 0,0,0", |b| {
        b.iter(|| black_box(chunk.get_block(0, 0, 0)));
    });

    read_group.bench_function("Read 8,8,150", |b| {
        b.iter(|| black_box(chunk.get_block(8, 8, 150)));
    });

    read_group.bench_function("Read rand", |b| {
        b.iter(|| {
            black_box(chunk.get_block(
                get_rand_in_range(0, 15),
                get_rand_in_range(0, 15),
                get_rand_in_range(0, 255),
            ))
        });
    });

    read_group.finish();

    let mut write_group = c.benchmark_group("edit_write");

    write_group.throughput(Throughput::Elements(1));

    write_group.bench_with_input("Write 0,0,0", &chunk, |b, chunk| {
        b.iter(|| {
            let mut chunk = chunk.clone();
            black_box(chunk.set_block(0, 0, 0, block!("bricks"))).unwrap();
        });
    });

    write_group.bench_with_input("Write 8,8,150", &chunk, |b, chunk| {
        b.iter(|| {
            let mut chunk = chunk.clone();
            black_box(chunk.set_block(8, 8, 150, block!("bricks"))).unwrap();
        });
    });

    write_group.bench_with_input("Write rand", &chunk, |b, chunk| {
        b.iter(|| {
            let mut chunk = chunk.clone();
            black_box(chunk.set_block(
                get_rand_in_range(0, 15),
                get_rand_in_range(0, 15),
                get_rand_in_range(0, 255),
                block!("bricks"),
            ))
            .unwrap();
        });
    });

    write_group.throughput(Throughput::Elements(16 * 256 * 16));

    write_group.bench_with_input("Fill", &chunk, |b, chunk| {
        b.iter(|| {
            let mut chunk = chunk.clone();
            black_box(chunk.fill(block!("bricks"))).unwrap();
        });
    });

    write_group.bench_with_input("Manual Fill", &chunk, |b, chunk| {
        b.iter(|| {
            let mut chunk = chunk.clone();
            for x in 0..16 {
                for y in 0..256 {
                    for z in 0..16 {
                        black_box(chunk.set_block(x, y, z, block!("bricks"))).unwrap();
                    }
                }
            }
        });
    });

    write_group.bench_with_input("Manual batch fill same", &chunk, |b, chunk| {
        b.iter(|| {
            let mut chunk = chunk.clone();
            let mut batch = ferrumc_world::edit_batch::EditBatch::new(&mut chunk);
            for x in 0..16 {
                for y in 0..256 {
                    for z in 0..16 {
                        batch.set_block(x, y, z, black_box(block!("bricks")));
                    }
                }
            }
            black_box(batch.apply()).unwrap();
        });
    });

    write_group.bench_with_input("Manual batch fill diff", &chunk, |b, chunk| {
        b.iter(|| {
            let mut chunk = chunk.clone();
            let mut batch = ferrumc_world::edit_batch::EditBatch::new(&mut chunk);
            for x in 0..16 {
                for y in 0..256 {
                    for z in 0..16 {
                        let block = if (x + y + z) % 2 == 0 {
                            block!("bricks")
                        } else {
                            block!("stone")
                        };
                        batch.set_block(x, y, z, black_box(block));
                    }
                }
            }
            black_box(batch.apply()).unwrap();
        });
    });

    write_group.finish();
}

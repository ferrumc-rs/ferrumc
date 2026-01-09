use criterion::{Criterion, Throughput};
use ferrumc_macros::block;
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::chunk::Chunk;
use rand::Rng;
use std::hint::black_box;

fn get_rand_in_range(min: i32, max: i32) -> i32 {
    let mut rng = rand::rng();
    rng.random_range(min..=max)
}

pub(crate) fn bench_edits(c: &mut Criterion) {
    let chunk_data = std::fs::read("../../../.etc/raw_chunk.dat").unwrap();
    let mut chunk: Chunk = bitcode::decode(&chunk_data)
        .expect("If this fails, go run the dump_chunk test at src/lib/world/src/lib.rs");

    let mut read_group = c.benchmark_group("edit_read");

    read_group.throughput(Throughput::Elements(1));

    read_group.bench_function("Read 0,0,0", |b| {
        b.iter(|| black_box(chunk.get_block((0, 0, 0).into())));
    });

    read_group.bench_function("Read 8,8,150", |b| {
        b.iter(|| black_box(chunk.get_block((8, 150, 8).into())));
    });

    read_group.bench_function("Read rand", |b| {
        b.iter(|| {
            black_box(
                chunk.get_block(
                    (
                        get_rand_in_range(0, 15) as u8,
                        get_rand_in_range(0, 255) as i16,
                        get_rand_in_range(0, 15) as u8,
                    )
                        .into(),
                ),
            )
        });
    });

    read_group.finish();

    let mut write_group = c.benchmark_group("edit_write");

    write_group.throughput(Throughput::Elements(1));

    write_group.bench_with_input("Write 0,0,0", &chunk, |b, chunk| {
        b.iter(|| {
            let mut chunk = chunk.clone();
            black_box(chunk.set_block((0, 0, 0).into(), block!("bricks")));
        });
    });

    write_group.bench_with_input("Write 8,8,150", &chunk, |b, chunk| {
        b.iter(|| {
            let mut chunk = chunk.clone();
            black_box(chunk.set_block((8, 150, 8).into(), block!("bricks")));
        });
    });

    write_group.bench_with_input("Write rand", &chunk, |b, chunk| {
        b.iter(|| {
            let mut chunk = chunk.clone();
            black_box(
                chunk.set_block(
                    (
                        get_rand_in_range(0, 15) as u8,
                        get_rand_in_range(0, 255) as i16,
                        get_rand_in_range(0, 15) as u8,
                    )
                        .into(),
                    block!("bricks"),
                ),
            );
        });
    });

    write_group.throughput(Throughput::Elements(16 * 256 * 16));

    write_group.bench_with_input("Fill", &chunk, |b, chunk| {
        b.iter(|| {
            let mut chunk = chunk.clone();
            black_box(chunk.fill(block!("bricks")));
        });
    });

    write_group.bench_with_input("Manual Fill", &chunk, |b, chunk| {
        b.iter(|| {
            let mut chunk = chunk.clone();
            for x in 0..16 {
                for y in 0..256 {
                    for z in 0..16 {
                        black_box(chunk.set_block((x, y, z).into(), block!("bricks")));
                    }
                }
            }
        });
    });

    write_group.finish();
}

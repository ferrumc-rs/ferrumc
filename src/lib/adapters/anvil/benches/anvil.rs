use std::fs::File;
use std::path::PathBuf;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ferrumc_utils::root;
use rayon::prelude::*;
use fastanvil::Region;
use ferrumc_anvil::load_anvil_file;

fn criterion_benchmark(c: &mut Criterion) {
    let mut read_all_group = c.benchmark_group("Read All");
    
    read_all_group.bench_function("FerrumC Rayon", |b| {
        b.iter(|| {
            let file_path = PathBuf::from(root!(".etc/r.0.0.mca"));
            let loaded_file = load_anvil_file(file_path).unwrap();
            let locations = loaded_file.get_locations();
            locations.chunks(96).par_bridge().for_each(|chunk| {
                chunk.iter().for_each(|location| {
                    black_box(loaded_file.get_chunk_from_location(*location));
                });
            });
        });
    });
    
    read_all_group.bench_function("FerrumC", |b| {
        b.iter(|| {
            let file_path = PathBuf::from(root!(".etc/r.0.0.mca"));
            let loaded_file = load_anvil_file(file_path).unwrap();
            let locations = loaded_file.get_locations();
            locations.iter().for_each(|location| {
                black_box(loaded_file.get_chunk_from_location(*location));
            });
        });
    });
    
    read_all_group.bench_function("FastAnvil", |b| {
        b.iter(|| {
            let file = File::open(root!(".etc/r.0.0.mca")).unwrap();
            let mut region = Region::from_stream(file).unwrap();
            region.iter().for_each(|chunk| {
                black_box(chunk.unwrap().data);
            });
        });
    });
    
    read_all_group.finish();
    
    let mut read_one_group = c.benchmark_group("Read One");
    
    read_one_group.bench_function("FerrumC", |b| {
        b.iter(|| {
            let file_path = PathBuf::from(root!(".etc/r.0.0.mca"));
            let loaded_file = load_anvil_file(file_path).unwrap();
            black_box(loaded_file.get_chunk(0, 0));
        });
    });
    
    read_one_group.bench_function("FastAnvil", |b| {
        b.iter(|| {
            let file = File::open(root!(".etc/r.0.0.mca")).unwrap();
            let mut region = Region::from_stream(file).unwrap();
            black_box(region.read_chunk(0, 0).unwrap());
        });
    });
    
    read_one_group.finish();
}



criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
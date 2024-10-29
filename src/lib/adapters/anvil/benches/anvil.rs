extern crate core;

use std::fs::File;
use std::path::PathBuf;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ferrumc_anvil::loader::load_anvil_file;
use ferrumc_utils::root;
use rayon::prelude::*;
use fastanvil::Region;
use ferrumc_anvil::mappings::Chunk;

fn criterion_benchmark(c: &mut Criterion) {
    let read_all_group = {
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
        
        read_all_group
    };
    read_all_group.finish();
    
    let read_one_group = {
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
        
        read_one_group
    };
    read_one_group.finish();
    
    let read_with_nbt_all = {
        let mut read_with_nbt_all = c.benchmark_group("Read With NBT (All)");

        read_with_nbt_all.bench_function("FerrumC Rayon (NBT + Anvil)", |b| {
            b.iter(|| {
                let file_path = PathBuf::from(root!(".etc/r.0.0.mca"));
                let loaded_file = load_anvil_file(file_path).unwrap();
                let locations = loaded_file.get_locations();
                locations.chunks(96).par_bridge().for_each(|chunk| {
                    chunk.iter().for_each(|location| {
                        let chunk = loaded_file.get_chunk_from_location(*location).unwrap();
                        let chunk = Chunk::from_bytes(chunk.as_slice()).unwrap();
                        black_box(chunk);
                    });
                });
            });
        });

        read_with_nbt_all.bench_function("FerrumC (NBT + Anvil)", |b| {
            b.iter(|| {
                let file_path = PathBuf::from(root!(".etc/r.0.0.mca"));
                let loaded_file = load_anvil_file(file_path).unwrap();
                let locations = loaded_file.get_locations();
                locations.iter().for_each(|location| {
                    let chunk = loaded_file.get_chunk_from_location(*location).unwrap();
                    let chunk = Chunk::from_bytes(chunk.as_slice()).unwrap();
                    black_box(chunk);
                });
            });
        });

        read_with_nbt_all.bench_function("FastAnvil(NBT + Anvil)", |b| {
            b.iter(|| {
                let file = File::open(root!(".etc/r.0.0.mca")).unwrap();
                let mut region = Region::from_stream(file).unwrap();
                region.iter().for_each(|chunk| {
                    let chunk = chunk.unwrap().data;
                    let nbt: fastnbt::Value = fastnbt::from_bytes(chunk.as_slice()).unwrap();
                    black_box(nbt);
                });
            });
        });
        
        read_with_nbt_all
    };
    
    read_with_nbt_all.finish();
    
    let read_with_nbt_one = {
        let mut read_with_nbt = c.benchmark_group("Read With NBT (One)");

        read_with_nbt.bench_function("FerrumC (NBT + Anvil)", |b| {
            b.iter(|| {
                let file_path = PathBuf::from(root!(".etc/r.0.0.mca"));
                let loaded_file = load_anvil_file(file_path).unwrap();
                let chunk = loaded_file.get_chunk(0, 0).unwrap();
                let chunk = Chunk::from_bytes(chunk.as_slice()).unwrap();
                black_box(chunk);
            });
        });
        
        read_with_nbt.bench_function("Fast(NBT + Anvil)", |b| {
            b.iter(|| {
                let file = File::open(root!(".etc/r.0.0.mca")).unwrap();
                let mut region = Region::from_stream(file).unwrap();
                let chunk = region.read_chunk(0, 0).unwrap().unwrap();
                let nbt: fastnbt::Value = fastnbt::from_bytes(chunk.as_slice()).unwrap();
                black_box(nbt);
            });
        });

        read_with_nbt
    };
    
    read_with_nbt_one.finish();
    
    
    
    let parse_only_nbt = {
        let chunk = {
            let file_path = PathBuf::from(root!(".etc/r.0.0.mca"));
            let loaded_file = load_anvil_file(file_path).unwrap();
            loaded_file.get_chunk(0, 0).unwrap()
        };  
        
        let mut parse_only_nbt = c.benchmark_group("Parse Only NBT");
        
        parse_only_nbt.bench_function("FerrumC", |b| {
            b.iter(|| {
                let chunk = Chunk::from_bytes(chunk.as_slice()).unwrap();
                black_box(chunk);
            });
        });
        
        parse_only_nbt.bench_function("FastAnvil", |b| {
            b.iter(|| {
                let nbt: fastnbt::Value = fastnbt::from_bytes(chunk.as_slice()).unwrap();
                black_box(nbt);
            });
        });
        
        parse_only_nbt
    };
    
    parse_only_nbt.finish();
}



criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
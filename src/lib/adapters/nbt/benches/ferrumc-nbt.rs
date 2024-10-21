#![feature(portable_simd)]

use crate::structs::{BlockState, Chunk, Palette};
use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use fastnbt::Value;
use ferrumc_macros::NBTDeserialize;
use nbt as hematite_nbt;
use std::io::Cursor;

mod structs {
    use super::*;
    #[derive(NBTDeserialize)]
    pub(super) struct Chunk<'a> {
        #[nbt(rename = "xPos")]
        pub(crate) x_pos: i32,
        #[nbt(rename = "zPos")]
        pub(crate) z_pos: i32,
        #[nbt(rename = "Heightmaps")]
        pub(crate) heightmaps: Heightmaps<'a>,
        sections: Vec<Section<'a>>,
    }

    #[derive(NBTDeserialize)]
    pub(super) struct Heightmaps<'a> {
        #[nbt(rename = "MOTION_BLOCKING")]
        pub(crate) motion_blocking: &'a [i64],
    }

    #[derive(NBTDeserialize)]
    pub(super) struct Section<'a> {
        #[nbt(rename = "Y")]
        y: i8,
        block_states: Option<BlockState<'a>>,
    }

    #[derive(NBTDeserialize)]
    pub(super) struct BlockState<'a> {
        pub(crate) data: Option<&'a [i64]>,
        pub(crate) palette: Vec<Palette<'a>>,
    }

    #[derive(NBTDeserialize)]
    pub(super) struct Palette<'a> {
        #[nbt(rename = "Name")]
        pub(crate) name: &'a str,
    }
}
fn bench_ferrumc_nbt(data: &[u8]) {
    let chunk = Chunk::from_bytes(data).unwrap();
    assert_eq!(chunk.x_pos, 0);
    assert_eq!(chunk.z_pos, 32);
    assert_eq!(chunk.heightmaps.motion_blocking.len(), 37);
}

fn bench_simdnbt(data: &[u8]) {
    let nbt = simdnbt::borrow::read(&mut Cursor::new(data)).unwrap();

    let nbt = nbt.unwrap();
    let nbt = nbt.as_compound();
    let x_pos = nbt.get("xPos").unwrap().int().unwrap();
    let z_pos = nbt.get("zPos").unwrap().int().unwrap();

    let motion_blocking = nbt
        .get("Heightmaps")
        .unwrap()
        .compound()
        .unwrap()
        .get("MOTION_BLOCKING")
        .unwrap()
        .long_array()
        .unwrap();

    let sections = nbt.get("sections").unwrap().list().unwrap();
    let sections = sections
        .compounds()
        .unwrap()
        .into_iter()
        .filter_map(|section| {
            let y = section.get("Y").unwrap().byte().unwrap();
            let block_states = section.get("block_states")?;
            let block_states = block_states.compound().unwrap();
            let data = block_states.get("data")?;
            let data = data.long_array().unwrap();
            let data = data.leak();
            let palette = block_states.get("palette").unwrap().list().unwrap();
            let palette = palette
                .compounds()
                .unwrap()
                .into_iter()
                .map(|palette| {
                    let name = palette.get("Name").unwrap().string().unwrap();
                    let str = name.to_str();
                    let name = str.as_ref();
                    let name = Box::leak(name.to_string().into_boxed_str());
                    Palette { name }
                })
                .collect();
            Some(BlockState {
                data: Some(data),
                palette,
            })
        })
        .collect::<Vec<_>>();

    assert_eq!(x_pos, 0);
    assert_eq!(z_pos, 32);
    assert_eq!(motion_blocking.len(), 37);
    assert!(!sections.is_empty());
}

fn bench_simdnbt_owned(data: &[u8]) {
    let nbt = simdnbt::owned::read(&mut Cursor::new(data)).unwrap();
    assert!(nbt.is_some());
}

fn ussr_nbt_borrow(data: &[u8]) {
    let nbt = black_box(ussr_nbt::borrow::Nbt::read(&mut Cursor::new(data)).unwrap());
    black_box(nbt);
}

fn ussr_nbt_owned(data: &[u8]) {
    let nbt = black_box(ussr_nbt::owned::Nbt::read(&mut Cursor::new(data)).unwrap());
    black_box(nbt);
}

fn fastnbt(data: &[u8]) {
    let nbt: Value = black_box(fastnbt::from_reader(&mut Cursor::new(data)).unwrap());
    black_box(nbt);
}

fn crab_nbt(data: &[u8]) {
    let nbt = crab_nbt::Nbt::read(&mut Cursor::new(data)).unwrap();
    black_box(nbt);
}

fn hematite_nbt(data: &[u8]) {
    let nbt = hematite_nbt::Blob::from_reader(&mut Cursor::new(data)).unwrap();
    black_box(nbt);
}

fn criterion_benchmark(c: &mut Criterion) {
    // let cursor = Cursor::new(include_bytes!("../../../../../.etc/benches/region/r.0.0.mca"));
    // let file = std::fs::File::open(r#"D:\Minecraft\framework\ferrumc\ferrumc-2_0\ferrumc\.etc\benches\region\r.0.0.mca"#).unwrap();

    let data = include_bytes!("../../../../../.etc/benches/chunk_0-0.nbt");

    let mut group = c.benchmark_group("Chunk Data NBT Parsing");
    group.throughput(Throughput::Bytes(data.len() as u64));
    group.bench_function("FerrumC NBT", |b| {
        b.iter(|| bench_ferrumc_nbt(black_box(data)))
    });
    group.bench_function("simdnbt borrow", |b| {
        b.iter(|| bench_simdnbt(black_box(data)))
    });
    group.bench_function("simdnbt owned", |b| {
        b.iter(|| bench_simdnbt_owned(black_box(data)))
    });
    group.bench_function("fastnbt", |b| b.iter(|| fastnbt(black_box(data))));
    group.bench_function("ussr_nbt owned", |b| {
        b.iter(|| ussr_nbt_owned(black_box(data)))
    });
    group.bench_function("ussr_nbt borrow", |b| {
        b.iter(|| ussr_nbt_borrow(black_box(data)))
    });
    group.bench_function("crab_nbt", |b| b.iter(|| crab_nbt(black_box(data))));
    group.bench_function("hematite_nbt", |b| b.iter(|| hematite_nbt(black_box(data))));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

#![cfg(test)]

mod r#impl;

use std::thread::sleep;
use std::time::Duration;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nbt_lib::nbt_spec::serializer::NBTSerialize;
use nbt_lib::Serialize;

fn benchmark_serialize_compound_custom(c: &mut Criterion) {
    #[derive(Serialize)]
    #[nbt(is_root)]
    struct ESS {
        id: u8
    }
    let extremely_small_struct = ESS {
        id: 0
    } ;

    c.bench_function("serialize player no blackbox", |b| b.iter(|| {
        let mut buffer = black_box(Vec::with_capacity(1024));
        black_box(extremely_small_struct.serialize(&mut buffer)).unwrap();
        black_box(buffer);
    }));
}


criterion_group!(
    benches,
    benchmark_serialize_compound_custom,
);
criterion_main!(benches);
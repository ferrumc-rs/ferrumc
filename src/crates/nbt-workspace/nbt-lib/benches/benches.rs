#![cfg(test)]

mod r#impl;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nbt_lib::nbt_spec::serializer::NBTSerialize;
use crate::r#impl::NBTTestStruct;

fn benchmark_serialize_compound_custom(c: &mut Criterion) {
    let test_struct = NBTTestStruct::new();

    c.bench_function("serialize player custom", |b| b.iter(|| {
        /*let mut buffer = Vec::new();
        black_box(named_tag.serialize(&mut buffer).unwrap());
        black_box(buffer);*/
        let mut buffer = Vec::new();
        test_struct.serialize(&mut buffer).unwrap();
        black_box(buffer);
    }));
}

criterion_group!(
    benches,
    benchmark_serialize_compound_custom,
);
criterion_main!(benches);
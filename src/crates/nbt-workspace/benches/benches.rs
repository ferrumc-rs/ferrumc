use std::io::Cursor;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use custom_nbt::test::NBTTestStruct;
use custom_nbt::nbt_spec::serializer::NBTSerialize;

fn benchmark_serialize_compound_custom(c: &mut Criterion) {
    let test_struct = NBTTestStruct::new();
    let named_tag = test_struct.to_nbt();

    c.bench_function("serialize player custom", |b| b.iter(|| {
        let mut buffer = Vec::new();
        black_box(named_tag.serialize(&mut buffer).unwrap());
        black_box(buffer);
    }));
}

fn benchmark_serialize_compound_other(c: &mut Criterion) {
    let test_struct = NBTTestStruct::new();

    c.bench_function("serialize player fastnbt", |b| b.iter(|| {
        let _bytes = black_box(fastnbt::to_bytes(&test_struct).unwrap());
    }));
}

criterion_group!(
    benches,
    benchmark_serialize_compound_custom,
    benchmark_serialize_compound_other
);
criterion_main!(benches);
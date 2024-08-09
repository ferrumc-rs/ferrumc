use criterion::{black_box, criterion_group, criterion_main, Criterion};
use custom_nbt::nbt_spec::serializer::NBTSerialize;
use custom_nbt::test::NBTTestStruct;
use serde::Serialize;


fn benchmark_serialize_compound_custom(c: &mut Criterion) {
    let test_struct = NBTTestStruct::new();
    let named_tag = test_struct.to_nbt();

    c.bench_function("serialize compound custom", |b| b.iter(|| {
        let mut buffer = Vec::new();
        black_box(named_tag.serialize(&mut buffer)).unwrap();
    }));
}

fn benchmark_serialize_compound_fastnbt(c: &mut Criterion) {
    let test_struct = NBTTestStruct::new();

    c.bench_function("serialize compound fastnbt", |b| b.iter(|| {
        let buffer = black_box(fastnbt::to_bytes(&test_struct)).unwrap();
    }));
}

criterion_group!(
    benches,
    benchmark_serialize_compound_custom,
    benchmark_serialize_compound_fastnbt
);
criterion_main!(benches);
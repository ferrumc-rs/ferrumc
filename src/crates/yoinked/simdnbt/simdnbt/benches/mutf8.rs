use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use simdnbt::Mutf8Str;

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("mutf8");

    group.bench_function("to_str", |b| {
        let input = black_box(Mutf8Str::from_slice(b"asgwjiebrtiowuubrtiowerthb8qwertyuwerpotihnqwiortuhbweinoqwner9opiquwehuiowrtjbwerioltubnwrioutunqweol;rkinqweuiorbqweruiqvbwefP;WOEJ  Q0OEPWRIYGYUIEDRYASZTFHGC Ijkbuiljfn qwilrtb qsokjladfnqiowugrtbquiowerbq   we;roiqwerghqwioerhd5rtea456etrsdyutrioutyopuipjklhkjfgghjdffghasdgxvncm,bn,.bnioug78yufvukyhfyutdyf"));
        b.iter(|| {
            black_box(input.to_str());
        })
    });
    group.bench_function("to_string long", |b| {
        let input = black_box(Mutf8Str::from_slice(b"asgwjiebrtiowuubrtiowerthb8qwertyuwerpotihnqwiortuhbweinoqwner9opiquwehuiowrtjbwerioltubnwrioutunqweol;rkinqweuiorbqweruiqvbwefP;WOEJ  Q0OEPWRIYGYUIEDRYASZTFHGC Ijkbuiljfn qwilrtb qsokjladfnqiowugrtbquiowerbq   we;roiqwerghqwioerhd5rtea456etrsdyutrioutyopuipjklhkjfgghjdffghasdgxvncm,bn,.bnioug78yufvukyhfyutdyf"));
        b.iter(|| {
            black_box(input.to_string());
        })
    });
    group.bench_function("to_string short", |b| {
        let input = black_box(Mutf8Str::from_slice(b"hello world"));
        b.iter(|| {
            black_box(input.to_string());
        })
    });
    group.bench_function("to_owned into_string", |b| {
        let input = black_box(Mutf8Str::from_slice(b"asgwjiebrtiowuubrtiowerthb8qwertyuwerpotihnqwiortuhbweinoqwner9opiquwehuiowrtjbwerioltubnwrioutunqweol;rkinqweuiorbqweruiqvbwefP;WOEJ  Q0OEPWRIYGYUIEDRYASZTFHGC Ijkbuiljfn qwilrtb qsokjladfnqiowugrtbquiowerbq   we;roiqwerghqwioerhd5rtea456etrsdyutrioutyopuipjklhkjfgghjdffghasdgxvncm,bn,.bnioug78yufvukyhfyutdyf"));
        b.iter(|| {
            black_box(input.to_owned().into_string());
        })
    });
    group.bench_function("to_owned", |b| {
        let input = black_box(Mutf8Str::from_slice(b"asgwjiebrtiowuubrtiowerthb8qwertyuwerpotihnqwiortuhbweinoqwner9opiquwehuiowrtjbwerioltubnwrioutunqweol;rkinqweuiorbqweruiqvbwefP;WOEJ  Q0OEPWRIYGYUIEDRYASZTFHGC Ijkbuiljfn qwilrtb qsokjladfnqiowugrtbquiowerbq   we;roiqwerghqwioerhd5rtea456etrsdyutrioutyopuipjklhkjfgghjdffghasdgxvncm,bn,.bnioug78yufvukyhfyutdyf"));
        b.iter(|| {
            black_box(input.to_owned());
        })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
